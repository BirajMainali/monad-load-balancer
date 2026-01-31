use crate::logging::events::exporter_event::ExporterEvent;
use crate::logging::traits::log_exporter::LogExporter;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::Receiver;

pub struct Exporter<E: LogExporter> {
    exporter: E,
}

impl<E: LogExporter> Exporter<E> {
    pub fn new(exporter: E) -> Self {
        Self { exporter }
    }

    pub async fn run(&self, mut export_rx: Receiver<ExporterEvent>) -> anyhow::Result<()> {
        while let Some(event) = export_rx.recv().await {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
            let log_line = format!("timestamp={:?} {}\n", timestamp, event);
            self.exporter.export(&log_line).await?;
        }
        Ok(())
    }
}
