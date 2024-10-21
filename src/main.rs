use clap::Parser;
mod commands;
mod utils;
use std::error::Error;
use std::fs::File;
use std::io::Write;

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
        let output = commands::learn::learn().await.expect("Failed to execute learn command");
        let mut file = File::create("output.txt")?;
        file.write_all(output.as_bytes())?;
    }
    else {
        println!("{} is not a valid command", args.command);
    }

    Ok(())
}
