use core::panic;
use include_dir::{include_dir, Dir};
use rand::{self, random, thread_rng, Rng, RngCore};
use std::fs;

static PROJECT_DIR: Dir = include_dir!("./");

pub fn invoke(create: bool) {
    if exists().is_ok() {
        panic!("config.toml already exists. Remove it to create a new one");
    } else {
        println!("Creating a new config.toml file");
        match generate_new_files() {
            Ok(_) => println!("config.toml created successfully"),
            Err(e) => panic!("Error creating config.toml file: {}", e),
        }
    }
}

fn generate_new_files() -> std::io::Result<()> {
    // Get the sample config file. if not unwrap.
    // TODO: Need to handle this with better error handling.
    let sample_config = PROJECT_DIR.get_file("config.sample.toml").unwrap();

    let mut data = [0u8; 12];
    let mut data_cloned = data.clone();
    rand::thread_rng().fill_bytes(&mut data);

    for (i, v) in data.iter().enumerate() {
        let mut rng = thread_rng();
        let random_number: u8 = rng.gen_range(0..=4);
        if random_number == 1 {
            data_cloned[i].make_ascii_uppercase();
        }
    }

    //remove "dictpress_admin_password" and "dictpress_admin_username" from the new config file.
    let to = std::str::from_utf8(&data_cloned).unwrap();
    let new_config = fs::read_to_string("config.sample.toml")?;
    let new_config = new_config.replace("dictpress_admin_password", to);

    fs::write("config.toml", new_config)?;

    Ok(())
}

fn exists() -> std::io::Result<()> {
    let attr = fs::metadata("config.toml")?;
    Ok(())
}
