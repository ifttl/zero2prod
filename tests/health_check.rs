//! tests/health_check.rs

// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    // TODO the address is still hardcoded
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    // trying to bind port 0 will trigger an OS scan for an available port
    let server = zero2prod::run("127.0.0.1:0").expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
