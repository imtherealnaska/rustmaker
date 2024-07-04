use std::{io::stdin, process::exit};

use sqlx::{Executor, Pool, Sqlite};

use crate::{commands::new_config::Asset, configuration::get_configuration, startup};

pub async fn invoke() {
    // will need to move this , and have a new connection in every subcommand
    let toml_settings = get_configuration().expect("msg");
    let database_settings = toml_settings.db;

    // create db_handle >>> there should be a better way to do this
    let db_handle = startup::run(&database_settings).await.unwrap();

    // check for prompt ,get that through function arguments
    println!("** Running first time DB installation **");
    println!(
        "** IMPORTANT: This will overwrite tables and types in DB {}**",
        database_settings.dbname
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

    println!("hurray something happened ??? {:?}", result);
    // hard coding for now
    let ver = String::from("v2.0.0");
    match record_migration_version(ver, &db_handle).await {
        Ok(_) => println!("ok"),
        Err(_) => println!("eerr"),
    }
}

// remember to pass a ref and not consume pool , or bad things happen
async fn record_migration_version(ver: String, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO settings (key, value) VALUES ('migrations', json(?))
        ON CONFLICT (key) DO UPDATE SET value = json(value || json(?))
        "#,
        ver,
        ver
    )
    .execute(db)
    .await?;

    Ok(())
}
