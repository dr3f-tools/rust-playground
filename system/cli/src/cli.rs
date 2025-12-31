use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Adds two integers")]
pub struct Cli {
    #[arg(short)]
    pub a: i32,

    #[arg(short)]
    pub b: i32,
}

pub fn parse_cli() -> super::Cli {
    super::Cli::parse()
}
