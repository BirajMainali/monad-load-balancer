use crate::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend::Backend;
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub struct AdaptiveLeastConn;

impl AdaptiveLeastConn {
    pub fn new() -> Self {
        Self
    }
}

impl LoadBalancingAlgorithm for AdaptiveLeastConn {
    fn select_backend(&self, eligible_candidates: &Vec<Arc<Backend>>) -> Option<usize> {
        eligible_candidates
            .iter()
            .enumerate()
            .min_by_key(|(_, backend)| {
                let active_conn = backend.active_conn.load(Ordering::Relaxed);
                let avg_latency = backend.avg_latency_ms.load(Ordering::Relaxed);

                // Adaptive scoring: combine connection count and latency
                // Lower score is better. We weight connections more heavily than latency.
                // Formula: active_conn * 1000 + avg_latency
                // This ensures that a backend with fewer connections is preferred even with slightly higher latency
                (active_conn as usize) * 1000 + avg_latency
            })
            .map(|(idx, _)| idx)
    }
}
