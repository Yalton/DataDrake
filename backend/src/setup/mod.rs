mod state;

use clap::Parser;
use serde::Deserialize;
pub use state::State;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Exposed server port.
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

#[derive(Debug, Deserialize)]
pub struct Cfg {
    /// Webdriver connection string.
    pub browser: String,
}

impl Cfg {
    pub fn parse() -> Self {
        let file = std::fs::read_to_string("./config.toml").unwrap();
        toml::from_str(&file).unwrap()
    }
}
