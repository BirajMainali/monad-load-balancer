use crate::config::backend_config::BackendConfig;
use crate::config::balancer_server_config::BalancerServerConfig;
use crate::config::thresholds_config::ThresholdsConfig;
use serde::{Deserialize, Serialize};
use tokio::fs;

/// Root configuration for the load balancer system.
///
/// This structure maps directly to the top-level keys of the configuration file.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct LoadBalancerConfig {
    /// Core settings for the balancing logic and networking.
    #[serde(rename = "balancer")]
    pub balancer_cfg: BalancerServerConfig,

    /// A list of destination servers for traffic distribution.
    #[serde(rename = "backends")]
    pub backends: Vec<BackendConfig>,

    /// Safety limits and recovery parameters for traffic management.
    #[serde(rename = "thresholds")]
    pub thresholds_cfg: ThresholdsConfig,
}

impl LoadBalancerCfg {
    /// Loads configuration from the default config file
    pub async fn load() -> anyhow::Result<LoadBalancerConfig> {
        let raw = fs::read_to_string("config.yaml").await?;
        let cfg: LoadBalancerConfig = serde_yaml::from_str(&raw.as_str())?;
        Ok(cfg)
    }
}
