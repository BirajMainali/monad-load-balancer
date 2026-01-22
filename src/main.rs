mod config;

use config::model::Config;

#[tokio::main]
async fn main() {
    let config = Config::get_config().await.unwrap();
    println!("Config : {:?}", config);
}
