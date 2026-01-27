use std::sync::Arc;
use crate::balancer::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend_state::Backend;
use std::sync::atomic::Ordering;

pub struct LeastConn;

impl LeastConn {
    pub fn new() -> Self {
        Self
    }
}

impl LoadBalancingAlgorithm for LeastConn {
    fn select_backend(&self, backends: &Vec<Arc<Backend>>) -> Option<usize> {
        backends
            .iter()
            .enumerate()
            .min_by_key(|(_, backend)| backend.active_conn.load(Ordering::Relaxed))
            .map(|(idx, _)| idx)
    }
}
