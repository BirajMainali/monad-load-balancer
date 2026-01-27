use crate::balancer::algorithms::core::least_conn::LeastConn;
use crate::balancer::algorithms::core::round_robin::RoundRobin;
use crate::balancer::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::config::algorithm_cfg::AlgorithmType;
use std::sync::Arc;

pub struct Algorithm;

impl Algorithm {
    pub fn select(cfg: AlgorithmType) -> Arc<dyn LoadBalancingAlgorithm> {
        match cfg {
            AlgorithmType::RoundRobin => Arc::new(RoundRobin::new()),
            AlgorithmType::LeastConn => Arc::new(LeastConn::new()),
            AlgorithmType::AdaptiveLeastConn => todo!(),
            AlgorithmType::WeightedRoundRobin => todo!(),
        }
    }
}
