use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_data_form() {
    // Arange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();

    // Act
    let body = "name=Von%20Hux&email=hux%40gmail.com";
    let response = client
        .post(format!("{app_address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Hux", "missing email"),
        ("email=hux%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{app_address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {error_message}"
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}