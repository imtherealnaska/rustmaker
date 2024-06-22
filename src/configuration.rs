#[derive(serde::Deserialize)]
pub struct Consts {
    site: String,
    root_url: String,
    enable_submissions: bool,
    enable_glossary: bool,
    admin_username: Vec<u8>,
    admin_password: Vec<u8>,
}

impl Consts {
    fn new() -> Consts {
        Self {
            site: todo!(),
            root_url: todo!(),
            enable_submissions: todo!(),
            enable_glossary: todo!(),
            admin_username: todo!(),
            admin_password: todo!(),
        }
    }
}

pub fn get_configuration() -> Result<Consts, config::ConfigError> {
    let cur_dir = std::env::current_dir().expect("Could not find current directory");
    todo!()
}
