use crate::config::model::Config;
use tokio::fs;

/// Loads configuration from the default config file
pub async fn get_config() -> anyhow::Result<Config> {
    let content = fs::read_to_string("config.yaml").await?;
    let config: Config = serde_yaml::from_str(&content.as_str())?;
    Ok(config)
}
