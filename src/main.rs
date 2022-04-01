use std::{fs, process};

use clap::Parser;

use crate::config::Config;
use crate::database::get_database;
use crate::generation::execute;

mod config;
mod database;
mod generation;
mod sql;
mod str_util;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the table to add values to
    #[clap(short, long)]
    table: String,

    /// Number of records to create
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let config_file_name = "seed_config.json";
    let app_config_str: String;
    match fs::read_to_string(config_file_name) {
        Ok(s) => app_config_str = s,
        Err(e) => {
            println!("Failed to read {}: {}", config_file_name, e);
            process::exit(1)
        }
    }

    println!(
        "Will attempt to create {} records in {} table",
        args.count, args.table
    );

    let config: Config;
    match serde_json::from_str(app_config_str.as_str()) {
        Ok(c) => config = c,
        Err(e) => {
            println!(
                "Failed to parse database config file {}: {}",
                config_file_name, e
            );
            process::exit(1)
        }
    }

    let db = get_database(&config);

    let generators = config.create_generators();

    let generation_result = execute(db, generators, &args.table, args.count);
    match generation_result {
        Some(e) => println!("Error occurred during generation {}", e),
        _ => {
            println!(
                "Successfully created {} records in {} table",
                args.count, args.table
            );
        }
    }
}
