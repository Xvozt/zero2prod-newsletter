use actix_web::{HttpResponse, get, web};

#[derive(serde::Deserialize)]
pub struct Parameters {
    #[allow(dead_code)]
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm subscriber", skip(_parameters))]
#[get("/subscriptions/confirm")]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
