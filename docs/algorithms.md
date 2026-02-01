# Algorithms and Components Documentation

This document describes the supported load balancing algorithms and how each component contributes to the overall load balancing system.

## Load Balancing Algorithms

### 1. Round Robin

**Description**: Simple sequential distribution of connections across all available backends.

**Implementation**: `src/algorithms/core/round_robin.rs`

**How it works**:
- Maintains an atomic cursor that increments for each request
- Uses modulo operation to cycle through backends
- No consideration for backend health, capacity, or performance

**Pseudo-code**:
```rust
cursor += 1
selected_backend = cursor % total_backends
```

**Pros**:
- Simple and predictable
- No overhead
- Equal distribution regardless of backend characteristics

**Cons**:
- Ignores backend health status
- Doesn't consider server capacity
- May send traffic to overloaded or failing backends

**Best for**:
- Homogeneous server environments
- Development and testing
- When simplicity is prioritized over performance

---

### 2. Weighted Round Robin

**Description**: Distributes traffic based on predefined server weights/capacities.

**Implementation**: `src/algorithms/core/weighted_round_robin.rs`

**How it works**:
- Calculates total weight of all eligible backends
- Uses modulo operation on total weight
- Iterates through backends to find the weight range containing the selected value
- Falls back to regular round-robin if all weights are zero

**Pseudo-code**:
```rust
total_weight = sum(backend.current_weight for each backend)
cursor += 1
target_weight = cursor % total_weight

for each backend:
    if target_weight < backend.current_weight:
        return backend
    target_weight -= backend.current_weight
```

**Backend Field Contribution**:
- `current_weight`: Current effective weight (can be adjusted by health system)
- `base_weight`: Original configured weight

**Pros**:
- Respects server capacity differences
- Gradual weight adjustment by health system
- Predictable distribution patterns

**Cons**:
- Still ignores real-time performance
- More complex than basic round-robin

**Best for**:
- Heterogeneous server environments
- When server capacities are known and relatively static
- Backup server scenarios

---

### 3. Least Connections

**Description**: Routes traffic to the server with the fewest active connections.

**Implementation**: `src/algorithms/core/least_conn.rs`

**How it works**:
- Iterates through all eligible backends
- Finds backend with minimum active connection count
- Returns the index of the least-loaded backend

**Pseudo-code**:
```rust
selected_backend = argmin(backend.active_conn for each backend)
```

**Backend Field Contribution**:
- `active_conn`: Real-time count of active connections

**Pros**:
- Considers real-time load
- Simple and efficient
- Better utilization than round-robin

**Cons**:
- Doesn't consider connection duration or server performance
- May send traffic to slow but less busy servers

**Best for**:
- When connection count is the primary bottleneck
- Similar-performing servers
- Real-time load balancing needs

---

### 4. Adaptive Least Connections

**Description**: Advanced algorithm that considers both connection count and latency metrics.

**Implementation**: `src/algorithms/core/adaptive_least_conn.rs`

**How it works**:
- Calculates adaptive score for each backend
- Combines connection count and average latency
- Uses weighted formula where connections are prioritized over latency
- Selects backend with lowest adaptive score

**Pseudo-code**:
```rust
for each backend:
    score = (active_conn * 1000) + avg_latency_ms
selected_backend = argmin(score for each backend)
```

**Backend Field Contribution**:
- `active_conn`: Current connection count
- `avg_latency_ms`: Rolling average response time in milliseconds

**Pros**:
- Considers both load and performance
- Adapts to changing conditions
- Better overall performance metrics

**Cons**:
- More complex scoring logic
- Requires accurate latency tracking

**Best for**:
- Heterogeneous server environments
- Performance-critical applications
- Production environments with varying server capabilities

## Core Components

### Backend State Management

**File**: `src/state/backend.rs`

The `Backend` struct maintains the runtime state of each backend server:

**Key Fields**:
- `id`: Unique server identifier for logging/monitoring
- `addr`: Network address (IP:port) for connection routing
- `max_conn`: Maximum allowed concurrent connections
- `base_weight`: Original configured weight
- `current_weight`: Current effective weight (adjusted by health system)
- `active_conn`: Real-time active connection count
- `avg_latency_ms`: Rolling average latency for performance metrics

**Methods**:
- `from_cfg()`: Creates backend from configuration
- `exceeds_latency_threshold()`: Checks if latency exceeds critical threshold
- `is_weight_low()`: Determines if weight is at or below base level
- `is_currently_booting()`: Checks if backend is in recovery state
- `has_no_weight()`/`has_some_weight()`: Weight status checks
- `is_max_conn_reached()`: Connection limit check

### Health Monitoring System

**File**: `src/health/health.rs`

The health monitoring system continuously evaluates backend health and adjusts routing behavior:

**Key Functions**:
- Periodic health checks at configured intervals
- Latency measurement and threshold monitoring
- Circuit breaking when error rates exceed limits
- Gradual weight recovery for failing backends

**Health Algorithm**:
1. Measure backend latency and error rates
2. Compare against thresholds in configuration
3. Adjust `current_weight` based on health status
4. Implement circuit breaking for consistently failing backends
5. Gradually restore weights during recovery

### Configuration System

**Files**: `src/config/*.rs`

**Configuration Structure**:

#### Balancer Configuration (`balancer_server_cfg.rs`)
- `algorithm`: Selected load balancing algorithm
- `check_interval_ms`: Health check frequency
- `port`: Listening port for client connections

#### Backend Configuration (`backend_cfg.rs`)
- `id`: Server identifier
- `address`: Backend server address
- `max_connections`: Connection limit
- `weight`: Relative capacity weight

#### Thresholds Configuration (`thresholds_cfg.rs`)
- `latency_critical_ms`: Latency threshold for circuit breaking
- `error_rate_limit`: Error rate percentage limit
- `recovery_step`: Weight increment during recovery

### Connection Routing

**File**: `src/balancer/balancer.rs`

The main balancer component coordinates all routing decisions:

**Routing Flow**:
1. Accept incoming connection
2. Get list of healthy backends from health system
3. Apply selected algorithm to choose backend
4. Update backend connection count
5. Establish proxy connection to selected backend
6. Monitor connection and update metrics

### Logging and Monitoring

**Files**: `src/logging/*.rs`

**Logging System Components**:
- **Console Exporter**: Real-time logging to stdout
- **File Exporter**: Persistent logging to file system
- **Event System**: Structured logging for different event types

**Log Events**:
- Connection routing decisions
- Health check results
- Backend status changes
- Error conditions and recovery events

## Algorithm Selection Guide

### Decision Matrix

| Scenario | Recommended Algorithm | Rationale |
|----------|---------------------|-----------|
| Development/Test | Round Robin | Simple, predictable, no overhead |
| Production with similar servers | Least Connections | Considers real-time load |
| Production with varied capacity | Weighted Round Robin | Respects server differences |
| Performance-critical production | Adaptive Least Connections | Best overall performance |
| Mixed environment with backups | Weighted Round Robin | Can give less weight to backups |

### Performance Characteristics

| Algorithm | Time Complexity | Space Complexity | Health Awareness | Performance Consideration |
|-----------|----------------|------------------|------------------|-------------------------|
| Round Robin | O(1) | O(1) | No | None |
| Weighted Round Robin | O(n) | O(1) | Yes (via weight) | None |
| Least Connections | O(n) | O(1) | Yes (via conn count) | Connection load only |
| Adaptive Least Conn | O(n) | O(1) | Yes | Both load and performance |

## Field Contribution Summary

### Configuration Fields
- **algorithm**: Determines routing strategy
- **check_interval_ms**: Controls health monitoring frequency
- **port**: Network endpoint for client connections
- **max_connections**: Prevents backend overload
- **weight**: Influences traffic distribution (weighted algorithms)
- **latency_critical_ms**: Circuit breaking threshold
- **error_rate_limit**: Health degradation threshold
- **recovery_step**: Recovery speed for failed backends

### Runtime Fields
- **active_conn**: Input for connection-based algorithms
- **current_weight**: Dynamic capacity adjustment
- **avg_latency_ms**: Performance metric input for adaptive algorithms

Each field serves a specific purpose in the load balancing ecosystem, from initial configuration decisions to real-time routing optimizations. The combination of these fields enables intelligent traffic distribution while maintaining system stability and performance.