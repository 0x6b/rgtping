use std::{
    net::SocketAddr,
    ops::Add,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use gtp_rs::gtpv1::gtpu::{EchoResponse, Gtpv1Header, Messages, ECHO_REQUEST, MIN_HEADER_LENGTH};
use log::{debug, error, trace};
use tokio::{
    net::UdpSocket,
    time::{sleep, sleep_until, Instant},
};

use crate::Stats;

const TRACK_PINGS_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Pinger {
    // UDP socket to send and receive GTPv1-U packets
    socket: UdpSocket,
    // IP address and port of the peer
    peer: SocketAddr,
    // GTPv1-U header to be sent
    packet: Gtpv1Header,
    // Sequence number of the packet
    seq: u16,
    // Internal buffer for sending data
    send_buf: Vec<u8>,
    // Internal buffer to store received data
    recv_buf: [u8; 1024],
    // Number of sent packets (including lost)
    sent: u64,
    // Number of received packets
    received: u64,
    // Array to track times needed to send packets for statistics
    send_times: [f64; TRACK_PINGS_SIZE],
    // Array to track received packets to detect duplicates
    received_packets: [u64; TRACK_PINGS_SIZE],
    // Number of duplicate packets
    duplicate_packets: u64,
    // Number of refused packets
    refused_packets: u64,
    // Epoch time of the start of the operation, in milliseconds
    epoch_ms: u128,
    // Start time of the command for statistics
    start_time: Instant,
    // Last ping time to calculate RTT
    last_ping_time: Instant,
    // Last receive time to calculate RTT
    last_receive_time: Instant,
    // Interval between pings in milliseconds
    interval: Duration,
    // Time to wait for a response in milliseconds
    timeout: Duration,
    // Number of pings to send
    count: u64,
}

impl Pinger {
    pub async fn new(
        peer: SocketAddr,
        count: u64,
        interval_ms: u64,
        timeout_ms: u64,
    ) -> Result<Self> {
        let mut packet = Gtpv1Header {
            msgtype: ECHO_REQUEST,
            sequence_number: Some(0),
            ..Gtpv1Header::default()
        };
        // GTPv1 minimum header size is 8 bytes. When options are present,
        // the header size is 12 bytes. The length field is the total
        //length of optional header + payload.
        let header_size = packet.get_header_size() as u16;
        packet.length = header_size - MIN_HEADER_LENGTH as u16;
        let epoch_ms = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        let now = Instant::now();
        let pinger = Pinger {
            socket: UdpSocket::bind("0.0.0.0:0").await?,
            peer,
            packet,
            seq: 0,
            send_buf: Vec::with_capacity(header_size as usize),
            recv_buf: [0; 1024],
            sent: 0,
            received: 0,
            send_times: [0f64; TRACK_PINGS_SIZE],
            received_packets: [0u64; TRACK_PINGS_SIZE],
            duplicate_packets: 0,
            refused_packets: 0,
            epoch_ms,
            start_time: now,
            last_ping_time: now,
            last_receive_time: now,
            interval: Duration::from_millis(interval_ms),
            timeout: Duration::from_secs(if timeout_ms == 0 { u64::MAX } else { timeout_ms }),
            count,
        };
        debug!("Pinger created for {}", pinger.peer);
        Ok(pinger)
    }

    pub async fn ping(&mut self) -> Result<()> {
        debug!("Start pinging for {}", self.peer);
        for _ in 0..self.count {
            trace!("Sending packet with seq {}", self.seq);
            self.packet.sequence_number = Some(self.seq);
            self.send_buf.clear();
            self.packet.marshal(&mut self.send_buf);
            match self.socket.send_to(&self.send_buf, self.peer).await {
                Ok(_) => {
                    trace!(
                        "Sent data {} to {}",
                        &self
                            .send_buf
                            .iter()
                            .fold(String::new(), |acc, &b| format!("{acc} {b:02X}"))
                            .trim(),
                        self.peer
                    );
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::ConnectionRefused {
                        error!("Connection refused");
                    }
                    self.refused_packets += 1;
                    continue;
                }
            }

            self.seq += 1;
            self.sent += 1;
            self.last_ping_time = Instant::now();

            trace!("Waiting for response");
            tokio::select! {
                _ = async { sleep_until(Instant::now().add(self.timeout)).await } => {
                    debug!("Timed out");
                }
                result = self.socket.recv_from(&mut self.recv_buf) => {
                    match result {
                        Ok((n, src)) => {
                            trace!("Received data {} from {}", &self.recv_buf[..n].iter()
                                .fold(String::new(), |acc, &b| format!("{acc} {b:02X}"))
                                .trim(),
                                 self.peer);
                            let response = EchoResponse::unmarshal(&self.recv_buf[..n])?;
                            self.last_receive_time = Instant::now();
                            let duration = self
                                .last_receive_time
                                .duration_since(self.last_ping_time)
                                .as_secs_f64()
                                * 1000f64;
                            debug!(
                                "{n} bytes from {}: ver={} seq={} time={:.2} ms {}",
                                src.ip(),
                                self.recv_buf[0] >> 5, // always 1, though
                                response.header.sequence_number.unwrap_or(0),
                                duration,
                                if self.received_packets[self.seq as usize % TRACK_PINGS_SIZE] > 1 {
                                    "(DUP)"
                                } else {
                                    ""
                                }
                            );

                            // Some servers send a reply with a sequence number of 0, so we need to check if the
                            // sequence number is set and if not, use the last sequence number
                            let seq = if let Some(seq) = response.header.sequence_number {
                                seq
                            } else {
                                self.packet.sequence_number.unwrap_or(0)
                            };

                            // Update stats
                            self.send_times[seq as usize % TRACK_PINGS_SIZE] = duration;
                            self.received_packets[seq as usize % TRACK_PINGS_SIZE] += 1;
                            if self.received_packets[seq as usize % TRACK_PINGS_SIZE] > 1 {
                                self.duplicate_packets += 1;
                            }
                            self.received += 1;

                            sleep(self.interval).await;
                        },
                        Err(_) => {
                            error!("Port closed");
                            self.seq += 1;
                            sleep(self.interval).await;
                            continue;
                        }
                    }
                }

            }
        }

        debug!("Finish pinging for {}", self.peer);
        trace!(
            "Stats: epoch: {}, sent: {}, received: {}, duplicate: {}, refused: {}, seq: {}",
            self.epoch_ms,
            self.sent,
            self.received,
            self.duplicate_packets,
            self.refused_packets,
            self.seq
        );
        Ok(())
    }

    pub fn calculate_stats(&self) -> Stats {
        let mut min = 0f64;
        let mut max = 0f64;
        let mut sum = 0.0;
        let mut count = 0;

        for &time in self.send_times.iter().filter(|&&time| time > 0f64) {
            min = min.min(time);
            max = max.max(time);
            sum += time;
            count += 1;
        }

        let avg = sum / count as f64;

        let variance = self
            .send_times
            .iter()
            .filter(|&&time| time > 0f64)
            .map(|&time| (time - avg) * (time - avg))
            .sum::<f64>()
            / count as f64;
        let mdev = variance.sqrt();

        Stats {
            target: self.peer.to_string(),
            epoch_ms: self.epoch_ms,
            sent: self.sent,
            received: self.received,
            packet_loss_percentage: (self.sent - self.received) as f64 / self.sent as f64 * 100f64,
            duplicate_packets: self.duplicate_packets,
            refused_packets: self.refused_packets,
            duration: self.start_time.elapsed().as_secs_f64() * 1000f64,
            min,
            avg,
            max,
            mdev,
        }
    }
}
