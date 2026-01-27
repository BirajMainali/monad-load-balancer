use serde::{Deserialize, Serialize};

/// Rules for circuit breaking and health monitoring.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct ThresholdsCfg {
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
