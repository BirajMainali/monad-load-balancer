use async_trait::async_trait;

#[async_trait]
pub trait LogExporter: Send + Sync {
    async fn export(&self, log: &str) -> anyhow::Result<()>;
}
