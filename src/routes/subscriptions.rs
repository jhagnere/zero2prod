use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(_form: web::Json<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
