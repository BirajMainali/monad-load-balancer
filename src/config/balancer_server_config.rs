use crate::config::algorithm_config::AlgorithmConfig;
use serde::{Deserialize, Serialize};

/// Settings defining how the balancer operates.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct BalancerServerConfig {
    /// The specific strategy used to distribute traffic.
    #[serde[rename = "algorithm"]]
    pub algorithm: AlgorithmConfig,

    /// Frequency of health checks in milliseconds.
    #[serde(rename = "check_interval_ms")]
    pub check_interval_ms: f32,

    /// The network port the load balancer listens on.
    #[serde(rename = "port")]
    pub port: i16,
}
