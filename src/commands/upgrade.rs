use anyhow::{Context, Result};
use semver::Version;
use sqlx::{Row, SqlitePool};
use std::{
    future::Future,
    io::{self, Write},
    pin::Pin,
};

type MigrationFn =
    for<'a> fn(&'a SqlitePool) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

#[derive(Debug)]
struct Migration {
    version: &'static str,
    func: MigrationFn,
}

pub struct Config;

const MIGRATIONS: &[Migration] = &[Migration {
    version: "2.0.0",
    func: |db| Box::pin(v2_0_0(db)),
}];

pub async fn upgrade(db: &SqlitePool) -> Result<()> {
    print!("** IMPORTANT: Take a backup of the database before upgrading.\ncontinue (y/n)? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "y" {
        println!("upgrade cancelled");
        return Ok(());
    }

    let (_, to_run) = get_pending_migrations(db).await?;
    println!("to_run {:?}", to_run);
    if to_run.is_empty() {
        println!("no upgrades to run. Database is up to date.");
        return Ok(());
    }

    for migration in to_run {
        println!("running migration {}", migration.version);
        (migration.func)(db)
            .await
            .with_context(|| format!("error running migration {}", migration.version))?;

        if let Err(e) = record_migration_version(migration.version, db).await {
            match e.downcast_ref::<sqlx::Error>() {
                Some(sqlx::Error::Database(dbe)) if dbe.message().contains("no such table") => {
                    continue;
                }
                _ => {
                    return Err(
                        e.context(format!("error recording migration {}", migration.version))
                    )
                }
            }
        }
    }

    println!("upgrade complete");
    Ok(())
}

pub async fn check_upgrade(db: &SqlitePool) -> Result<()> {
    let (last_ver, to_run) = get_pending_migrations(db).await?;

    if to_run.is_empty() {
        return Ok(());
    }

    let versions: Vec<_> = to_run.iter().map(|m| m.version).collect();
    println!("there are {} pending database upgrade(s): {:?}. The last upgrade was {}. Backup the database and run --upgrade",
        to_run.len(), versions, last_ver);

    Ok(())
}

async fn get_pending_migrations(db: &SqlitePool) -> Result<(String, Vec<&'static Migration>)> {
    let last_ver = get_last_migration_version(db).await?;
    let clean_ver = last_ver.trim_matches(|c| c == '[' || c == ']' || c == '"' || c == 'v');
    println!("last version is {}", clean_ver);
    let to_run = MIGRATIONS
        .iter()
        .skip_while(|m| {
            println!("m.version is {}", m.version);
            let m_version = Version::parse(m.version).unwrap();
            let last_version = Version::parse(clean_ver).unwrap();
            m_version <= last_version
        })
        .collect();

    Ok((last_ver, to_run))
}

async fn get_last_migration_version(db: &SqlitePool) -> Result<String> {
    let result = sqlx::query(
        "SELECT COALESCE((SELECT value FROM settings WHERE key='migrations'), '0.0.0')",
    )
    .fetch_one(db)
    .await;

    match result {
        Ok(row) => Ok(row.get(0)),
        Err(e) => match e {
            sqlx::Error::Database(dbe) if dbe.message().contains("no such table") => {
                Ok("0.0.0".to_string())
            }
            other => Err(other.into()),
        },
    }
}

async fn record_migration_version(version: &str, db: &SqlitePool) -> Result<()> {
    sqlx::query("INSERT INTO settings (key, value) VALUES ('migrations', ?)")
        .bind(version)
        .execute(db)
        .await?;
    Ok(())
}

async fn v2_0_0(_db: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS settings(
        key TEXT NOT NULL UNIQUE,
        value TEXT NOT NULL DEFAULT '{}' ,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
    CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key);
    "#,
    )
    .execute(_db)
    .await?;

    sqlx::query("ALTER TABLE entries ADD COLUMN IF NOT EXISTS meta TEXT NOT NULL DEFAULT '{}'")
        .execute(_db)
        .await?;

    Ok(())
}
