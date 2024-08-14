use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;

mod routes;
use routes::*;

mod database;
use database::*;

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database = database_connection().await.expect("Database connection Failed");
    println!("Database connection established");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(home)
            .service(hello)
            .service(create_new_user)
            .service(create_new_todo)
            .service(get_all_todo)
            .service(update_existing_todo)
            .service(delete_todo)
    }).bind(("127.0.0.1", std::env::var("APP_PORT").unwrap().parse::<u16>().unwrap()))?
    .run();

    println!("Server running at 127.0.0.1:{}", std::env::var("APP_PORT").expect("App Port must be set"));
    server.await
}
