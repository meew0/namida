use std::sync::{Condvar, Mutex};

use crate::{
    datagram,
    types::{BlockIndex, BlockSize},
};

pub const MAX_BLOCKS_QUEUED: u32 = 4096;

#[derive(Debug)]
pub struct Buffer {
    mutex: Mutex<Internal>,
    data_ready_cond: Condvar,
    space_ready_cond: Condvar,
}

#[derive(Debug)]
struct Internal {
    headers: Box<[datagram::Header; MAX_BLOCKS_QUEUED as usize]>,
    blocks: Box<[u8]>,
    block_size: BlockSize,
    base_data: u32,
    count_data: u32,
    count_reserved: u32,
    data_ready: bool,
    space_ready: bool,
}

impl Buffer {
    /// Returns the number of elements currently stored in the ring.
    ///
    /// # Panics
    /// Panics if the ring mutex is poisoned.
    pub fn count(&self) -> u32 {
        self.mutex
            .lock()
            .expect("mutex should not be poisoned")
            .count_data
    }

    /// Returns true if ring is full.
    ///
    /// # Panics
    /// Panics if the ring mutex is poisoned.
    pub fn is_full(&self) -> bool {
        !self
            .mutex
            .lock()
            .expect("mutex should not be poisoned")
            .space_ready
    }

    /// Cancels the reservation for the slot that was most recently reserved.
    ///
    /// # Panics
    /// Panics if the ring mutex is poisoned, or when there is no reserved slot.
    pub fn cancel(&self) {
        // get a lock on the ring buffer
        let mut guard = self.mutex.lock().expect("mutex should not be poisoned");

        // convert the reserved slot into space
        guard.count_reserved = guard
            .count_reserved
            .checked_sub(1)
            .expect("Attempt made to cancel unreserved slot in ring buffer");

        // signal that space is available
        guard.space_ready = true;
        drop(guard);
        self.space_ready_cond.notify_all();
    }

    /// Confirms that data is now available in the slot that was most recently reserved. This data
    /// will be handled by the disk thread.
    ///
    /// # Panics
    /// Panics if the ring mutex is poisoned, when there is no reserved slot, or when the buffer
    /// count overflows.
    pub fn confirm(&self) {
        // get a lock on the ring buffer
        let mut guard = self.mutex.lock().expect("mutex should not be poisoned");

        // convert the reserved slot into data
        guard.count_data = guard
            .count_data
            .checked_add(1)
            .expect("Ring buffer overflow");
        guard.count_reserved = guard
            .count_reserved
            .checked_sub(1)
            .expect("Attempt made to confirm unreserved slot in ring buffer");

        // signal that data is available
        guard.data_ready = true;
        drop(guard);
        self.data_ready_cond.notify_all();
    }

    /// Creates the ring buffer data structure for a transfer and returns the new data structure.
    /// The new ring buffer will hold `MAX_BLOCKS_QUEUED` datagrams with the given block size.
    ///
    /// # Panics
    /// Panics if there is an overflow in the amount of data.
    #[must_use]
    pub fn create(block_size: BlockSize) -> Self {
        let blocks_len = (block_size.0 as usize)
            .checked_mul(MAX_BLOCKS_QUEUED as usize)
            .expect("ring buffer size overflow");

        let zero_header = datagram::Header {
            block_index: BlockIndex(0),
            block_type: datagram::BlockType::Original,
        };

        let internal = Internal {
            headers: Box::new([zero_header; MAX_BLOCKS_QUEUED as usize]),
            blocks: allocate_zeroed_boxed_slice(blocks_len),
            block_size,
            base_data: 0,
            count_data: 0,
            count_reserved: 0,
            data_ready: false,
            space_ready: true,
        };

        Self {
            mutex: Mutex::new(internal),
            data_ready_cond: Condvar::new(),
            space_ready_cond: Condvar::new(),
        }
    }

    /// Calls the given callback with a view of the datagram at the head of the ring. This will
    /// block if the ring is currently empty.
    ///
    /// # Panics
    /// Panics if the ring mutex is poisoned, or becomes poisoned while waiting. Will also panic
    /// if an arithmetic overflow occurs.
    pub fn peek<T, F: FnOnce(datagram::View) -> T>(&self, callback: F) -> T {
        // get a lock on the ring buffer
        let mut guard = self.mutex.lock().expect("mutex should not be poisoned");

        // wait for the data-ready variable to make us happy
        while !guard.data_ready {
            guard = self
                .data_ready_cond
                .wait(guard)
                .expect("mutex should not be poisoned");
        }

        // find the slice we want
        let first_index = guard
            .block_size
            .0
            .checked_mul(guard.base_data)
            .expect("first_index overflow") as usize;
        let last_index = guard
            .block_size
            .0
            .checked_mul(guard.base_data.checked_add(1).expect("base_data overflow"))
            .expect("last_index overflow") as usize;

        // call the callback with the datagram
        callback(datagram::View {
            header: guard.headers[guard.base_data as usize],
            block: &guard.blocks[first_index..last_index],
        })
    }

    /// Attempts to remove a datagram from the head of the ring. This will block if the ring is
    /// currently empty.
    ///
    /// # Panics
    /// Panics if the ring mutex is poisoned, or becomes poisoned while waiting. Will also panic on
    /// if an arithmetic overflow occurs.
    pub fn pop(&self) {
        let mut guard = self.mutex.lock().expect("mutex should not be poisoned");
        while !guard.data_ready {
            guard = self
                .data_ready_cond
                .wait(guard)
                .expect("mutex should not be poisoned");
        }

        guard.base_data =
            (guard.base_data.checked_add(1).expect("base_data overflow")) % MAX_BLOCKS_QUEUED;
        guard.count_data = guard
            .count_data
            .checked_sub(1)
            .expect("count_data underflow");
        if guard.count_data == 0 {
            guard.data_ready = false;
        }

        guard.space_ready = true;
        drop(guard);
        self.space_ready_cond.notify_all();
    }

    /// Reserves a slot in the ring buffer, and stores the given datagram in it. This will block if
    /// no space is available in the ring buffer.
    ///
    /// # Panics
    /// Panics if there is an attempt to reserve two slots at once, if the mutex is poisoned, or
    /// if there is an arithmetic overflow.
    pub fn reserve(&self, datagram: datagram::View) {
        self.reserve_internal(|header, block| {
            *header = datagram.header;
            block.copy_from_slice(datagram.block);
        });
    }

    /// Reserves a slot in the ring buffer, and zeroes it. This will block if no space is available
    /// in the ring buffer.
    ///
    /// # Panics
    /// Panics if there is an attempt to reserve two slots at once, if the mutex is poisoned, or
    /// if there is an arithmetic overflow.
    pub fn reserve_zero(&self) {
        self.reserve_internal(|header, _block| {
            header.block_index = BlockIndex(0);
        });
    }

    // The significant_drop_tightening is a false positive; clippy does not recognise that the
    // borrow of the guarded data survives until after the callback returns
    #[allow(clippy::significant_drop_tightening)]
    fn reserve_internal(&self, callback: impl FnOnce(&mut datagram::Header, &mut [u8])) {
        // get a lock on the ring buffer
        let mut guard = self.mutex.lock().expect("mutex should not be poisoned");

        // figure out which slot comes next
        let next = guard
            .base_data
            .checked_add(guard.count_data)
            .and_then(|n| n.checked_add(guard.count_reserved))
            .expect("next overflow")
            % MAX_BLOCKS_QUEUED;

        // wait for the space-ready variable to make us happy
        while !guard.space_ready {
            println!("FULL! -- reserve() blocking.");
            println!(
                "space_ready = {}, data_ready = {}",
                guard.space_ready, guard.data_ready
            );
            guard = self
                .space_ready_cond
                .wait(guard)
                .expect("mutex should not be poisoned");
        }

        // perform the reservation
        assert_eq!(
            guard.count_reserved, 0,
            "Attempt made to reserve two slots in ring buffer"
        );
        guard.count_reserved = 1;
        let after_next = next.checked_add(1).expect("next overflow");
        if after_next % MAX_BLOCKS_QUEUED == guard.base_data {
            guard.space_ready = false;
        }

        // find the slice we want
        let first_index = guard
            .block_size
            .0
            .checked_mul(next)
            .expect("first_index overflow") as usize;
        let last_index = guard
            .block_size
            .0
            .checked_mul(after_next)
            .expect("last_index overflow") as usize;
        let internal = &mut *guard;

        // call the callback with the slice
        callback(
            &mut internal.headers[next as usize],
            &mut internal.blocks[first_index..last_index],
        );
    }
}

#[must_use]
pub fn allocate_zeroed_boxed_slice(len: usize) -> Box<[u8]> {
    vec![0; len].into_boxed_slice()
}
