use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Update all flake inputs
    #[arg(short, long)]
    pub update: bool,

    /// Use `boot` instead of `switch`
    #[arg(short, long)]
    pub boot: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
