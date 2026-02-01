use crate::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend::Backend;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct LeastConn;

impl LeastConn {
    pub fn new() -> Self {
        Self
    }
}

impl LoadBalancingAlgorithm for LeastConn {
    fn select_backend(&self, eligible_candidates: &Vec<Arc<Backend>>) -> Option<usize> {
        eligible_candidates
            .iter()
            .enumerate()
            .min_by_key(|(_, backend)| backend.active_conn.load(Ordering::Relaxed))
            .map(|(idx, _)| idx)
    }
}
