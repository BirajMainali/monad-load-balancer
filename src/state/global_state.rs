use crate::balancer::backend_state::Backend;
use crate::config::thresholds_cfg::ThresholdsCfg;
pub struct GlobalState {
    pub thresholds: ThresholdsCfg,
    pub backends: Vec<Backend>,
    pub balancer_cfg: crate::config::balancer_server_cfg::BalancerServerCfg,
}
