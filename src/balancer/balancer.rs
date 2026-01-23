use crate::state::backend_state::Backend;
use crate::state::shared_state::SharedState;
use std::io;
use std::sync::atomic::Ordering;
use tokio::io::{copy, split};
use tokio::net::TcpStream;
use tokio::try_join;

#[derive(Clone)]
pub struct Balancer {
    state: SharedState,
}

impl Balancer {
    pub fn new(state: SharedState) -> Self {
        Balancer { state }
    }

    pub async fn resolve(&self, client: TcpStream) -> io::Result<()> {
        let row_guard = self.state.read().await;
        // step 1: filter non negotiable candidates
        let candidates: Vec<&Backend> = row_guard
            .backends
            .iter()
            // 1.2 : filter out address which is already occupied
            .filter(|b| b.active_conn.load(Ordering::Relaxed) < b.max_conn as u32)
            // 1.1 : filter out address which has no 0 width.
            .filter(|b| b.current_weight > 0.0)
            .collect();
        // step 3: identify the algorithm
        // step 4: fetch next backend addr using algorithm impl
        let backend: &Backend;
        self.proxy(client, backend).await?;
        Ok(())
    }

    async fn proxy(&self, client: TcpStream, backend_state: &Backend) -> io::Result<()> {
        backend_state.active_conn.fetch_add(1, Ordering::Relaxed);

        let addr = backend_state.addr.clone();
        let backend = TcpStream::connect(addr).await?;

        let (mut cr, mut cw) = split(client);
        let (mut br, mut bw) = split(backend);
`
        let client_to_backend = copy(&mut cr, &mut bw);
        let backend_to_client = copy(&mut br, &mut cw);

        try_join!(client_to_backend, backend_to_client)?;

        backend_state.active_conn.fetch_sub(1, Ordering::Relaxed);
        Ok(())
    }
}
