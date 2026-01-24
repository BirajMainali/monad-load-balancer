use serde::{Deserialize, Serialize};

/// Supported load balancing strategies.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[derive(Clone)]
pub enum AlgorithmCfg {
    /// Routes traffic to the server with the fewest active connections.
    LeastConn,
    /// Adjusts routing based on real-time backend latency.
    AdaptiveLeastConn,
    /// Simple sequential distribution.
    RoundRobin,
    /// Distribution based on predefined server weights.
    WeightedRoundRobin,
}
