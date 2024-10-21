use clap::Parser;
mod commands;
mod utils;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;

#[derive(Parser)]
struct Cli {
    command: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if args.command == "add" {
        let _ = commands::add::add().await;
    }else if args.command == "learn" {
        let utc: DateTime<Utc> = Utc::now(); 
        let timestamp_str = utc.format("%Y%m%d%H%M%S").to_string();
        let output = commands::learn::learn().await.expect("Failed to execute learn command");
        let file_name = format!("assignments_{}.txt", timestamp_str);
        let mut file = File::create(&file_name)?;
        file.write_all(output.as_bytes())?;
    }
    else {
        println!("{} is not a valid command", args.command);
    }

    Ok(())
}
