use actix_web::{get, Responder};

#[get("/")]
pub async fn home() -> impl Responder {
    let response = "Welcome to Rust!!!";

    response
}
