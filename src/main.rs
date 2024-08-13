use actix_web::{get, web::{self, Path}, App, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    let response = "Welcome to Rust!!!";

    response
}

#[get("/hello/{first_name}/{last_name}")]
async fn hello(params: Path<(String, String)>) -> impl Responder {
    let response = format!("Hello {} {}", params.0, params.1);

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(hello)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}
