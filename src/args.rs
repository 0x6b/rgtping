use std::net::SocketAddr;

use clap::Parser;

/// Command line arguments
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Array of IP address and port number (IP:port) to ping, delimited by a space
    #[arg(value_delimiter = ' ')]
    pub target_ips: Vec<SocketAddr>,
    /// Number of pings to send
    #[arg(short, long, default_value = "5")]
    pub count: u64,
    /// Interval between pings in milliseconds
    #[arg(short, long, default_value = "1000")]
    pub interval_ms: u64,
    /// Time to wait for a response, in milliseconds. 0 means wait indefinitely.
    #[arg(short = 'W', long, default_value = "10000")]
    pub timeout_ms: u64,
    /// Output format, either "json" or "text"
    #[arg(short, long, default_value = "json")]
    pub format: Format,
}

/// Output format
#[derive(Debug, Clone)]
pub enum Format {
    Json,
    Text,
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str().chars().next().unwrap() {
            'j' => Format::Json,
            't' => Format::Text,
            _ => Format::Text,
        }
    }
}

impl Args {
    /// Parse command line arguments and return the result. It's just a wrapper around
    /// [`clap::Parser::parse()`], but is defined here so that we can call it without importing
    /// `clap::Parser` at the call site.
    pub fn parse_from_cli() -> Self {
        Args::parse()
    }
}
