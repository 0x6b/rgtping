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
        "target": "192.168.205.10:2152", /// Gtping target IP address
        "epoch_ms": 1726372276714, /// Epoch time of the start of the command, in milliseconds
        "duration": 3026.165084, /// Total duration in milliseconds
        "sent": 3, /// Number of sent packets (including lost)
        "received": 3, /// Number of received packets
        "packet_loss_percentage": 0.0, /// Packet loss percentage
        "duplicate_packets": 0, /// Number of duplicate packets
        "refused_packets": 0, /// Number of refused packets
        "min": 0.0, /// Minimum RTT in milliseconds
        "max": 14.750917000000001, /// Maximum RTT in milliseconds
        "avg": 5.607903, /// Average RTT in milliseconds
        "mdev": 6.4650903803150355 /// Mean deviation of RTT in milliseconds
      }
      // ...
    ]
    ```

## Usage

```console
ping(8) equivalent for GTPv1-U (3GPP TS 29.281).

Usage: rgtping [OPTIONS] [TARGET_IPS]...

Arguments:
  [TARGET_IPS]...  Array of IP address and port number (IP:port) to ping, delimited
                   by a space

Options:
  -c, --count <COUNT>
          Number of pings to send [default: 5]
  -i, --interval-ms <INTERVAL_MS>
          Interval between pings in milliseconds [default: 1000]
  -W, --timeout-ms <TIMEOUT_MS>
          Time to wait for a response, in milliseconds. 0 means wait indefinitely
          [default: 10000]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Acknowledgement

- [ThomasHabets/gtping: GTP Ping](https://github.com/ThomasHabets/gtping/) for the inspiration. Note that this project is not a fork, and won't be compatible with the original gtping.
- [ErvinsK/gtp-rs: Pure Rust implementation of 3GPP GTP (GPRS Tunneling Protocol) - GTPv1 and GTPv2](https://github.com/ErvinsK/gtp-rs) for doing the all the heavy lifting. It's a cool project.

## LICENSE

GPLv2, as the original gtping. See [LICENSE](LICENSE) for detail.
