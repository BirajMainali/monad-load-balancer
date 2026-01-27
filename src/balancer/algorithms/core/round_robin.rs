use std::sync::Arc;
use crate::balancer::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend_state::Backend;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    fn select_backend(&self, backends: &Vec<Arc<Backend>>) -> Option<usize> {
        if backends.is_empty() {
            return None;
        }

        let idx = self.cursor.fetch_add(1, Ordering::Relaxed);
        Some(idx % backends.len())
    }
}
