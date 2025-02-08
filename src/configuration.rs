use std::collections::HashMap;

use config::ConfigError;

#[derive(serde::Deserialize, Debug)]
pub struct ConfigSettings {
    pub app: AppSettings,
    pub results: ResultSettings,
    pub glossary: GlossarySettings,
    pub db: DBSettings,
    pub lang: LangSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct AppSettings {
    pub address: String,
    pub admin_username: String,
    pub admin_password: String,
    pub admin_assets: Vec<String>,
    pub root_url: String,
    pub enable_submissions: bool,
    pub enable_pages: bool,
    pub dicts: Vec<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ResultSettings {
    pub default_per_page: u8,
    pub max_per_page: u8,
    pub num_page_nums: u8,
}

#[derive(serde::Deserialize, Debug)]
pub struct GlossarySettings {
    pub enabled: bool,
    pub default_per_page: u8,
    pub max_per_page: u8,
    pub num_page_nums: u8,
}

#[derive(serde::Deserialize, Debug)]
pub struct LangSettings {
    pub name: Option<String>,
    pub tokenizer: Option<String>,
    pub tokenizer_type: Option<String>,
    pub types: Option<HashMap<String, String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct LangTypes {
    pub noun: String,
    pub adj: String,
    pub verb: String,
    pub adv: String,
    pub conj: String,
}

#[derive(serde::Deserialize)]
pub struct Consts {
    pub site: String,
    pub root_url: String,
    pub enable_submissions: bool,
    pub enable_glossary: bool,
    pub admin_username: Vec<u8>,
    pub admin_password: Vec<u8>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DBSettings {
    pub host: String,
    pub port: u32,
    pub dbname: String,
    pub user: String,
    pub password: String,
}

// impl DBSettings {
//     pub fn without_db(&self) -> PgConnectOptions {
//         SqliteConnectOptions::new()
//             .host(&self.host)
//             .username(&self.username)
//             .password(self.password.expose_secret())
//             .port(self.port)
//     }

//     pub fn with_db(&self) -> PgConnectOptions {
//         self.without_db().database(&self.database_name)
//     }
// }

impl Consts {
    fn new() -> Consts {
        Self {
            site: "".to_string(),
            root_url: "/".to_string(),
            enable_submissions: todo!(),
            enable_glossary: todo!(),
            admin_username: todo!(),
            admin_password: todo!(),
        }
    }
}

pub fn get_configuration() -> Result<ConfigSettings, ConfigError> {
    let directory = std::env::current_dir().expect("Failed to get cwd");
    let settings = config::Config::builder()
        .add_source(config::File::from(directory.join("config.toml")))
        .build()?;

    settings.try_deserialize::<ConfigSettings>()
}
