use actix_web::{Responder, web};
use actix_web::error::Error;
use chrono::{DateTime, TimeZone, Utc, Duration, Timelike};
use sqlx::PgPool;
use crate::queries::word::get_word_for_date;

#[derive(serde::Deserialize)]
pub struct RequestData {
    pub word: String,
    pub date: String,
}

#[derive(serde::Serialize)]
pub struct ResponseData {
    validation_result: ValidationResult,
    date: String,
    letters: Vec<Letter>
}

#[derive(serde::Serialize)]
pub enum ValidationResult {
    Correct,
    SomeCorrect,
    Incorrect
}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum LetterState {
    Correct,
    Present,
    NotPresent,
}

#[derive(serde::Serialize)]
pub struct Letter {
    value: char,
    state: LetterState,
    position: usize,
}

pub async fn validate_word(
    request: web::Json<RequestData>,
    db_connection: web::Data<PgPool>,
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
        &db_connection,
    )
        .await;

    let result = match word.trim().eq(word_to_guess.as_str()) {
        true => ValidationResult::Correct,
        false => {
            let shared_letters: String = word
                .chars()
                .filter(|char| word_to_guess.contains(&char.to_string()))
                .collect();
            let some_correct = shared_letters.len() > 0;
            println!("shared letters: {}", shared_letters);
            if some_correct {
                ValidationResult::SomeCorrect
            } else {
                ValidationResult::Incorrect
            }
        }
    };

    let letters: Vec<Letter> = word
        .chars()
        .enumerate()
        .map(|(index, char)| {
            Letter {
                value: char,
                state: get_letter_state(index, char, word_to_guess.clone()),
                position: index,
            }
        }).collect();

    Ok(web::Json(ResponseData {
        validation_result: result,
        date: date.to_string(),
        letters
    }))
}

fn get_letter_state(index: usize, letter: char, word: String) -> LetterState {
    match word.chars().nth(index).unwrap().eq(&letter) {
        true => LetterState::Correct,
        false => {
            let is_present = word.contains(letter);

            if is_present {
                LetterState::Present
            } else {
                LetterState::NotPresent
            }
        }
    }
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
        &db_connection,
    )
        .await
        .expect("failed to query word to guess")
        .trim()
        .to_string();

    word_to_guess
}