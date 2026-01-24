use crate::config::backend_cfg::BackendCfg;
use std::sync::atomic::AtomicU32;

pub struct Backend {
    pub id: String,
    pub addr: String,
    pub max_conn: i32,
    pub base_weight: f32,
    pub current_weight: f32,
    pub active_conn: AtomicU32,
    pub avg_latency: AtomicU32,
    pub error_rate: f32,
}

impl Backend {
    pub fn from_cfg(cfg: &BackendCfg) -> Self {
        Self {
            id: cfg.id.clone(),
            addr: cfg.address.clone(),
            max_conn: cfg.max_conn,
            base_weight: cfg.weight,
            current_weight: cfg.weight,
            active_conn: AtomicU32::new(0),
            avg_latency: AtomicU32::new(0),
            error_rate: 0.0,
        }
    }
}
