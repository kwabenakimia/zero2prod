//! lib.rs
pub mod configuration;
pub mod routes;
pub mod startup;

// use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
// use std::net::TcpListener;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// create a request handler returning an ok response
// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// #[derive(serde::Deserialize)]
// struct FormData {
//     email: String,
//     name: String,
// }

// Let's start simple: we always return a 200 OK
// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }
