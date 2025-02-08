use crate::configuration::DBSettings;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

use crate::configuration::get_configuration;

pub async fn get_db_handle() -> Result<(Pool<Sqlite>, DBSettings), Box<dyn std::error::Error>> {
    let toml_settings = get_configuration().expect("msg");
    let database_settings = toml_settings.db;

    // create db_handle >>> there should be a better way to do this
    let db_handle = run(&database_settings).await?;
    Ok((db_handle, database_settings))
}

pub async fn get_connection_pool(dbconfig: &DBSettings) -> SqlitePool {
    SqlitePool::connect(&dbconfig.dbname).await.unwrap()
}

pub async fn run(database_settings: &DBSettings) -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(&database_settings.dbname)
        .await
        .unwrap_or(false)
    {
        println!("Creating database {}", database_settings.dbname);
        match Sqlite::create_database(&database_settings.dbname).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error : {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = get_connection_pool(database_settings).await;
    Ok(db)
}
