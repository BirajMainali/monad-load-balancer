use std::fmt;

#[derive(Debug)]
pub enum ExporterEvent {
    WeightDecreased {
        addr: String,
        old: u64,
        new: u64,
        latency_ms: u64,
    },
    WeightIncreased {
        addr: String,
        old: u64,
        new: u64,
        latency_ms: u64,
    },
    BackendDown {
        addr: String,
    },
    Error {
        err: String,
    },
}

impl fmt::Display for ExporterEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let log_message = match self {
            ExporterEvent::WeightDecreased {
                addr,
                old,
                new,
                latency_ms,
            } => {
                format!(
                    "level=info event=WeightDecreased addr={} old={} new={} latency_ms={}",
                    addr, old, new, latency_ms
                )
            }
            ExporterEvent::WeightIncreased {
                addr,
                old,
                new,
                latency_ms,
            } => {
                format!(
                    "level=info event=WeightIncreased addr={} old={} new={} latency_ms={}",
                    addr, old, new, latency_ms
                )
            }
            ExporterEvent::BackendDown { addr } => {
                format!("level=warn event=BackendDown addr={}", addr)
            }
            ExporterEvent::Error { err } => {
                format!("level=error event=Error err=\"{}\"", err)
            }
        };

        // Step 2: Write the log at the end
        write!(f, "{}", log_message)
    }
}
