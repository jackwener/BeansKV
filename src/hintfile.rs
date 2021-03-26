use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::ErrorResult;

pub struct HintFile {
    pub id: u128,
    pub is_readonly: bool,

    file: std::fs::File,
    pub path: std::path::PathBuf,
}

impl HintFile {
    pub fn create(path: &std::path::Path, is_readonly: bool) -> ErrorResult<IndexFile> {}

    pub fn write(
        &mut self,
        key: &[u8],
        file_id: u128,
        offset: u64,
        timestamp: u128,
    ) -> ErrorResult<u64> {
        let entry = IndexEntry {
            key: key.to_vec(),
            file_id,
            offset,
            timestamp,
        };
    }

    pub fn read(&mut self, offset: u64) -> ErrorResult<IndexEntry> {
        self.file.seek(SeekFrom::Start(offset))?;

        let decoded: IndexEntry = bincode::deserialize_from(&self.file)?;

        Ok(decoded)
    }
}