use std::net::TcpListener;

use zero2prod::get_random_port;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let (listener, port) = get_random_port();
    let server = zero2prod::run(listener).expect("Failed to spawn :(");

    let _ = tokio::spawn(server);

    format!("http://0.0.0.0:{port}")
}
