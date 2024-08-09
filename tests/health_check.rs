use std::net::{TcpListener, TcpStream};

// spins up an instance of our application and return its address
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _spawn = tokio::spawn(server);
    //let _ = spawn;
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    // Arrange Request
    let client = reqwest::Client::new();

    // Act, on the request and await the response
    let response = client
        .get(format!("{}/health_check", addr))
        // .get("http://127.0.0.1:0/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange a Client
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Client request and response
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    // Arrange
    let app_addr = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the name"),
        ("email=ursula_le_guin%40gmail.com", "missing the email"),
        ("", "missing both name and email"),
    ];

    // Act
    for (invalid_body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with Bad Request when the payload was {}",
            error_msg
        );
    }
}
