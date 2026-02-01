# Monad Load Balancer Setup Guide

This guide will help you set up and run the Monad Load Balancer from source.

## Prerequisites

### System Requirements
- **Operating System**: Linux, macOS, or Windows
- **Rust**: Latest stable version (2024 edition)
- **Memory**: Minimum 512MB RAM
- **Disk Space**: 50MB for the application and logs

### Installing Rust

If you don't have Rust installed, use rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

## Getting the Source Code

```bash
git clone <repository-url>
cd monad-load-balancer
```

## Building the Project

### Development Build

```bash
cargo build
```

### Production Build

```bash
cargo build --release
```

The production binary will be located at:
- Linux/macOS: `target/release/monad_load_balancer`
- Windows: `target/release/monad_load_balancer.exe`

## Running the Load Balancer

### 1. Configure the System

Copy and modify the configuration file:
```bash
cp config.yaml.example config.yaml
```

Edit `config.yaml` to match your backend servers and requirements.

### 2. Start the Load Balancer

For development:
```bash
cargo run
```

For production:
```bash
./target/release/monad_load_balancer
```

### 3. Verify Installation

The load balancer will:
- Start listening on the configured port (default: 9090)
- Begin health monitoring of backend servers
- Start logging to both console and file (`log.txt`)

Test with curl:
```bash
curl http://localhost:9090
```

## Development Workflow

### Running Tests

```bash
cargo test
```

### Running Integration Tests

```bash
cargo test --test integration_tests
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Troubleshooting

### Port Already in Use
If you get "address already in use" error:
1. Check what's using the port: `netstat -tulpn | grep 9090`
2. Kill the process or change the port in `config.yaml`

### Backend Connection Issues
1. Verify backend servers are running
2. Check network connectivity
3. Review health check logs in `log.txt`

### Configuration Errors
1. Validate YAML syntax: `python -c "import yaml; yaml.safe_load(open('config.yaml'))"`
2. Check all required fields are present
3. Verify backend addresses are correct

## Environment Variables

You can override configuration using environment variables:

- `RUST_LOG`: Set logging level (e.g., `RUST_LOG=debug`)
- `CONFIG_PATH`: Path to configuration file (default: `config.yaml`)

Example:
```bash
RUST_LOG=debug cargo run
```

## Production Considerations

### Systemd Service

Create `/etc/systemd/system/monad-load-balancer.service`:

```ini
[Unit]
Description=Monad Load Balancer
After=network.target

[Service]
Type=simple
User=loadbalancer
WorkingDirectory=/opt/monad-load-balancer
ExecStart=/opt/monad-load-balancer/target/release/monad_load_balancer
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable monad-load-balancer
sudo systemctl start monad-load-balancer
```

### Docker Support

Build Docker image:
```bash
docker build -t monad-load-balancer .
```

Run with Docker:
```bash
docker run -p 9090:9090 -v $(pwd)/config.yaml:/app/config.yaml monad-load-balancer
```

### Monitoring

- Monitor log file: `tail -f log.txt`
- Check process status: `systemctl status monad-load-balancer`
- Monitor resource usage: `htop` or `top`