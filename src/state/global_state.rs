use crate::config::balancer_server_cfg::BalancerServerCfg;
use crate::config::thresholds_cfg::ThresholdsCfg;
use crate::state::backend_state::Backend;

pub struct GlobalState {
    pub thresholds_cfg: ThresholdsCfg,
    pub backends: Vec<Backend>,
    pub balancer_cfg: BalancerServerCfg,
}
