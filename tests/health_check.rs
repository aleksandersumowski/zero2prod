use std::net::TcpListener;
use zero2pr::startup::run;

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind");
    let port = listener.local_addr().expect("").port();
    let server = run(listener).expect("Failed to bind addres");
    let _ = tokio::spawn(server);
    port
}

#[tokio::test]
async fn dummy_test() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://127.0.0.1:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200() {
    let app_port = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("http://127.0.0.1:{app_port}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("No luck");
    assert_eq!(200, response.status().as_u16());

}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new(); let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("http://127.0.0.1:{}/subscriptions", &app_address)) .header("Content-Type", "application/x-www-form-urlencoded") .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
// Assert
        assert_eq!( 400,
                    response.status().as_u16(),
                    // Additional customised error message on test failure
                    "The API did not fail with 400 Bad Request when the payload was {}.",
                    error_message
        ); }
}
