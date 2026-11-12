use clap::{Parser, builder::NonEmptyStringValueParser};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Update all flake inputs
    #[arg(short, long)]
    pub update: bool,

    /// Use `boot` instead of `switch`
    #[arg(short, long)]
    pub boot: bool,

    /// Specify an alternate host name to rebuild
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    pub hostname: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
