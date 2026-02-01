use crate::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend::Backend;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct RoundRobin {
    cursor: AtomicUsize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            cursor: AtomicUsize::new(0),
        }
    }
}

impl LoadBalancingAlgorithm for RoundRobin {
    fn select_backend(&self, eligible_candidates: &Vec<Arc<Backend>>) -> Option<usize> {
        if eligible_candidates.is_empty() {
            return None;
        }

        let idx = self.cursor.fetch_add(1, Ordering::Relaxed);
        Some(idx % eligible_candidates.len())
    }
}
