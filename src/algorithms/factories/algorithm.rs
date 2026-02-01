use crate::algorithms::core::least_conn::LeastConn;
use crate::algorithms::core::round_robin::RoundRobin;
use crate::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::config::algorithm_cfg::AlgorithmType;
use std::sync::Arc;
use crate::algorithms::core::adaptive_least_conn::AdaptiveLeastConn;

pub struct Algorithm;

impl Algorithm {
    pub fn select(cfg: AlgorithmType) -> Arc<dyn LoadBalancingAlgorithm> {
        match cfg {
            AlgorithmType::RoundRobin => Arc::new(RoundRobin::new()),
            AlgorithmType::LeastConn => Arc::new(LeastConn::new()),
            AlgorithmType::AdaptiveLeastConn => Arc::new(AdaptiveLeastConn::new()),
            AlgorithmType::WeightedRoundRobin => todo!(),
        }
    }
}
