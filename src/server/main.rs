use monad_load_balancer::balancer::balancer::Balancer;
use monad_load_balancer::config::load_balancer_cfg::LoadBalancerCfg;
use monad_load_balancer::state::backend_state::Backend;
use monad_load_balancer::state::global_state::GlobalState;
use monad_load_balancer::state::shared_state::SharedState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let cfg = LoadBalancerCfg::load().await.unwrap();
    let server_port = format!("0.0.0.0:{}", cfg.balancer_cfg.port);

    let state: SharedState = Arc::new(RwLock::new(GlobalState {
        thresholds_cfg: cfg.thresholds_cfg,
        balancer_cfg: cfg.balancer_cfg,
        backends: cfg
            .backends
            .into_iter()
            .map(|b| Backend::from_cfg(&b))
            .collect(),
    }));

    let listener = TcpListener::bind(server_port).await.unwrap();
    println!("Listening on {}", server_port);

    let balancer = Balancer::new(state);
    loop {
        let (client, client_addr) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", client_addr);

        let balancer = balancer.clone();
        tokio::spawn(async move {
            if let Err(e) = balancer.resolve(client).await {
                eprintln!("Error proxying connection: {:?}", e);
            }
        });
    }
}
