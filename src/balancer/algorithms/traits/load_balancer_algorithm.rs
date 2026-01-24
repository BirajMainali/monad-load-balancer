use crate::state::backend_state::Backend;

pub trait LoadBalancingAlgorithm: Send + Sync {
    fn select_backend(&self, backends: &Vec<&Backend>) -> Option<usize>;
}
