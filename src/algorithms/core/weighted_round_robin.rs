use crate::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend::Backend;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct WeightedRoundRobin {
    cursor: AtomicUsize,
}

impl WeightedRoundRobin {
    pub fn new() -> Self {
        Self {
            cursor: AtomicUsize::new(0),
        }
    }
}

impl LoadBalancingAlgorithm for WeightedRoundRobin {
    fn select_backend(&self, eligible_candidates: &Vec<Arc<Backend>>) -> Option<usize> {
        if eligible_candidates.is_empty() {
            return None;
        }

        let total_weight: u64 = eligible_candidates
            .iter()
            .map(|backend| backend.current_weight.load(Ordering::Relaxed))
            .sum();

        if total_weight == 0 {
            // Fallback to round-robin if all weights are 0
            let idx = self.cursor.fetch_add(1, Ordering::Relaxed);
            return Some(idx % eligible_candidates.len());
        }

        let idx = self.cursor.fetch_add(1, Ordering::Relaxed);
        let mut current_weight = (idx as u64) % total_weight;

        for (i, backend) in eligible_candidates.iter().enumerate() {
            let backend_weight = backend.current_weight.load(Ordering::Relaxed);
            if backend_weight == 0 {
                continue;
            }

            if current_weight < backend_weight {
                return Some(i);
            }
            current_weight -= backend_weight;
        }

        // This shouldn't happen in normal cases, but provide a fallback
        Some(0)
    }
}
