use crate::routes::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}
#[derive(thiserror::Error)]
pub enum ConfirmationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("There is no subscriber with the provided token")]
    UnknownToken,
}

impl Debug for ConfirmationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ConfirmationError {
    fn status_code(&self) -> StatusCode {
        match self {
            ConfirmationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ConfirmationError::UnknownToken => StatusCode::UNAUTHORIZED,
        }
    }
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters))]
#[get("/subscriptions/confirm")]
pub async fn confirm(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> Result<HttpResponse, ConfirmationError> {
    let subscriber_id = get_subscriber_id_by_token(&pool, &parameters.subscription_token)
        .await
        .context("Failed to retrive a subscriber with the provided token")?
        .ok_or(ConfirmationError::UnknownToken)?;
    confirm_subscriber(&pool, subscriber_id).await.context("Failed to update subscriber to 'confirmed' status")?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(token, pool))]
pub async fn get_subscriber_id_by_token(
    pool: &PgPool,
    token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        token,
    )
    .fetch_optional(pool)
    .await?;
    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' where id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
