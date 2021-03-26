
use std::io::{Seek, SeekFrom};

pub struct DataFileIterator {
    file: std::fs::File,
}

impl Iterator for DataFileIterator {
    type Item = (u64, Entry);

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.file.seek(SeekFrom::Current(0)).unwrap();
        let decoded_maybe = bincode::deserialize_from(&self.file);
        Some((offset, decoded_maybe.ok()?))
    }
}

pub struct Entry {
    // TODO: crc: impl later
    pub timestamp: u128,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}