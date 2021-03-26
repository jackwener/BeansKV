use super::Database;

use crate::ErrorResult;

impl Database {
    pub fn put(&mut self, key: &[u8], value: &[u8]) -> ErrorResult<()> {
        let data_file_id = self.current_data_file.get_id();
        let timestamp = crate::utils::time();

        let offset = self.current_data_file.write(key,value,timestamp)?;
        self.keydir.set(&key, data_file_id, offset, timestamp)?;

        if offset >= self.data_file_limit {
            return self.switch_new_data_file();
        }

        Ok(())
    }
}