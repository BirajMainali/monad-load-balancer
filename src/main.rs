mod algorithms;
mod config;
use tokio::net::TcpListener;

use config::loader::get_config;

#[tokio::main]
async fn main() {
    let config = get_config().await.unwrap();
    println!("Config : {:?}", config);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.balancer.port))
        .await
        .unwrap();

    let rr = algorithms::round_robin::RoundRobin::new(config.backends);
    println!("Load Balancer listening on {:?}", listener.local_addr());

    // use copy_bidirectional
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from {:?}", addr);

        tokio::spawn(async move {
            // Handle the connection
            println!("Handling connection from {:?}", addr);
            // Here you would typically forward the connection to a selected backend
        });
    }
}
