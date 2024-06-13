use std::path::PathBuf;

pub fn invoke(file: PathBuf) {
    if file.exists() {
        println!("file exists");
    } else {
        println!("file does not exist");
    }
}
