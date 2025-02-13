pub mod commands;
pub mod configuration;
pub mod indicphone;
pub mod startup;
pub mod telemetry;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version , about , long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Import a CSV file into the database. eg bin import /path/to/file.csv
    Import,
    ///upgrade database to the current version
    Upgrade {},
    /// current version of the build.
    Version,
    ///path to one or more config files (will be merged in order) (default [config.toml]) only 5 files for now.
    Config {
        #[clap(short, long, default_value = "config.toml")]
        #[arg(value_parser=clap::value_parser!(PathBuf) , num_args=1..6)]
        files: Vec<PathBuf>,
    },
    /// Run first time DB installation
    Install,
    /// Assume 'yes' to prompts during --install/upgrade
    Yes,
    /// Path to a site theme. If left empty, only HTTP APIs will be available
    Site,
    /// Generate a new sample config.toml file.
    NewConfig,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let db = commands::init::get_db_handle()
        .await
        .expect("Failed to get db handle");

    match args.command {
        Command::Import => {}
        Command::Config { files } => commands::config::invoke(files),
        Command::Install => {
            commands::install::invoke(db).await;
        }
        Command::Yes => {
            println!("this is from yes");
        }
        Command::Site => {
            println!("this is from site");
        }
        Command::NewConfig {} => {
            commands::new_config::invoke();
        }
        Command::Upgrade {} => commands::upgrade::upgrade(&db.0).await.expect("Failed"),
        Command::Version => todo!(),
    }
    Ok(())
}
