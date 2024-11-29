//! src/routes/health_check.rs
use actix_web::HttpResponse;

// create a request handler returning an ok response
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
