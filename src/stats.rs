use std::fmt::Display;

use serde::Serialize;

/// Holds results from a series of GTP ping commands
#[derive(Debug, Serialize)]
pub struct Stats {
    /// Gtping target IP address
    pub target: String,
    /// Epoch time of the start of the command, in milliseconds
    pub epoch_ms: u128,
    /// Total duration in milliseconds
    pub duration: f64,
    /// Number of sent packets (including lost)
    pub sent: u64,
    /// Number of received packets
    pub received: u64,
    /// Packet loss percentage
    pub packet_loss_percentage: f64,
    /// Number of duplicate packets
    pub duplicate_packets: u64,
    /// Number of refused packets
    pub refused_packets: u64,
    /// Number of timed out packets
    pub timed_out_packets: i32,
    /// Minimum RTT in milliseconds
    pub min: f64,
    /// Maximum RTT in milliseconds
    pub max: f64,
    /// Average RTT in milliseconds
    pub avg: f64,
    /// Mean deviation of RTT in milliseconds
    pub mdev: f64,
}

impl Display for Stats {
    /// Formats the statistics in a human-readable format, as close as possible to ping
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push('\n');
        s.push_str(&format!("--- {} GTP ping statistics ---", self.target));
        s.push('\n');
        s.push_str(&format!(
            "{} packets transmitted, {} received, {:.2}% packet loss, time {:.0}ms",
            self.sent,
            self.received,
            (self.sent - self.received) as f64 / self.sent as f64 * 100f64,
            self.duration
        ));
        s.push('\n');
        s.push_str(&format!(
            "{} dups, {} connection refused, {} timed out",
            self.duplicate_packets, self.refused_packets, self.timed_out_packets
        ));
        s.push('\n');
        s.push_str(&format!(
            "rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/{:.3} ms",
            self.min, self.avg, self.max, self.mdev
        ));
        write!(f, "{}", s)
    }
}
