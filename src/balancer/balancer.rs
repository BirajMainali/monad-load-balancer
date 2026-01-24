use crate::balancer::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend_state::Backend;
use crate::state::shared_state::SharedState;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::io::{copy, split};
use tokio::net::TcpStream;
use tokio::try_join;

#[derive(Clone)]
pub struct Balancer {
    state: SharedState,
    algorithm: Arc<dyn LoadBalancingAlgorithm>,
}

impl Balancer {
    pub fn new(state: SharedState, algorithm: Arc<dyn LoadBalancingAlgorithm>) -> Self {
        Self { state, algorithm }
    }

    pub async fn resolve(&self, client: TcpStream) -> anyhow::Result<()> {
        let row_guard = self.state.read().await;

        // step 1: filter non-negotiable candidates
        let candidates: Vec<&Backend> = row_guard
            .backends
            .iter()
            .filter(|b| b.active_conn.load(Ordering::Relaxed) < b.max_conn as u32)
            .filter(|b| b.current_weight > 0.0)
            .map(|b| b)
            .collect();

        let index = self.algorithm.select_backend(&candidates);

        match index {
            Some(idx) => {
                let backend = candidates[idx];
                backend.active_conn.fetch_add(1, Ordering::Relaxed);
                self.proxy(client, backend.addr.clone()).await?;
                backend.active_conn.fetch_sub(1, Ordering::Relaxed);
            }
            _ => {
                todo!("Need to implement certain pooling or wait")
            }
        }
        Ok(())
    }

    async fn proxy(&self, client: TcpStream, backend_addr: String) -> anyhow::Result<()> {
        let backend = TcpStream::connect(backend_addr).await?;
        let (mut cr, mut cw) = split(client);
        let (mut br, mut bw) = split(backend);
        let client_to_backend = copy(&mut cr, &mut bw);
        let backend_to_client = copy(&mut br, &mut cw);
        try_join!(client_to_backend, backend_to_client)?;
        Ok(())
    }
}
