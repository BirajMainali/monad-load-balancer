use crate::config::balancer_server_cfg::BalancerServerCfg;
use crate::config::thresholds_cfg::ThresholdsCfg;
use crate::logging::events::exporter_event::ExporterEvent;
use crate::state::backend::Backend;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use tokio::time::{Instant, sleep, timeout};

pub struct Health {
    threshold: ThresholdsCfg,
    balancer: BalancerServerCfg,
    backends: Arc<RwLock<Vec<Arc<Backend>>>>,
    health_tx: Sender<ExporterEvent>,
}
impl Health {
    pub fn new(
        threshold: ThresholdsCfg,
        balancer: BalancerServerCfg,
        backends: Arc<RwLock<Vec<Arc<Backend>>>>,
        health_tx: Sender<ExporterEvent>,
    ) -> Self {
        Self {
            threshold,
            balancer,
            backends,
            health_tx,
        }
    }
    pub async fn monitor(&self) -> anyhow::Result<()> {
        loop {
            let backends = {
                let rg = self.backends.read().await;
                rg.clone()
            };
            for backend in backends {
                let addr = backend.addr.clone();
                let curr_weight = backend.current_weight.load(Ordering::Relaxed);

                let result =
                    Self::get_current_backend_latency(&addr, self.threshold.latency_critical_ms)
                        .await;

                match result {
                    Some(latency) => {
                        // Update average latency with exponential moving average
                        let current_avg = backend.avg_latency_ms.load(Ordering::Relaxed);
                        let new_avg = if current_avg == 0 {
                            latency as usize
                        } else {
                            // EMA with alpha = 0.3 (30% weight to new measurement)
                            (current_avg * 7 + latency as usize * 3) / 10
                        };
                        backend.avg_latency_ms.store(new_avg, Ordering::Relaxed);

                        match backend
                            .exceeds_latency_threshold(latency, self.threshold.latency_critical_ms)
                        {
                            // If the threshold exceeds then reducing the wight gracefully.
                            // In consistent critical latency,It the wight become 0 then it will be stopped routing.
                            true => {
                                let new_wight =
                                    std::cmp::max(0, curr_weight - self.threshold.recovery_step);
                                backend.current_weight.swap(new_wight, Ordering::Relaxed);

                                self.health_tx
                                    .send(ExporterEvent::WeightDecreased {
                                        addr: addr.clone(),
                                        old: curr_weight,
                                        new: new_wight,
                                        latency_ms: latency,
                                    })
                                    .await?;
                            }
                            // If the latency is good (not exceeded) and the wight is low which shows booting or adjusting.
                            // so that setting wight incrementally
                            false => {
                                if backend.is_weight_low(curr_weight) {
                                    let new_weight = std::cmp::max(
                                        backend.base_weight,
                                        curr_weight + self.threshold.recovery_step,
                                    );
                                    backend.current_weight.swap(new_weight, Ordering::Relaxed);

                                    self.health_tx
                                        .send(ExporterEvent::WeightIncreased {
                                            addr: addr.clone(),
                                            old: curr_weight,
                                            new: new_weight,
                                            latency_ms: latency,
                                        })
                                        .await?;
                                }
                            }
                        }
                    }
                    None => {
                        // If any error occurred while performing ping then setting the wight = 0.
                        // Which means, applying circuit breaker unless it warm up.
                        backend.current_weight.swap(0, Ordering::Relaxed);

                        self.health_tx
                            .send(ExporterEvent::BackendDown { addr: addr.clone() })
                            .await?;
                    }
                }
            }
            sleep(Duration::from_millis(self.balancer.check_interval_ms)).await;
        }
    }
    async fn get_current_backend_latency(addr: &str, timeout_threshold: u64) -> Option<u64> {
        let timeout_dur = Duration::from_millis(timeout_threshold);
        let start = Instant::now();

        let connect_result = timeout(timeout_dur, TcpStream::connect(addr)).await;

        match connect_result {
            Ok(Ok(_stream)) => Some(start.elapsed().as_millis() as u64),
            Ok(Err(_)) => None,
            Err(_) => None,
        }
    }
}
