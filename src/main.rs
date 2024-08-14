use actix_web::{App, HttpServer};

mod routes;
use routes::*;

mod database;
use database::*;

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = database_connection().await.expect("Database connection Failed");
    println!("Database connection established");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .service(home)
            .service(hello)
            .service(create_new_user)
    }).bind(("127.0.0.1", 8080))?
    .run();

    println!("Server running at 127.0.0.1:8080");
    server.await
}
