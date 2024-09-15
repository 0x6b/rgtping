mod args;

use anyhow::Error;
use args::Args;
use env_logger::Env;
use rgtping::{Pinger, Stats};
use tokio::spawn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::new();

    // Holds the pinger instances
    let mut pingers = Vec::with_capacity(args.target_ips.len());
    // Holds the handles to the spawned threads which actually send the pings
    let mut handles = Vec::with_capacity(args.target_ips.len());
    // Holds the results of the pings from each handle
    let mut results = Vec::with_capacity(args.target_ips.len());

    // Create a pinger for each target IP
    for target in args.target_ips {
        let pinger = Pinger::new(target, args.count, args.interval_ms, args.timeout_ms).await?;
        pingers.push(pinger);
    }

    // Spawn a thread for each pinger to send the pings
    for mut pinger in pingers {
        let handle = spawn(async move {
            pinger.ping().await?;
            Ok::<Stats, Error>(pinger.calculate_stats())
        });
        handles.push(handle);
    }

    // Wait for all the threads to finish and collect the results
    for handle in handles {
        results.push(handle.await??);
    }

    // Print the results in JSON format
    println!("{}", serde_json::to_string_pretty(&results)?);

    Ok(())
}
