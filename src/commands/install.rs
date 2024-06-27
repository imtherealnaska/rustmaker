use std::{io::stdin, process::exit};

use sqlx::{Executor, Pool};

use crate::{commands::new_config::Asset, configuration::DBSettings};

pub async fn invoke(db_settings: DBSettings, db_handle: Pool<sqlx::Sqlite>) {
    // check for prompt ,get that through function arguments
    println!("** Running first time DB installation **");
    println!(
        "** IMPORTANT: This will overwrite tables and types in DB {}**",
        db_settings.dbname
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

    println!("schema query is this {}", schema_queries);
    // Pin<Box<dyn Future<Output = Result<Sqlitequery,Error>> + Send>>
    // execute the query
    let result = db_handle
        .execute(schema_queries)
        .await
        .expect("issue with exec");

    println!("hurray something happened ??? {:?}", result);
}
