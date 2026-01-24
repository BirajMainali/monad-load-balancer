use serde::{Deserialize, Serialize};

/// A specific server destination where traffic is routed.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct BackendCfg {
    /// Unique identifier for the server (e.g., "srv-01").
    #[serde(rename = "id")]
    pub id: String,

    /// The IP address and port (e.g., "10.0.0.1:8080").
    #[serde(rename = "address")]
    pub address: String,

    /// Maximum concurrent connections allowed for this backend.
    #[serde(rename = "max_connections")]
    pub max_conn: i32,

    /// Relative priority/capacity of this backend compared to others.
    #[serde(rename = "weight")]
    pub weight: f32,
}