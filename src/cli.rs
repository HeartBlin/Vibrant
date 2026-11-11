use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Update all flake inputs
    #[arg(short, long)]
    pub update: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
