use monad_load_balancer::balancer::algorithms::factories::algorithm_factory::Algorithm;
use monad_load_balancer::balancer::balancer::Balancer;
use monad_load_balancer::config::load_balancer_cfg::LoadBalancerCfg;
use monad_load_balancer::health::health::Health;
use monad_load_balancer::state::backend::Backend;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let cfg = LoadBalancerCfg::load()
        .await
        .expect("load balancer cfg failed");
    dbg!(&cfg);

    let server_port = format!("0.0.0.0:{}", cfg.balancer_cfg.port);

    let backends: Arc<RwLock<Vec<Arc<Backend>>>> = {
        let backend = cfg
            .backends
            .iter()
            .map(|b| Arc::new(Backend::from_cfg(b)))
            .collect();
        Arc::new(RwLock::new(backend))
    };

    let config = cfg.clone();
    let health_backends = backends.clone();

    tokio::spawn(async move {
        let health = Health::new(config.thresholds_cfg, config.balancer_cfg, health_backends);
        health.observe_and_tune_backends().await.unwrap();
    });

    let algorithm = Algorithm::select(cfg.balancer_cfg.algorithm);
    let balancer = Balancer::new(algorithm, backends);

    let listener = TcpListener::bind(&server_port)
        .await
        .expect("failed to bind server port, Please make sure address is available");

    loop {
        let (client, client_addr) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", client_addr);

        let balancer = balancer.clone();
        tokio::spawn(async move {
            if let Err(e) = balancer.route_connection(client).await {
                eprintln!("Error proxying connection: {:?}", e);
            }
        });
    }
}
