# rgtping

ping(8) equivalent for GTPv1-U (3GPP TS 29.281).

> [!WARNING]
> Error handling is not really implemented. This is a toy project for my own learning. Not recommended for production monitoring.

## Features

- Send GTPv1-U Echo request to multiple endpoints simultaneously.
- Output is an array of JSON objects, one for each endpoint e.g.
    ```json5
    [
      {
        "target": "192.168.205.10:2152",
        "sent": 10, // Number of packets sent
        "received": 10, // Number of packets received
        "packet_loss_percentage": 0.0, // Packet loss percentage
        "duplicate_packets": 0, // Number of duplicate packets
        "refused_packets": 0, // Number of refused packets (maybe incorrect)
        "duration": 10164.467290999999, // Total duration in milliseconds
        "min": 0.0, // Minimum RTT in milliseconds
        "max": 53.558417, // Maximum RTT in milliseconds
        "avg": 13.0501917, // Average RTT in milliseconds
        "mdev": 14.24073989248037, // Standard deviation of RTT in milliseconds
        "epoch_ms": 1726323245070 // epoch time of the start of the operation, in milliseconds
      }
    ]
    ```

## Usage

```console
$ rgtping --help
Usage: rgtping [OPTIONS] [TARGET_IPS]...

Arguments:
  [TARGET_IPS]...  Array of IP address and port number (IP:port) to ping [default: 192.168.205.10:2152]

Options:
  -c, --count <COUNT>              Number of pings to send [default: 5]
  -i, --interval-ms <INTERVAL_MS>  Interval between pings in milliseconds [default: 1000]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Acknowledgement

- [ThomasHabets/gtping: GTP Ping](https://github.com/ThomasHabets/gtping/) for the inspiration. Note that this project is not a fork, and won't be compatible with the original gtping.
- [ErvinsK/gtp-rs: Pure Rust implementation of 3GPP GTP (GPRS Tunneling Protocol) - GTPv1 and GTPv2](https://github.com/ErvinsK/gtp-rs) for doing the all the heavy lifting. It's a cool project.

## LICENSE

GPLv2, as the original gtping. See [LICENSE](LICENSE) for detail.
