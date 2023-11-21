use std::sync::{Condvar, Mutex};

use crate::datagram;

#[derive(Debug)]
pub struct RingBuffer {
    mutex: Mutex<Internal>,
    data_ready_cond: Condvar,
    space_ready_cond: Condvar,
}

#[derive(Debug)]
struct Internal {
    headers: Box<[datagram::Header; 4096]>,
    blocks: Box<[u8]>,
    block_size: u32,
    base_data: u32,
    count_data: u32,
    count_reserved: u32,
    data_ready: bool,
    space_ready: bool,
}

impl RingBuffer {
    pub fn count(&self) -> u32 {
        (*self.mutex.lock().unwrap()).count_data
    }

    pub fn is_full(&self) -> bool {
        !(*self.mutex.lock().unwrap()).space_ready
    }

    pub fn cancel(&mut self) {
        let mut guard = self.mutex.lock().unwrap();
        guard.count_reserved = guard
            .count_reserved
            .checked_sub(1)
            .expect("Attempt made to cancel unreserved slot in ring buffer");
        guard.space_ready = true;
        self.space_ready_cond.notify_all();
    }

    pub fn confirm(&self) {
        let mut guard = self.mutex.lock().unwrap();
        guard.count_data = guard
            .count_data
            .checked_add(1)
            .expect("Ring buffer overflow");
        guard.count_reserved = guard
            .count_reserved
            .checked_sub(1)
            .expect("Attempt made to confirm unreserved slot in ring buffer");
        guard.data_ready = true;
        self.data_ready_cond.notify_all();
    }

    pub fn create(block_size: u32) -> Self {
        let blocks_len = block_size as usize * 4096;

        let zero_header = datagram::Header {
            block_index: 0,
            block_type: 0,
        };

        let internal = Internal {
            headers: Box::new([zero_header; 4096]),
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

    pub fn peek<T>(&self, callback: impl FnOnce(datagram::View) -> T) -> T {
        let mut guard = self.mutex.lock().unwrap();
        while !guard.data_ready {
            guard = self.data_ready_cond.wait(guard).unwrap();
        }

        let first_index = (guard.block_size * guard.base_data) as usize;
        let last_index = (guard.block_size * (guard.base_data + 1)) as usize;

        callback(datagram::View {
            header: guard.headers[guard.base_data as usize],
            block: &guard.blocks[first_index..last_index],
        })
    }

    pub fn pop(&self) {
        let mut guard = self.mutex.lock().unwrap();
        while !guard.data_ready {
            guard = self.data_ready_cond.wait(guard).unwrap();
        }

        guard.base_data = (guard.base_data + 1) % 4096;
        guard.count_data = guard.count_data.checked_sub(1).unwrap();
        if guard.count_data == 0 {
            guard.data_ready = false;
        }

        guard.space_ready = true;
        self.space_ready_cond.notify_all();
    }

    pub fn reserve(&mut self, datagram: datagram::View) {
        self.reserve_internal(|header, block| {
            *header = datagram.header;
            block.copy_from_slice(datagram.block);
        });
    }

    pub fn reserve_zero(&mut self) {
        self.reserve_internal(|header, _block| {
            header.block_index = 0;
        });
    }

    fn reserve_internal(&mut self, mut callback: impl FnOnce(&mut datagram::Header, &mut [u8])) {
        let mut guard = self.mutex.lock().unwrap();

        let next = (guard.base_data + guard.count_data + guard.count_reserved) % 4096;
        while !guard.space_ready {
            println!("FULL! -- reserve() blocking.");
            println!(
                "space_ready = {}, data_ready = {}",
                guard.space_ready, guard.data_ready
            );
            guard = self.space_ready_cond.wait(guard).unwrap();
        }

        guard.count_reserved += 1;
        if guard.count_reserved > 1 {
            panic!("Attempt made to reserve two slots in ring buffer");
        }

        if (next + 1) % 4096 == guard.base_data {
            guard.space_ready = false;
        }

        let first_index = (guard.block_size * next) as usize;
        let last_index = (guard.block_size * (next + 1)) as usize;
        let internal = &mut *guard;
        callback(
            &mut internal.headers[next as usize],
            &mut internal.blocks[first_index..last_index],
        );
    }
}

pub fn allocate_zeroed_boxed_slice(len: usize) -> Box<[u8]> {
    let mut vec = Vec::with_capacity(len);
    vec.resize(len, 0);
    vec.into_boxed_slice()
}
