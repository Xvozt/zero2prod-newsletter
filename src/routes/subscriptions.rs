use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use log;
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
    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.name,
        form.email,
    );
    let user_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Saving new subscriber to database", request_id
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
    ).execute(pool.get_ref()).await
    {
        Ok(_) => {
            log::info!("request_id {} - New sub '{}' was successfully saved with id: '{}'", request_id, form.name, user_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }

}