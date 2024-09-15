use std::net::SocketAddr;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Array of IP address and port number (IP:port) to ping, delimited by a space
    #[arg(default_value = "192.168.205.10:2152", value_delimiter = ' ')]
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
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
