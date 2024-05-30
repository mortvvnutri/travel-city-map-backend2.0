// routers/users.rs
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[openapi]
#[get("/")]
pub async fn list_users() -> Json<Vec<User>> {
    // Пример данных
    Json(vec![
        User { id: 1, username: "user1".to_string(), email: "user1@example.com".to_string() },
        User { id: 2, username: "user2".to_string(), email: "user2@example.com".to_string() },
    ])
}

#[openapi]
#[post("/")]
pub async fn create_user(user: Json<User>) -> Json<User> {
    // Здесь вы можете вставить пользователя в базу данных
    user
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_users, create_user]
}
