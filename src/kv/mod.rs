mod get;
mod put;

use std::fs::create_dir_all;
use crate::datafile::{DataFile,DataFileMetadata};
use super::keydir::KeyDir;

#[derive(Clone, Debug)]
pub struct Options {
    pub base_dir: std::path::PathBuf,
    pub data_file_limit: u64,
}

pub struct Database {
    options: Options,

    keydir: KeyDir,

    // active file:
    current_data_file: DataFile,

    // datafiles
    data_files: Vec<DataFileMetadata>,
    data_files_cache: LruCache<u128, DataFile>,

    // once the active DataFile has reached the threshold
    // defined in data_file_limit, it will open a new data_file:
    data_file_limit: u64,
}


pub fn new(options: Options) -> ErrorResult<Database> {
    // best effort:
    let _ = env_logger::try_init();

    create_dir_all(&options.base_dir).map_err(|source| Error::CreateDatabaseDir {
        path: options.base_dir.to_path_buf(),
        source,
    })?;

    let created_dir = create_dir_all(&options.base_dir);
    if let Err(err_msg) = created_dir {
        return Err(new_err(&format!(
            "Failed to create '{}': {}",
            options.base_dir.display(),
            err_msg
        )));
    }

    let path = std::path::Path::new(&options.base_dir);

    let filename = crate::config::data_file_format(crate::utils::time());
    let data_file = DataFile::create(&path.join(filename), false)?;

    let mut db = Database {
        options: options.clone(),
        keydir: KeyDir::new(),
        current_data_file: data_file,
        data_files: Vec::new(),
        data_files_cache: LruCache::new(128),
        data_file_limit: options.data_file_limit,
    };

    db.startup(&path)?;

    Ok(db)
}

pub struct Stats {
    pub num_immutable_datafiles: u64,
    pub num_keys: u64,
}