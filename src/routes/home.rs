use actix_web::{get, Responder};

#[get("/")]
pub async fn home() -> impl Responder {
    println!("{}", std::env::var("APP_PORT").unwrap().parse::<u16>().unwrap());
    let response = "Welcome to Rust!!!";

    response
}
