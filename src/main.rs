#![allow(unused)]

use std::path::PathBuf;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {

    #[arg(help = "File name or relative path to file to be copied")]
    filename: PathBuf,

    #[arg(help = "Destination path")]
    destination: PathBuf,

    #[arg(short, long, help = "Rename file")]
    rename: Option<String>,
}

fn main() {
    let args = Cli::parse();
    
    let source_path = args.filename.to_str().unwrap(); 
    let destination_path = args.destination.to_str().unwrap(); 
    let name = args.rename.unwrap_or(String::new());

    println!("{source_path} {destination_path} {name}");
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
