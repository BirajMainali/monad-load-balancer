use crate::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend::Backend;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::io::{copy, split};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::try_join;

#[derive(Clone)]
pub struct Balancer {
    active_backends: Arc<RwLock<Vec<Arc<Backend>>>>,
    algorithm: Arc<dyn LoadBalancingAlgorithm>,
}

impl Balancer {
    pub fn new(
        algorithm: Arc<dyn LoadBalancingAlgorithm>,
        active_backends: Arc<RwLock<Vec<Arc<Backend>>>>,
    ) -> Self {
        Self {
            algorithm,
            active_backends,
        }
    }

    pub async fn route_connection(&self, client: TcpStream) -> anyhow::Result<()> {
        let rg = self.active_backends.read().await;

        let candidates = rg
            .iter()
            .filter(|b| !b.has_no_wight() || !b.is_max_conn_reached())
            .map(|x| x.clone())
            .collect();

        let selected = match self.algorithm.select_backend(&candidates) {
            Some(i) => i,
            None => todo!("Implement waiting for certain time."),
        };

        let backend = candidates[selected].clone();
        backend.active_conn.fetch_add(1, Ordering::Relaxed);
        let result = self.perform_routing(client, backend.addr.clone()).await;
        backend.active_conn.fetch_sub(1, Ordering::Relaxed);

        result
    }

    async fn perform_routing(&self, client: TcpStream, backend_addr: String) -> anyhow::Result<()> {
        let backend = TcpStream::connect(backend_addr).await?;
        let (mut cr, mut cw) = split(client);
        let (mut br, mut bw) = split(backend);
        let client_to_backend = copy(&mut cr, &mut bw);
        let backend_to_client = copy(&mut br, &mut cw);
        try_join!(client_to_backend, backend_to_client)?;
        Ok(())
    }
}
