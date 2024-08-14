use actix_web::{http::StatusCode, post, web::Json, Responder};
use serde::Serialize;

use crate::routes::User;

#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    user: User,
}

#[post("/users")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    (
        Json(CreateUserResponse {
            id: 1,
            user: user.0
        }),
        StatusCode::CREATED,
    )
}