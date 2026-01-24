use crate::balancer::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend_state::Backend;

pub struct LeastConn;

impl LeastConn {
    pub fn new() -> Self {
        Self
    }
}

impl LoadBalancingAlgorithm for LeastConn {
    fn select_backend(&self, backends: &[Backend]) -> Option<usize> {
        let index = backends
            .iter()
            .enumerate()
            .min_by_key(|(_, b)| b.active_connections())
            .map(|(i, _)| i);

        return index;
    }
}
