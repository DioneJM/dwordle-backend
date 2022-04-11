use actix_web::{HttpResponse, web};
use actix_web::error::Error;
use sqlx::PgPool;
use crate::queries::word::get_word_for_date;

#[derive(serde::Deserialize)]
pub struct RequestData {
    word: String
}

pub async fn validate_word(
    request: web::Json<RequestData>,
    db_connection: web::Data<PgPool>
) -> Result<HttpResponse, Error> {
    let word = request.0.word.to_lowercase();
    let date_index = 1;
    let word_to_guess = get_word_for_date(date_index, &db_connection).await.expect("failed to query word to guess");
    println!("word to guess: {}", word_to_guess);
    Ok(HttpResponse::Ok().body(format!("word to validate: {}", word)))
}