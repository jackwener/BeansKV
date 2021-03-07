use std::collections::BTreeMap;

use crate::ErrorResult;

#[derive(Debug, Clone, Copy)]
pub struct KeyDirEntry {
    pub file_id: u128,
    pub offset: u64,
    pub timestamp: u128,
}

#[derive(Default)]
pub struct KeyDir {
    map: BTreeMap<Vec<u8>, KeyDirEntry>,
}

impl KeyDir {
    pub fn new() -> KeyDir {
        Self::default()
    }

    pub fn set(&mut self, key :&[u8], file_id: u128, offset: u64, timestamp: u128)  -> ErrorResult<()> {
        self.map.insert(Vec::from(key), KeyDirEntry{file_id,offset,timestamp});

        Ok(())
    }


    // TODO this should probably return a reference to the KeyDirEntry
    pub fn get(&self, key: &[u8]) -> ErrorResult<KeyDirEntry> {
        // TODO this can just be an ok_or_else
        if !self.entries.contains_key(key) {
            let key_str = format!("key not found: {}", std::str::from_utf8(key)?);
            return Err(string_error::new_err(key_str.as_str()));
        }
        let entry = self.entries.get(key).cloned().unwrap();
        Ok(entry)
    }

    // TODO this result is never made
    pub fn remove(&mut self, key: &[u8]) -> ErrorResult<()> {
        self.entries.remove(&key.to_vec());
        Ok(())
    }
}
