// routers/places.rs
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub latitude: f64,
    pub longitude: f64,
    pub puskinskaya_card: bool,
}

#[openapi]
#[get("/")]
pub async fn list_places() -> Json<Vec<Place>> {
    // Пример данных
    Json(vec![
        Place { id: 1, name: "Place 1".to_string(), description: Some("Description 1".to_string()), category_id: Some(1), latitude: 55.7558, longitude: 37.6176, puskinskaya_card: false },
        Place { id: 2, name: "Place 2".to_string(), description: Some("Description 2".to_string()), category_id: Some(2), latitude: 48.8566, longitude: 2.3522, puskinskaya_card: true },
    ])
}

#[openapi]
#[post("/")]
pub async fn create_place(place: Json<Place>) -> Json<Place> {
    // Здесь вы можете вставить место в базу данных
    place
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_places, create_place]
}
