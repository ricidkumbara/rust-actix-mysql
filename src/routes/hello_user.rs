use actix_web::{get, http::StatusCode, web::{Json, Path}, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn new(first_name: String, last_name: String) -> Self {
        Self { 
            first_name, 
            last_name 
        }
    }    
}

#[get("/hello/{first_name}/{last_name}")]
pub async fn hello(params: Path<(String, String)>) -> impl Responder {
    let response = User::new(params.0.clone(), params.0.clone());

    (Json(response), StatusCode::OK)
}