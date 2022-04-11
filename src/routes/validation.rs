use actix_web::{HttpResponse, web};
use actix_web::error::Error;

#[derive(serde::Deserialize)]
pub struct RequestData {
    word: String
}

pub async fn validate_word(
    request: web::Json<RequestData>
) -> Result<HttpResponse, Error> {
    let word = request.0.word.to_lowercase();
    Ok(HttpResponse::Ok().body(format!("word to validate: {}", word)))
}