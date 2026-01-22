use monad_load_balancer::algorithms::round_robin::RoundRobin;
use monad_load_balancer::config::model::Backend;
#[tokio::test]
async fn round_robin_test() {
    let backends: Vec<Backend> = vec![
        Backend {
            id: "1".to_string(),
            address: "10.0.0.1:8080".to_string(),
            max_connections: 100,
            weight: 1.0,
        },
        Backend {
            id: "2".to_string(),
            address: "10.0.0.2:8080".to_string(),
            max_connections: 150,
            weight: 2.0,
        },
        Backend {
            id: "3".to_string(),
            address: "10.0.0.3:8080".to_string(),
            max_connections: 200,
            weight: 3.0,
        },
    ];

    let mut rr = RoundRobin::new(backends);
    for i in 0..5 {
        let backend = rr.get_next_backend().unwrap();
        let next_index = ((i % 3) + 1).to_string();
        assert_eq!(backend.id, next_index);
    }
}
