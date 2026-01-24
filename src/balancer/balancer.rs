use crate::balancer::algorithm::LoadBalancingAlgorithm;
use crate::balancer::algorithms::traits::load_balancer_algorithm::LoadBalancingAlgorithm;
use crate::state::backend_state::Backend;
use crate::state::shared_state::SharedState;
use std::io;
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

    pub async fn resolve(&self, client: TcpStream) -> io::Result<()> {
        let row_guard = self.state.read().await;

        // step 1: filter non-negotiable candidates
        let candidates: Vec<&Backend> = row_guard
            .backends
            .iter()
            .filter(|b| b.active_conn.load(Ordering::Relaxed) < b.max_conn as u32)
            .filter(|b| b.current_weight > 0.0)
            .collect();

        // step 2: select backend using algorithm
        let idx = self
            .algorithm
            .select(&candidates)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no backend available"))?;

        let backend = candidates[idx];

        // IMPORTANT: release lock before async I/O
        drop(row_guard);

        // step 3: proxy traffic
        self.proxy(client, backend).await
    }

    async fn proxy(&self, client: TcpStream, backend_state: &Backend) -> io::Result<()> {
        backend_state.active_conn.fetch_add(1, Ordering::Relaxed);

        let addr = backend_state.addr.clone();
        let backend = TcpStream::connect(addr).await?;

        let (mut cr, mut cw) = split(client);
        let (mut br, mut bw) = split(backend);

        let client_to_backend = copy(&mut cr, &mut bw);
        let backend_to_client = copy(&mut br, &mut cw);

        try_join!(client_to_backend, backend_to_client)?;

        backend_state.active_conn.fetch_sub(1, Ordering::Relaxed);

        Ok(())
    }
}
