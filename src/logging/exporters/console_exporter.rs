use async_trait::async_trait;
use crate::logging::traits::log_exporter::LogExporter;

pub struct ConsoleExporter;

#[async_trait]
impl LogExporter for ConsoleExporter {
    async fn export(&self, log: &str) -> anyhow::Result<()> {
        println!("{}", log);
        Ok(())
    }
}
