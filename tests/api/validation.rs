use serde_json::json;
use crate::helpers::spawn_app;

#[tokio::test]
async fn validate_word_correct() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/validate", &app.address))
        .json(&json!({
            "word": "podex",
            "date": "2022-04-13T23:59:59Z"
        }))
        .send()
        .await
        .expect("Failed validating word");

    assert!(response.status().is_success());
    assert_eq!("correct", response.text().await.expect("No text found"))
}

#[tokio::test]
async fn validate_word_incorrect() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/validate", &app.address))
        .json(&json!({
            "word": "wrong",
            "date": "2022-04-13T23:59:59Z"
        }))
        .send()
        .await
        .expect("Failed validating word");

    assert!(response.status().is_success());
    assert_eq!("incorrect", response.text().await.expect("No text found"))
}

#[tokio::test]
async fn validate_word_some_correct() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/validate", &app.address))
        .json(&json!({
            "word": "pedal",
            "date": "2022-04-13T23:59:59Z"
        }))
        .send()
        .await
        .expect("Failed validating word");

    assert!(response.status().is_success());
    assert_eq!("some correct", response.text().await.expect("No text found"))
}