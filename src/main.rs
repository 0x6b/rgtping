mod args;

use anyhow::{Ok, Result};
use args::Args;
use env_logger::Env;
use rgtping::Pinger;
use tokio::spawn;

use crate::args::Format;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let Args { target_ips, count, interval_ms, timeout_ms, format } = Args::parse_from_cli();

    // Holds the pinger instances
    let mut pingers = Vec::with_capacity(target_ips.len());
    // Holds the handles to the spawned threads which actually send the pings
    let mut handles = Vec::with_capacity(target_ips.len());
    // Holds the results of the pings from each thread
    let mut results = Vec::with_capacity(target_ips.len());

    // Create a pinger for each target IP
    for target in target_ips {
        pingers.push(Pinger::new(target, count, interval_ms, timeout_ms).await?);
    }

    // Spawn a thread for each pinger to send the pings
    for mut pinger in pingers {
        handles.push(spawn(async move {
            pinger.ping().await?;
            Ok(pinger.calculate_stats())
        }));
    }

    // Wait for all the threads to finish and collect the results
    for handle in handles {
        results.push(handle.await??);
    }

    // Print the results in JSON format
    match format {
        Format::Json => println!("{}", serde_json::to_string_pretty(&results)?),
        Format::Text => results.iter().for_each(|s| println!("{s}")),
    }

    Ok(())
}
