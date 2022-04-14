use actix_web::{Responder, web};
use actix_web::error::Error;
use chrono::{DateTime, TimeZone, Utc, Duration, Timelike};
use sqlx::PgPool;
use crate::queries::word::get_word_for_date;

#[derive(serde::Deserialize)]
pub struct RequestData {
    pub word: String,
    pub date: String
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct ResponseData {
    validation_result: ValidationResult,
    date: String
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum ValidationResult {
    Correct,
    SomeCorrect,
    Incorrect
}

pub async fn validate_word(
    request: web::Json<RequestData>,
    db_connection: web::Data<PgPool>
) -> Result<impl Responder, Error> {
    let word = request.0.word.to_lowercase();
    let parsed_date = DateTime::parse_from_rfc3339(request.0.date.as_str())
        .expect("Failed to parse date")
        .with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap();

    let date = Utc.timestamp(parsed_date.timestamp(), 0);
    let word_to_guess = get_word_to_guess_for(
        date,
        &db_connection
    )
        .await;

    let result = match word.trim().eq(word_to_guess.as_str()) {
        true => ValidationResult::Correct,
        false => {
            let some_correct = word.chars().any(|char| word_to_guess.contains(char));
            if some_correct {
                ValidationResult::SomeCorrect
            } else {
                ValidationResult::Incorrect
            }
        }
    };

    Ok(web::Json(ResponseData {
        validation_result: result,
        date: date.to_string()
    }))
}

pub fn get_duration_since_epoch_date(reference_date: DateTime<Utc>) -> Duration {
    let epoch_date = DateTime::parse_from_rfc3339("2022-04-11T00:00:00Z")
        .expect("Failed to parse epoch date");
    let epoch_date = Utc.timestamp(epoch_date.timestamp(), 0);

    reference_date.signed_duration_since(epoch_date)
}

pub async fn get_word_to_guess_for(date: DateTime<Utc>, db_connection: &PgPool) -> String {
    let time_since_epoch = get_duration_since_epoch_date(date);
    let date_index = time_since_epoch.num_days();

    let word_to_guess = get_word_for_date(
        date_index
            .try_into()
            .expect("Failed to convert date_index"),
        &db_connection
    )
        .await
        .expect("failed to query word to guess")
        .trim()
        .to_string();

    word_to_guess
}