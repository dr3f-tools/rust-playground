use clap::Parser;
use std::path::PathBuf;

/// A simple CLI tool description
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The pattern to search for
    pattern: String,

    /// The path to the file to read
    #[arg(short, long)] // Defines a short (-p) and long (--path) flag
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Pattern: {:?}", args.pattern);
    println!("Path: {:?}", args.path);
}
