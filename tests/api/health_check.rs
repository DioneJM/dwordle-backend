use crate::helpers::spawn_app;

#[tokio::test]
async fn heallth_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed health check");

    assert!(response.status().is_success());
    assert_eq!("dwordle - A - OK", response.text().await.expect("No text found"))
}