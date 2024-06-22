use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

/// will take in list of files and add it to one config files
pub fn invoke(files: Vec<PathBuf>) {
    for file in files {
        if file.exists() {
            match append_files(file) {
                Ok(_) => println!("merging files complete"),
                Err(_) => println!("merging failed"),
            }
        } else {
            println!("File does not exist: {:?}", file);
        }
    }
}

pub fn append_files(file: PathBuf) -> std::io::Result<()> {
    // append this file to a common config file
    println!("Appending file: {:?}", file);
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("config.toml")?;
    // FIXME : Too many question marks ::: dont like :-(
    //write contents of file to config.toml
    let mut file_contents = fs::File::open(file)?;

    let mut buffer = Vec::new();

    file_contents.read_to_end(&mut buffer)?;

    config_file.write_all(&buffer)?;
    Ok(())
}
