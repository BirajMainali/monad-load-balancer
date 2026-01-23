use crate::config::backend_cfg::BackendCfg;
use crate::config::balancer_server_cfg::BalancerServerCfg;
use crate::config::thresholds_cfg::ThresholdsCfg;
use serde::{Deserialize, Serialize};
use tokio::fs;

/// Root configuration for the load balancer system.
///
/// This structure maps directly to the top-level keys of the configuration file.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoadBalancerCfg {
    /// Core settings for the balancing logic and networking.
    #[serde(rename = "balancer")]
    pub balancer_cfg: BalancerServerCfg,

    /// A list of destination servers for traffic distribution.
    #[serde(rename = "backends")]
    pub backends: Vec<BackendCfg>,

    /// Safety limits and recovery parameters for traffic management.
    #[serde(rename = "thresholds")]
    pub thresholds_cfg: ThresholdsCfg,
}

impl LoadBalancerCfg {
    /// Loads configuration from the default config file
    pub async fn load() -> anyhow::Result<LoadBalancerCfg> {
        let raw = fs::read_to_string("config.yaml").await?;
        let cfg: LoadBalancerCfg = serde_yaml::from_str(&raw.as_str())?;
        Ok(cfg)
    }
}
