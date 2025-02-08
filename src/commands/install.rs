use std::{io::stdin, process::exit};

use crate::{
    commands::{init::get_db_handle, new_config::Asset},
    configuration::DBSettings,
};
use serde_json;
use sqlx::{migrate::MigrateDatabase, Executor, Pool, Sqlite};

pub async fn invoke(database_all: (Pool<Sqlite>, DBSettings)) {
    // check for prompt ,get that through function arguments
    let (db_handle, database_info) = (database_all.0, database_all.1);
    println!("** Running first time DB installation **");
    println!(
        "** IMPORTANT: This will overwrite tables and types in DB ---- {}**",
        database_info.dbname
    );

    let mut ok = "".to_string();
    println!("continue (y/n)?");
    stdin().read_line(&mut ok).unwrap();
    let input = ok.trim();

    if input == "y" {
        println!("Installing database");
    } else {
        println!("Exiting without installing");
        exit(1);
    }

    let schema_file =
        Asset::get("deps/queries/schema_sqlite.sql").expect("Failed to get schema.sql file");
    let schema_queries = std::str::from_utf8(schema_file.data.as_ref()).unwrap();

    // Pin<Box<dyn Future<Output = Result<Sqlitequery,Error>> + Send>>
    // execute the query
    let result = db_handle
        .execute(schema_queries)
        .await
        .expect("issue with exec");

    // only if the db exists.
    if Sqlite::database_exists(database_info.dbname.as_str())
        .await
        .is_ok()
    {
        // hard coding for now
        println!("DB found {} ", database_info.dbname);
        let ver = String::from("v2.0.0");
        match record_migration_version(ver, &db_handle).await {
            Ok(_) => println!("ok"),
            Err(_) => println!("eerr"),
        }
    } else {
        println!("invoke finished");
    }
}

async fn record_migration_version(ver: String, db: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let current_value = sqlx::query_scalar!("SELECT value FROM settings WHERE key = 'migrations'")
        .fetch_optional(db)
        .await?;

    let updated_value = match current_value {
        Some(val) => {
            let mut migrations: Vec<String> =
                serde_json::from_str(&val).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            migrations.push(ver);
            serde_json::to_string(&migrations).map_err(|e| sqlx::Error::Protocol(e.to_string()))?
        }
        None => {
            serde_json::to_string(&vec![ver]).map_err(|e| sqlx::Error::Protocol(e.to_string()))?
        }
    };

    sqlx::query!(
        "INSERT INTO settings (key, value) VALUES ('migrations', ?)
        ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        updated_value
    )
    .execute(db)
    .await?;

    Ok(())
}
