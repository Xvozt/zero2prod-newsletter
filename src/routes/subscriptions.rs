use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use tracing::{self, Instrument};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    let user_id = Uuid::new_v4();
    let query_span = tracing::info_span!(
        "Saving new subscriber in the database"
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4 )
        "#,
        user_id,
        form.email,
        form.name,
        Utc::now()
    ).execute(pool.get_ref())
        .instrument(query_span)
        .await
    {
        Ok(_) => {
            tracing::info!("request_id {} - New sub '{}' was successfully saved with id: '{}'", request_id, form.name, user_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }

}