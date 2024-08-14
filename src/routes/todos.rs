use actix_web::{delete, get, post, put, web::{Data, Json, Path}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNewTodo {
    title: String,
    description: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExistingTodo {
    id: i32,
    title: String,
    description: Option<String>
}

#[derive(Serialize)]
pub struct TypeDBError {
    error: String,
}

#[post("/todos")]
pub async fn create_new_todo(db: Data<MySqlPool>, body: Json<CreateNewTodo>) -> impl Responder {
    let response = sqlx::query("insert into todos(title, description) values (?, ?)")
        .bind(&body.title)
        .bind(&body.description)
        .execute(&**db)
        .await;

    match response {
        Ok(id) => HttpResponse::Created().json(Todo {
            id: id.last_insert_id() as i32,
            title: body.title.clone(),
            description: body.description.clone(),
        }),
        Err(_e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: _e.to_string(),
        })
    }
}

#[get("/todos")]
pub async fn get_all_todo(db: Data<MySqlPool>) -> impl Responder {
    let res: Result<Vec<Todo>, sqlx::Error> = sqlx::query_as("select id, title, description from todos")
        .fetch_all(&**db)
        .await;

    match res {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: _e.to_string(),
        })
    }
}

#[put("/todos/{id}")]
pub async fn update_existing_todo(db: Data<MySqlPool>, params: Path<i32>, body: Json<UpdateExistingTodo>) -> impl Responder {
    let response = sqlx::query("update todos set title = ?, description = ? where id = ?")
        .bind(&body.title)
        .bind(&body.description)
        .bind(&params.clone())
        .execute(&**db)
        .await;

    match response {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo(db: Data<MySqlPool>, id: Path<i32>) -> impl Responder {
    let response = sqlx::query("delete from todos where id = ?")
        .bind(id.clone())
        .execute(&**db)
        .await;

    match response {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}