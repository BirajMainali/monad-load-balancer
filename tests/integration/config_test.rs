use monad_load_balancer::config::loader::get_config;
#[tokio::test]
async fn test_load_config() {
    let config = get_config().await;
    assert!(config.is_ok());
}
