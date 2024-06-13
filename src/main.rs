pub(crate) mod commands;
pub(crate) mod indicphone;

use clap::Parser;
use clap::Subcommand;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version , about , long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Import a CSV file into the database. eg --import /path/to/file.csv
    Import,

    ///upgrade database to the current version
    Upgrade,

    /// current version of the build.
    Version,

    ///path to one or more config files (will be merged in order) (default [config.toml])
    Config {
        #[clap(short, long, default_value = "config.toml")]
        file: PathBuf,
    },

    /// Run first time DB installation
    Install,

    /// Assume 'yes' to prompts during --install/upgrade
    Yes,

    /// Path to a site theme. If left empty, only HTTP APIs will be available
    Site,

    /// Generate a new sample config.toml file.
    NewConfig {
        #[clap(short, long, default_value = "true")]
        create: bool,
    },
}

struct Lang {
    name: String,
    types: HashMap<String, String>,
    tokenizer_name: String,
    tokenizer_type: String,
    //    Tokenizer: data.Tokenizer,
}

struct Consts {
    site: String,
    root_url: String,
    enable_submissions: bool,
    enable_glossary: bool,
    admin_username: Vec<u8>,
    admin_password: Vec<u8>,
}

// need to set up log
// need to use the config vars .

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Import => {}
        Command::Config { file } => commands::config::invoke(file),
        Command::Install => {
            println!("this is from install");
        }
        Command::Yes => {
            println!("this is from yes");
        }
        Command::Site => {
            println!("this is from site");
        }
        Command::NewConfig { create } => {
            commands::new_config::invoke(create);
        }
        Command::Upgrade => todo!(),
        Command::Version => todo!(),
    }
    Ok(())
}
