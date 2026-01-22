use crate::model::Config;

pub mod model;

#[tokio::main]
async fn main() {
    let config = Config::get_config().await.unwrap();
    println!("Config : {:?}", config);
}
