use monad_load_balancer::balancer::algorithms::factories::algorithm_factory::AlgorithmFactory;
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
    let cfg = LoadBalancerCfg::load()
        .await
        .expect("load balancer cfg failed");
    dbg!(&cfg);

    let server_port = format!("127.0.0.1:{}", cfg.balancer_cfg.port);

    let state: SharedState = Arc::new(RwLock::new(GlobalState {
        thresholds_cfg: cfg.thresholds_cfg.clone(),
        balancer_cfg: cfg.balancer_cfg.clone(),
        backends: cfg
            .backends
            .into_iter()
            .map(|b| Backend::from_cfg(&b))
            .collect(),
    }));

    let algorithm = AlgorithmFactory::select_algorithm(&cfg.balancer_cfg.algorithm);
    let balancer = Balancer::new(state, algorithm);

    let listener = TcpListener::bind(&server_port)
        .await
        .expect("failed to bind server port, Please make sure address is available");

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
