use super::Database;

impl Database {
    pub fn put(&mut self, key: &[u8], value: &[u8]) -> ErrorResult<()> {
        let cur_file_id = self.current_data_file.get_id();
        let timestamp = crate::util::now_time();

        let offset = self.current_data_file.write();
        self.keydir.set();

        if offset >= self.data_file_limit {

            return self.switch_new_data_file();
        }

        Ok(())
    }
}