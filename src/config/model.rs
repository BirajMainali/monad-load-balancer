use serde::{Deserialize, Serialize};

/// Root configuration for the load balancer system.
///
/// This structure maps directly to the top-level keys of the configuration file.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    /// Core settings for the balancing logic and networking.
    #[serde(rename = "balancer")]
    pub balancer: Balancer,

    /// A list of destination servers for traffic distribution.
    #[serde(rename = "backends")]
    pub backends: Vec<Backend>,

    /// Safety limits and recovery parameters for traffic management.
    #[serde(rename = "thresholds")]
    pub thresholds: Thresholds,
}

/// Settings defining how the balancer operates.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Balancer {
    /// The specific strategy used to distribute traffic.
    #[serde[rename = "algorithm"]]
    pub algorithm: Algorithm,

    /// Frequency of health checks in milliseconds.
    #[serde(rename = "check_interval_ms")]
    pub check_interval_ms: f32,

    /// The network port the load balancer listens on.
    #[serde(rename = "port")]
    pub port: i16,
}

/// A specific server destination where traffic is routed.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Backend {
    /// Unique identifier for the server (e.g., "srv-01").
    #[serde(rename = "id")]
    pub id: String,

    /// The IP address and port (e.g., "10.0.0.1:8080").
    #[serde(rename = "address")]
    pub address: String,

    /// Maximum concurrent connections allowed for this backend.
    #[serde(rename = "max_connections")]
    pub max_connections: i32,

    /// Relative priority/capacity of this backend compared to others.
    #[serde(rename = "weight")]
    pub weight: f32,
}

/// Rules for circuit breaking and health monitoring.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Thresholds {
    /// Maximum response time allowed before a backend is flagged.
    #[serde(rename = "latency_critical_ms")]
    pub latency_critical_ms: f32,

    /// The ratio (0.0 - 1.0) of allowed failed requests.
    #[serde(rename = "error_rate_limit")]
    pub error_rate_limit: f32,

    /// The increment by which weight is restored during server recovery.
    #[serde(rename = "recovery_step")]
    pub recovery_step: f32,
}

/// Supported load balancing strategies.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Algorithm {
    /// Routes traffic to the server with the fewest active connections.
    LeastConn,
    /// Adjusts routing based on real-time backend latency.
    AdaptiveLeastConn,
    /// Simple sequential distribution.
    RoundRobin,
    /// Distribution based on predefined server weights.
    WeightedRoundRobin,
}
