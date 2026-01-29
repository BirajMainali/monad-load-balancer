use monad_load_balancer::algorithms::factories::algorithm::Algorithm;
use monad_load_balancer::balancer::balancer::Balancer;
use monad_load_balancer::config::load_balancer_cfg::LoadBalancerCfg;
use monad_load_balancer::health::health::{Health, HealthEvent};
use monad_load_balancer::state::backend::Backend;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{RwLock, mpsc};

#[tokio::main]
async fn main() {
    let (health_tx, mut health_rx): (Sender<HealthEvent>, Receiver<HealthEvent>) =
        mpsc::channel(8 * 1024);

    let cfg = LoadBalancerCfg::load()
        .await
        .expect("load balancer cfg failed");

    let server_port = format!("0.0.0.0:{}", cfg.balancer_cfg.port);

    let backends: Arc<RwLock<Vec<Arc<Backend>>>> = {
        let backend = cfg
            .backends
            .iter()
            .map(|b| Arc::new(Backend::from_cfg(b)))
            .collect();
        Arc::new(RwLock::new(backend))
    };

    {
        let config = cfg.clone();
        let health_backends = backends.clone();
        let health_tx = health_tx.clone();

        tokio::spawn(async move {
            let health = Health::new(
                config.thresholds_cfg,
                config.balancer_cfg,
                health_backends,
                health_tx,
            );

            health.watchdog().await.unwrap();
        });
    }

    let algorithm = Algorithm::select(cfg.balancer_cfg.algorithm);
    let balancer = Balancer::new(algorithm, backends);
    let health_tx = health_tx.clone();

    let listener = TcpListener::bind(&server_port)
        .await
        .expect("failed to bind server port, Please make sure address is available");

    loop {
        let (client, client_addr) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", client_addr);

        let balancer = balancer.clone();
        let health_tx = health_tx.clone();

        tokio::spawn(async move {
            if let Err(e) = balancer.route_connection(client).await {
                health_tx
                    .send(HealthEvent::Error {
                        err: format!("Failed to route connection: {}", e),
                    })
                    .await
                    .unwrap();
            }
        });
    }
}
