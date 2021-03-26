use BeansKV;
use bytesize::ByteSize;

fn main() {
    let options = BeansKV::Options {
        base_dir: std::path::PathBuf::from("./db"),
        data_file_limit: ByteSize::mb(1).as_u64(),
    };

    let mut db = BeansKV::new(options).expect("open database");

    for i in 0..100 {
        let key = format!("key {}", i);
        let value = format!("value {}", i);
        db.put(key.as_bytes(), value.as_bytes()).unwrap();
    }



}