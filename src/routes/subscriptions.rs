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
    log::info!("Saving a new subscriber details for subscriber with email the database");
    let user_id = Uuid::new_v4();
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
            log::info!("New subscriber details for subscriber with email {:?} has been saved", user_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}