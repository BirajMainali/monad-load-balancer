use std::sync::Arc;
use crate::state::backend::Backend;

pub trait LoadBalancingAlgorithm: Send + Sync {
    fn select_backend(&self, eligible_candidates: &Vec<Arc<Backend>>) -> Option<usize>;
}
