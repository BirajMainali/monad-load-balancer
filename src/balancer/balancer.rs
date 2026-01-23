use crate::state::shared_state::SharedState;
use serde_yaml::Value::String;
use std::io;
use std::sync::atomic::Ordering;
use tokio::io::{copy, split};
use tokio::net::TcpStream;
use tokio::try_join;

struct Balancer {
    state: SharedState,
}

impl Balancer {
    pub fn new(state: SharedState) -> Self {
        Balancer { state }
    }

    pub async fn resolve(&self, client: TcpStream) -> io::Result<()> {
        let row_guard = self.state.read().await;

        // step 1: filter out non-workable backends
        let candidates = row_guard
            .backends
            .iter()
            // 1.2 : max connection reached backend
            .filter(|b| b.active_conn.load(Ordering::Relaxed) < b.max_conn as u32)
            // 1.1 : apply non-notable filters such as wight
            .filter(|b| b.current_weight > 0.0)
            .map(|b| b.addr.clone())
            .collect();
        // step 3: identify the algorithm
        // step 4: fetch next backend addr using algorithm impl
        let addr = String.clone();
        self.proxy(client, addr).await?;
        Ok(())
    }

    async fn proxy(&self, client: TcpStream, addr: String) -> io::Result<()> {
        let backend = TcpStream::connect(addr).await?;

        let (mut cr, mut cw) = split(client);
        let (mut br, mut bw) = split(backend);

        let client_to_backend = copy(&mut cr, &mut bw);
        let backend_to_client = copy(&mut br, &mut cw);

        try_join!(client_to_backend, backend_to_client)?;

        Ok(())
    }
}
