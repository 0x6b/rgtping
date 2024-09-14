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

    let mut pingers = Vec::with_capacity(args.target_ips.len());
    let mut handles = Vec::with_capacity(args.target_ips.len());
    let mut results = Vec::with_capacity(args.target_ips.len());

    for target in args.target_ips {
        let pinger = Pinger::new(target, args.count, args.interval_ms).await?;
        pingers.push(pinger);
    }

    for mut pinger in pingers {
        let handle = spawn(async move {
            pinger.ping().await?;
            Ok::<Stats, Error>(pinger.calculate_stats())
        });
        handles.push(handle);
    }

    for handle in handles {
        results.push(handle.await??);
    }

    println!("{}", serde_json::to_string_pretty(&results)?);

    Ok(())
}
