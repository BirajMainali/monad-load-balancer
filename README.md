# Monad Load Balancer

A high-performance load balancer written in Rust with multiple routing algorithms and health monitoring.

## MVP Features

- **Load Balancing Algorithms**: Round Robin, Weighted Round Robin, Least Connections, Adaptive Least Connections
- **Health Monitoring**: Automatic backend health checks with circuit breaking
- **Configuration**: YAML-based configuration with hot-reload support
- **Logging**: Structured logging with console and file exporters

## Quick Start

```bash
# Build
cargo build --release

# Configure
cp config.yaml.example config.yaml
# Edit config.yaml with your backend servers

# Run
./target/release/monad_load_balancer
```

## Documentation

- [Installation Guide](docs/setup.md) - Setup and installation instructions
- [Configuration Guide](docs/installation.md) - Configuration options and examples  
- [Algorithms Documentation](docs/algorithms.md) - Load balancing algorithms and components

## Example Configuration

```yaml
balancer:
  algorithm: "adaptive_least_conn"
  check_interval_ms: 500
  port: 9090

backends:
  - id: "api-1"
    address: "10.0.0.1:8080"
    max_connections: 1000
    weight: 10

thresholds:
  latency_critical_ms: 200
  error_rate_limit: 5
  recovery_step: 2
```

## License

MIT License

Copyright (c) 2025 Monad Load Balancer

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.