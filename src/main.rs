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
    
    //let name = args.rename.unwrap_or(String::new());
    let src = args.filename.to_str().unwrap(); 
    let dest = args.destination.to_str().unwrap(); 

    let mut dest_path: String;

    match args.rename {
        Some(name) => {
            dest_path = if dest.ends_with("/") {
                format!("{}{}", dest, name)
            } else {
                format!("{}/{}", dest, name)
            };
        },

        None => todo!()
    }

    println!("{src} {dest_path} ");
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
