use sqlx::PgPool;

pub async fn get_word_for_date(
    date_index: i32,
    db_connection: &PgPool
) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT word
        FROM words
        WHERE id = $1;
        "#,
        date_index
    ).fetch_one(db_connection)
        .await
        .expect("word not found");
    Ok(row.word)
}