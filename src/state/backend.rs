use crate::config::backend_cfg::BackendCfg;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Backend {
    pub id: String,
    pub addr: String,
    pub max_conn: u64,
    pub base_weight: u64,
    pub current_weight: AtomicU64,
    pub active_conn: AtomicU64,
    pub avg_latency_ms: AtomicUsize,
}

impl Backend {
    pub fn from_cfg(cfg: &BackendCfg) -> Self {
        Self {
            id: cfg.id.clone(),
            addr: cfg.address.clone(),
            max_conn: cfg.max_conn,
            base_weight: cfg.weight,
            current_weight: AtomicU64::new(cfg.weight),
            avg_latency_ms: AtomicUsize::new(0),
            active_conn: AtomicU64::new(0),
        }
    }

    pub fn exceeds_latency_threshold(
        &self,
        curr_latency: u64,
        critical_latency_threshold: u64,
    ) -> bool {
        curr_latency >= critical_latency_threshold
    }

    pub fn is_weight_low(&self, current_weight: u64) -> bool {
        current_weight <= self.base_weight
    }

    pub fn is_currently_booting(
        &self,
        critical_latency_threshold: u64,
        curr_latency: u64,
        current_weight: u64,
    ) -> bool {
        self.exceeds_latency_threshold(curr_latency, critical_latency_threshold)
            && self.is_weight_low(current_weight)
    }

    pub fn has_no_wight(&self) -> bool {
        self.current_weight.load(Ordering::Relaxed) <= 0
    }

    pub fn has_some_wight(&self) -> bool {
        self.current_weight.load(Ordering::Relaxed) > 0
    }

    pub fn is_max_conn_reached(&self) -> bool {
        self.active_conn.load(Ordering::Relaxed) > self.max_conn
    }
}
