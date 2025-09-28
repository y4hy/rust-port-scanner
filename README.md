# rust-port-scanner

A fast, minimal TCP port scanner written in Rust. It aims to be simple to use, quick to run, and easy to extend.

## Features

- Concurrent scanning for speed
- Human-readable output

## Getting started

### Prerequisites
- Rust toolchain (stable). Install via [rustup](https://rustup.rs/).

### Build
```bash
cargo build --release
```

### Run
Print help to see available flags:
```bash
cargo run --release -- --help
```

Example invocations (adjust to your CLI if different):
```bash
# Scan common ports on a single host
cargo run --release -- -t 192.168.1.10

# Scan a specific list of ports
cargo run --release -- -t 192.168.1.10 -p 1-1024
```

## Ideas

- UDP scanning
- CIDR/host list input
- Output to JSON
- Service banner grabbing
- Rate limiting and backoff

## Notes

- This tool performs TCP connect scanning and requires no special privileges.
- Network scanning can trigger alerts. Only scan hosts you own or have explicit permission to test.
