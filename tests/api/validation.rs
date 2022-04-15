use serde_json::{json, Value};
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
    let body: Value = response.json().await.expect("Failed to parse json");
    assert_eq!(
        json!({
            "validation_result": "Correct",
            "date": "2022-04-13 00:00:00 UTC",
            "letters": [
                {
                    "value": "p",
                    "state": "Correct",
                    "position": 0
                },
                {
                    "value": "o",
                    "state": "Correct",
                    "position": 1
                },
                {
                    "value": "d",
                    "state": "Correct",
                    "position": 2
                },
                {
                    "value": "e",
                    "state": "Correct",
                    "position": 3
                },
                {
                    "value": "x",
                    "state": "Correct",
                    "position": 4
                },

            ]
        }),
        body
    )
}

#[tokio::test]
async fn validate_word_incorrect() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/validate", &app.address))
        .json(&json!({
            "word": "12345",
            "date": "2022-04-13T23:59:59Z"
        }))
        .send()
        .await
        .expect("Failed validating word");

    assert!(response.status().is_success());
    let body: Value = response.json().await.expect("Failed to parse json");
    assert_eq!(
        json!({
            "validation_result": "Incorrect",
            "date": "2022-04-13 00:00:00 UTC",
            "letters": [
                {
                    "value": "1",
                    "state": "NotPresent",
                    "position": 0
                },
                {
                    "value": "2",
                    "state": "NotPresent",
                    "position": 1
                },
                {
                    "value": "3",
                    "state": "NotPresent",
                    "position": 2
                },
                {
                    "value": "4",
                    "state": "NotPresent",
                    "position": 3
                },
                {
                    "value": "5",
                    "state": "NotPresent",
                    "position": 4
                },

            ]
        }),
        body
    )
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
    let body: Value = response.json().await.expect("Failed to parse json");
    assert_eq!(
        json!({
            "validation_result": "SomeCorrect",
            "date": "2022-04-13 00:00:00 UTC",
            "letters": [
                {
                    "value": "p",
                    "state": "Correct",
                    "position": 0
                },
                {
                    "value": "e",
                    "state": "Present",
                    "position": 1
                },
                {
                    "value": "d",
                    "state": "Correct",
                    "position": 2
                },
                {
                    "value": "a",
                    "state": "NotPresent",
                    "position": 3
                },
                {
                    "value": "l",
                    "state": "NotPresent",
                    "position": 4
                },

            ]
        }),
        body
    )
}