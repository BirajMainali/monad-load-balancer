use crate::logging::traits::log_exporter::LogExporter;
use async_trait::async_trait;
use std::path::PathBuf;

pub struct FileExporter {
    log_file: PathBuf,
}

impl FileExporter {
    pub fn new(log_file: impl Into<PathBuf>) -> Self {
        Self {
            log_file: log_file.into(),
        }
    }
}

#[async_trait]
impl LogExporter for FileExporter {
    async fn export(&self, log: &str) -> anyhow::Result<()> {
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
            .await?;
        tokio::io::AsyncWriteExt::write_all(&mut file, log.as_bytes()).await?;
        Ok(())
    }
}
