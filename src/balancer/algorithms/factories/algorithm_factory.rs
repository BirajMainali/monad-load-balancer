use crate::balancer::algorithms::core::least_conn::LeastConn;
use crate::balancer::algorithms::core::round_robin::RoundRobin;
use crate::balancer::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::config::algorithm_cfg::AlgorithmCfg;
use std::sync::Arc;

pub struct AlgorithmFactory;

impl AlgorithmFactory {
    pub fn select_algorithm(cfg: &AlgorithmCfg) -> Arc<dyn LoadBalancingAlgorithm> {
        match cfg {
            AlgorithmCfg::RoundRobin => Arc::new(RoundRobin::new()),
            AlgorithmCfg::LeastConn => Arc::new(LeastConn::new()),
            AlgorithmCfg::AdaptiveLeastConn => todo!(),
            AlgorithmCfg::WeightedRoundRobin => todo!(),
        }
    }
}
