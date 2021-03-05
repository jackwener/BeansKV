use std::path::{Path,PathBuf};

use crate::keydir::{KeyDirEntry, KeyDir};
use crate::datafile::DataFile;

impl super::Database {

    pub fn keys(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.keydir.keys()
    }

    pub fn keys_range(
        &self,
        min: &[u8],
        max: &[u8],
    ) -> impl Iterator<Item = (&Vec<u8>, &KeyDirEntry)> {
        self.keydir.keys_range(min, max)
    }

    pub fn keys_range_min(&self, min: &[u8]) -> impl Iterator<Item = (&Vec<u8>, &KeyDirEntry)> {
        self.keydir.keys_range_min(min)
    }

    pub fn keys_range_max(&self, max: &[u8]) -> impl Iterator<Item = (&Vec<u8>, &KeyDirEntry)> {
        self.keydir.keys_range_max(max)
    }

    fn get_data_files_except_current(&self, base_dir: &Path) -> ErrorResult<Vec<PathBuf>> {
        let mut entries = self.glob_files(&base_dir, crate::config::DATA_FILE_GLOB_FORMAT)?;

        entries.sort_by(|a, b| natord::compare(a.to_str().unwrap(), b.to_str().unwrap()));

        // Remove current data file since the current data file is mutable:
        entries.retain(|x| {
            x.file_name().unwrap().to_str().unwrap()
                != self
                .current_data_file
                .path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        });

        Ok(entries)
    }

    // get_datafile_at should only be used for debugging:
    pub fn get_datafile_at(&mut self, index: u32) -> DataFile {
        let df = self.data_files.get_mut(index as usize).unwrap();
        DataFile::create(&df.path, true).unwrap()
    }

    pub fn get_current_datafile(&mut self) -> DataFile {
        let path = self.current_data_file.path.as_path();
        DataFile::create(&path, true).unwrap()
    }
}