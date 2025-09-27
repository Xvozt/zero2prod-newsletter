use actix_web::web::{Data, Form};
use actix_web::{HttpResponse, post};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{self};
use uuid::Uuid;

use crate::domain::NewSubscriber;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
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
pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &Data<PgPool>,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now(),
        "confirmed",
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
