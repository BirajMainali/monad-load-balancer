mod config;

use config::loader::get_config;

#[tokio::main]
async fn main() {
    let config = get_config().await.unwrap();
    println!("Config : {:?}", config);
}
