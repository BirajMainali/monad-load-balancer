# Installation and Configuration Guide

This guide covers how to install and configure the Monad Load Balancer for different deployment scenarios.

## Configuration File Structure

The load balancer uses a YAML configuration file (`config.yaml`) with three main sections:

### 1. Balancer Configuration

Defines the core load balancing behavior:

```yaml
balancer:
  algorithm: "adaptive_least_conn"    # Load balancing algorithm
  check_interval_ms: 500              # Health check frequency in milliseconds
  port: 9090                         # Port for incoming connections
```

**Field Descriptions:**
- `algorithm`: The load balancing algorithm to use. Supported values:
  - `least_conn` - Routes to server with fewest active connections
  - `adaptive_least_conn` - Routes based on real-time latency and connections
  - `round_robin` - Simple sequential distribution
  - `weighted_round_robin` - Distribution based on server weights
- `check_interval_ms`: How often to perform health checks (500-5000ms recommended)
- `port`: TCP port for client connections (1-65535)

### 2. Backend Configuration

List of destination servers:

```yaml
backends:
  - id: "srv-01"                     # Unique server identifier
    address: "10.0.0.1:8080"         # IP:port of the backend server
    max_connections: 1000             # Maximum concurrent connections
    weight: 10                       # Relative weight/capacity
  
  - id: "srv-02"
    address: "10.0.0.2:8080"
    max_connections: 1000
    weight: 10
```

**Field Descriptions:**
- `id`: Unique identifier for logging and monitoring
- `address`: Backend server address in `IP:PORT` format
- `max_connections`: Maximum concurrent connections allowed
- `weight`: Relative weight (only used by weighted algorithms)

### 3. Thresholds Configuration

Health monitoring and circuit breaking rules:

```yaml
thresholds:
  latency_critical_ms: 200            # Max response time before marking as unhealthy
  error_rate_limit: 5                # Error rate percentage (0-100)
  recovery_step: 2                   # Weight increment during recovery
```

**Field Descriptions:**
- `latency_critical_ms`: Response time threshold in milliseconds
- `error_rate_limit`: Error rate percentage that triggers circuit breaking
- `recovery_step`: How quickly to restore server weight during recovery

## Installation Scenarios

### Development Environment

1. **Clone and Build**
   ```bash
   git clone <repository>
   cd monad-load-balancer
   cargo build
   ```

2. **Configure Development Backends**
   ```yaml
   balancer:
     algorithm: "round_robin"
     check_interval_ms: 1000
     port: 9090
   
   backends:
     - id: "dev-api-1"
       address: "127.0.0.1:3001"
       max_connections: 100
       weight: 1
     
     - id: "dev-api-2"
       address: "127.0.0.1:3002"
       max_connections: 100
       weight: 1
   ```

3. **Run Development Server**
   ```bash
   cargo run
   ```

### Production Environment

#### High Availability Setup

```yaml
balancer:
  algorithm: "adaptive_least_conn"
  check_interval_ms: 200
  port: 80

backends:
  - id: "prod-api-1"
    address: "10.0.1.10:8080"
    max_connections: 5000
    weight: 100
  
  - id: "prod-api-2"
    address: "10.0.1.11:8080"
    max_connections: 5000
    weight: 100
  
  - id: "prod-api-3"
    address: "10.0.1.12:8080"
    max_connections: 5000
    weight: 50

thresholds:
  latency_critical_ms: 100
  error_rate_limit: 2
  recovery_step: 5
```

#### Microservices Architecture

```yaml
balancer:
  algorithm: "least_conn"
  check_interval_ms: 500
  port: 80

backends:
  # Service A - High traffic
  - id: "service-a-1"
    address: "10.0.2.1:9001"
    max_connections: 2000
    weight: 80
  
  - id: "service-a-2"
    address: "10.0.2.2:9001"
    max_connections: 2000
    weight: 80
  
  # Service B - Medium traffic
  - id: "service-b-1"
    address: "10.0.3.1:9002"
    max_connections: 1000
    weight: 40
  
  - id: "service-b-2"
    address: "10.0.3.2:9002"
    max_connections: 1000
    weight: 40
```

## Algorithm Selection Guide

### Choose `least_conn` when:
- Backend servers have similar capabilities
- Connection count is the primary bottleneck
- You want simple, predictable behavior

### Choose `adaptive_least_conn` when:
- Backend performance varies significantly
- Response times are critical
- You have heterogeneous server infrastructure

### Choose `round_robin` when:
- All backends are identical
- You want maximum simplicity
- Performance is not critical

### Choose `weighted_round_robin` when:
- Backend servers have different capacities
- You want to allocate traffic proportionally
- Server capabilities are known and stable

## Performance Tuning

### Connection Management

```yaml
# For high-throughput scenarios
backends:
  - id: "high-capacity-server"
    address: "10.0.1.100:8080"
    max_connections: 10000    # Increase for powerful servers
    weight: 100              # Higher weight for better performance
```

### Health Check Optimization

```yaml
# For critical services
balancer:
  check_interval_ms: 200    # More frequent checks

thresholds:
  latency_critical_ms: 50   # Stricter latency requirements
  error_rate_limit: 1       # Lower error tolerance
```

### Resource Allocation

```yaml
# For mixed-capacity environments
backends:
  # Primary servers
  - id: "primary-1"
    address: "10.0.1.10:8080"
    max_connections: 5000
    weight: 100
  
  - id: "primary-2"
    address: "10.0.1.11:8080"
    max_connections: 5000
    weight: 100
  
  # Backup servers
  - id: "backup-1"
    address: "10.0.2.10:8080"
    max_connections: 2000
    weight: 25
  
  - id: "backup-2"
    address: "10.0.2.11:8080"
    max_connections: 2000
    weight: 25
```

## Advanced Configuration

### Environment-Specific Configs

Create multiple config files:
- `config.dev.yaml` - Development environment
- `config.staging.yaml` - Staging environment  
- `config.prod.yaml` - Production environment

Select using environment variable:
```bash
CONFIG_PATH=config.prod.yaml ./target/release/monad_load_balancer
```

### Configuration Validation

Before starting, validate your configuration:

```bash
# Check YAML syntax
python -c "import yaml; yaml.safe_load(open('config.yaml'))"

# Test with dry run (if implemented)
./target/release/monad_load_balancer --dry-run --config config.yaml
```

### Monitoring Integration

The load balancer logs to both console and file. Configure log rotation:

```bash
# logrotate configuration for /etc/logrotate.d/monad-load-balancer
/opt/monad-load-balancer/log.txt {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 loadbalancer loadbalancer
}
```

## Security Considerations

### Network Security

1. **Firewall Configuration**
   ```bash
   # Allow only load balancer port
   sudo ufw allow 9090/tcp
   sudo ufw deny from 10.0.0.0/8 to any port 8080
   ```

2. **Backend Isolation**
   ```yaml
   backends:
     - id: "internal-api-1"
       address: "10.0.1.10:8080"  # Use internal IPs
       max_connections: 1000
       weight: 10
   ```

### Configuration Security

1. **File Permissions**
   ```bash
   chmod 600 config.yaml
   chown loadbalancer:loadbalancer config.yaml
   ```

2. **Backup Configuration**
   ```bash
   cp config.yaml config.yaml.backup
   chmod 600 config.yaml.backup
   ```

## Troubleshooting Configuration Issues

### Common Problems

1. **Backend Unreachable**
   - Verify backend addresses are correct
   - Check network connectivity
   - Ensure backend services are running

2. **Incorrect Algorithm**
   - Validate algorithm name in configuration
   - Check for typos in algorithm specification

3. **Port Conflicts**
   - Ensure the specified port is available
   - Check for other services using the same port

### Debug Mode

Enable verbose logging:
```bash
RUST_LOG=debug ./target/release/monad_load_balancer
```

### Configuration Testing

Test configuration changes safely:
1. Create a test configuration file
2. Validate syntax
3. Run with test backends
4. Monitor logs for errors
5. Gradually apply to production