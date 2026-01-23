use crate::config::backend_cfg::Backend;
use crate::config::balancer_cfg::BalancerCfg;
use crate::config::thresholds_cfg::Thresholds;
use serde::{Deserialize, Serialize};

/// Root configuration for the load balancer system.
///
/// This structure maps directly to the top-level keys of the configuration file.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    /// Core settings for the balancing logic and networking.
    #[serde(rename = "balancer")]
    pub balancer_cfg: BalancerCfg,

    /// A list of destination servers for traffic distribution.
    #[serde(rename = "backends")]
    pub backends: Vec<Backend>,

    /// Safety limits and recovery parameters for traffic management.
    #[serde(rename = "thresholds")]
    pub thresholds: Thresholds,
}

impl Config {
    /// Loads configuration from the default config file }
    pub async fn load() -> anyhow::Result<Config> {
        let raw = fs::read_to_string("config.yaml").await?;
        let cfg: Config = serde_yaml::from_str(&raw.as_str())?;
        Ok(cfg)
    }
}
