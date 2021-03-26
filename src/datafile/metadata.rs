

#[derive(Clone, Debug)]
pub struct DataFileMetadata {
    pub id: u128,
    pub path: std::path::PathBuf,
}

/// CleanFile is a wrapper for File which deletes the file on close
/// if the file's size is 0:
#[derive(Debug)]
pub struct CleanFile {
    file: Option<std::fs::File>,
    path: std::path::PathBuf,
}

impl std::ops::Deref for CleanFile {
    type Target = std::fs::File;

    fn deref(&self) -> &Self::Target {
        self.file.as_ref().unwrap()
    }
}

impl std::ops::DerefMut for CleanFile {
    fn deref_mut(&mut self) -> &mut std::fs::File {
        self.file.as_mut().unwrap()
    }
}

impl Drop for CleanFile {
    fn drop(&mut self) {
        self.file.take();

        let path = &self.path.as_path();
        let file_metadata = std::fs::metadata(path);
        if let Ok(metadata) = file_metadata {
            if metadata.len() == 0 {
                log::trace!(
                    "Datafile.drop: removing file since its empty {}",
                    path.display()
                );
                let _ = std::fs::remove_file(path);
            }
        }
    }
}
