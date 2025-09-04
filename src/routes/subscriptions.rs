use actix_web::web::Data;
use actix_web::{HttpResponse, post, web};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{self};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

fn is_name_valid(s: &str) -> bool {
    let is_empty_or_only_whitespace = s.trim().is_empty();
    let forbidden_symbols = ['(', ')', '/', '\\', '"', '<', '>', '{', '}'];
    let is_too_long = s.graphemes(true).count() > 255;
    let contains_forbidden_characters = s.chars().any(|g| forbidden_symbols.contains(&g));

    !(is_empty_or_only_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
name = "Adding a subscriber",
skip(form, pool),
fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name
    )
)]
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_name_valid(&form.name) {
        return HttpResponse::BadRequest().finish();
    }
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &Data<PgPool>, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4 )
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
