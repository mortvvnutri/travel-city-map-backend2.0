use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

#[openapi]
#[get("/")]
pub async fn list_categories() -> Json<Vec<Category>> {
    // Пример данных
    Json(vec![
        Category { id: 1, name: "Category 1".to_string(), parent_id: None },
        Category { id: 2, name: "Category 2".to_string(), parent_id: Some(1) },
    ])
}

#[openapi]
#[post("/")]
pub async fn create_category(category: Json<Category>) -> Json<Category> {
    // Здесь вы можете вставить категорию в базу данных
    category
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_categories, create_category]
}
