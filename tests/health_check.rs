//! tests/health_check.rs

// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to swpawn out app.");

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<(), std::io::Error> {
    zero2prod::run().await
}