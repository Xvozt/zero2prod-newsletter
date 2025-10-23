use actix_web::{HttpResponse, get, web};

#[derive(serde::Deserialize)]
pub struct Parameters {
    #[allow(dead_code)]
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
#[get("/subscriptions/confirm")]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    // Get reference to the pool

    // Retrieve subscriber id by the token

    // Change subscriber status to confirmed
    HttpResponse::Ok().finish()
}
