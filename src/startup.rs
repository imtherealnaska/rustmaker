use sqlx::migrate::MigrateDatabase;
use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::SqlitePool;

use crate::configuration::DBSettings;

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
