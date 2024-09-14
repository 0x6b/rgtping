use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Stats {
    pub target: String,
    pub sent: u64,
    pub received: u64,
    pub packet_loss_percentage: f64,
    pub duplicate_packets: u64,
    pub refused_packets: u64,
    pub duration: f64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub mdev: f64,
    pub epoch_ms: u128,
}

impl Display for Stats {
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
            "{} dups, {} connection refused",
            self.duplicate_packets, self.refused_packets
        ));
        s.push('\n');
        s.push_str(&format!(
            "rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/{:.3} ms",
            self.min, self.avg, self.max, self.mdev
        ));
        write!(f, "{}", s)
    }
}
