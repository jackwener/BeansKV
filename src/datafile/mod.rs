mod iterator;
mod metadata;

pub use metadata::DataFileMetadata;

use metadata::CleanFile;

use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;

use serde::{Deserialize, Serialize};
use iterator::{DataFileIterator, Entry};
use crate::ErrorResult;


#[derive(Debug)]
pub struct DataFile {
    pub id: u128,
    pub is_readonly: bool,

    file: CleanFile,
    pub path: std::path::PathBuf,
}

impl DataFile {
    pub fn create(path: &std::path::Path, is_readonly: bool) -> ErrorResult<DataFile> {
        let datafile = if is_readonly {
            OpenOptions::new().read(true).open(&path)?
        } else {
            OpenOptions::new().read(true).append(true).write(true).create(true).open(&path)?
        };


        let id = crate::utils::extract_id_from_filename(&path.to_path_buf())?;

        let df = DataFile {
            id,
            file: CleanFile {
                file: Some(datafile),
                path: path.to_path_buf(),
            },
            is_readonly,
            path: path.to_path_buf(),
        };

        Ok(df)
    }

    pub fn get_id(&self) -> u128 {
        self.id as u128
    }

    pub fn write(&mut self, key: &[u8], value: &[u8], timestamp: u128) -> ErrorResult<u64> {
        let entry = Entry {
            timestamp,
            key: key.to_vec(),
            value: value.to_vec(),
        };
        use std::io::Write as _;

        let offset = self.file.seek(SeekFrom::Current(0))?;
        // serialize_into is vastly slower than serializing to avec then doing 1 big write
        let encoded: Vec<u8> = bincode::serialize(&entry)?;
        self.file.write_all(&encoded)?;
        Ok(offset)
    }

    pub fn remove(&mut self, key: &[u8], timestamp: u128) -> ErrorResult<u64> {
        self.write(key, crate::config::REMOVE_TOMBSTONE, timestamp)
    }

    pub fn read(&mut self, offset: u64) -> ErrorResult<Entry> {
        let mmap = unsafe { memmap::MmapOptions::new().map(&self.file)? };
        let decoded: Entry = bincode::deserialize(&mmap[(offset as usize)..])?;
        Ok(decoded)
    }

    pub fn iter(&mut self) -> DataFileIterator {
        let file = std::fs::File::open(&self.path).unwrap();

        DataFileIterator { file }
    }

    pub fn sync(&mut self) -> ErrorResult<()> {
        self.file.sync_all().map_err(Into::into)
    }

    // pub fn inspect(&mut self, with_header: bool) -> String {
    //     let mut list = String::new();
    //
    //     if with_header {
    //         list.push_str(format!("Datafile {}:\n", self.id).as_str());
    //     }
    //
    //     for (offset, entry) in self.iter() {
    //         let mut op = "S"; // Set
    //
    //         if entry.value == crate::config::REMOVE_TOMBSTONE {
    //             op = "D" // Delete
    //         }
    //
    //         let line = format!(
    //             "{:0>8} | {: >1} | {} | {}\n",
    //             offset,
    //             op,
    //             std::str::from_utf8(&entry.key).unwrap(),
    //             std::str::from_utf8(&entry.value).unwrap()
    //         );
    //         list.push_str(&line);
    //     }
    //
    //     list.trim_end().to_string()
    // }
}
