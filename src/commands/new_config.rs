use core::panic;
use rand::{self, distributions::Alphanumeric, thread_rng, Rng};
use rust_embed::Embed;
use std::fs;

#[derive(Embed)]
#[folder = "deps/"]
#[prefix = "deps/"]
pub struct Asset;

// this function should be called when the user runs the command `dictpress new-config`
pub fn invoke() {
    match exists() {
        Ok(_) => {
            panic!("config.toml file already exists. Delete it before creating a new one.");
        }
        Err(_) => generate_new_files().expect("Generating config.toml file failed"),
    }
}

fn generate_new_files() -> std::io::Result<()> {
    // Get the sample config file. if not unwrap.
    // TODO: Need to handle this with better error handling.
    // remove "dictpress_admin_password" and "dictpress_admin_username" from the new config file.
    // let to = std::str::from_utf8(&data_cloned).unwrap();
    // Oooh functionaaalllll
    println!("Creating a new config.toml file");
    let fake_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    // println!("to string is {}", fake_password);
    let sample_config = Asset::get("deps/config.sample.toml").unwrap();
    let new_config = sample_config.data.as_ref();

    let new_config =
        std::str::from_utf8(new_config).expect("issue with converting types from u8 to str");

    let replaced_file_contents = new_config.replace("dictpress_admin_password", &fake_password);

    fs::write("config.toml", replaced_file_contents)?;

    Ok(())
}

fn exists() -> std::io::Result<()> {
    let attr = fs::metadata("config.toml")?;
    if attr.is_file() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "config.toml not found",
        ))
    }
}
