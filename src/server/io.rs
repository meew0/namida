use std::{
    borrow::Cow,
    fs::DirEntry,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::{
    datagram::{self, BlockType},
    types::{BlockIndex, FileMetadata, FileSize},
};

use super::Session;

/// Tries to read the given block from the currently open file. If successful, a datagram view is
/// created based on the data read. The caller must supply a buffer exactly big enough to fit one
/// block.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if no file is present, or if the seek position overflows.
pub fn build_datagram<'a>(
    session: &mut Session,
    block_index: BlockIndex,
    block_type: BlockType,
    block_buffer: &'a mut [u8],
) -> anyhow::Result<datagram::View<'a>> {
    assert_eq!(block_buffer.len(), crate::common::BLOCK_SIZE as usize);

    // move the file pointer to the appropriate location
    let file = session
        .transfer
        .file
        .as_mut()
        .expect("a file should be present");
    file.seek(SeekFrom::Start(
        u64::from(crate::common::BLOCK_SIZE)
            .checked_mul(u64::from((block_index.safe_sub(BlockIndex(1))).0))
            .expect("file position overflow"),
    ))?;

    // try to read in the block
    let read_amount = file.read(block_buffer)?;
    if read_amount < crate::common::BLOCK_SIZE as usize
        && block_index < session.properties.block_count
    {
        println!(
            "WARNING: only read {} instead of {} bytes for block {} out of {}",
            read_amount,
            crate::common::BLOCK_SIZE,
            block_index.0,
            session.properties.block_count.0
        );
    }

    // build the datagram & return success
    Ok(datagram::View {
        header: datagram::Header {
            block_index,
            block_type,
        },
        block: block_buffer,
    })
}

/// Recursively index files and subdirectories, starting with the given initial list of
/// files/directories. The resulting file metadata objects will be stored in the given `Vec`.
pub fn index_files(paths: &[PathBuf], files: &mut Vec<FileMetadata>) {
    index_files_internal(paths.iter().map(Cow::Borrowed), files);
}

fn index_files_internal<'a>(
    paths: impl Iterator<Item = Cow<'a, PathBuf>>,
    files: &mut Vec<FileMetadata>,
) {
    for path in paths {
        match std::fs::metadata(path.as_path()) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    // We found a directory — try to recursively index files and subdirectories
                    // within this directory
                    match std::fs::read_dir(path.as_path()) {
                        Ok(read_dir) => {
                            // We need to use `zip` and a separate function, instead of a closure,
                            // because of type recursion limits
                            // (see https://stackoverflow.com/q/54613966)
                            let paths = read_dir
                                .zip(std::iter::repeat(path))
                                .filter_map(entry_filter_map_func);
                            index_files_internal(paths, files);
                        }
                        Err(err) => {
                            eprintln!(
                                "Could not index directory: '{}', error: {err}",
                                path.display(),
                            );
                        }
                    }
                } else if metadata.is_file() {
                    // We found a file, append its path and size
                    files.push(FileMetadata {
                        path: path.into_owned(),
                        size: FileSize(metadata.len()),
                    });
                }
            }
            Err(err) => {
                eprintln!(
                    "Could not get metadata of file to be indexed: '{}', error: {err}",
                    path.display(),
                );
            }
        }
    }
}

fn entry_filter_map_func(
    tuple: (std::io::Result<DirEntry>, Cow<'_, PathBuf>),
) -> Option<Cow<'static, PathBuf>> {
    let (maybe_entry, path) = tuple;
    match maybe_entry {
        Ok(entry) => Some(Cow::Owned(entry.path())),
        Err(err) => {
            eprintln!(
                "Could not get entry within directory: '{}', error: {err}",
                path.display()
            );
            None
        }
    }
}
