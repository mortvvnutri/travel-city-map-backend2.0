// routers/reviews.rs
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Review {
    pub id: i32,
    pub user_id: i32,
    pub place_id: i32,
    pub rating: i32,
    pub comment: Option<String>,
}

#[openapi]
#[get("/")]
pub async fn list_reviews() -> Json<Vec<Review>> {
    // Пример данных
    Json(vec![
        Review { id: 1, user_id: 1, place_id: 1, rating: 5, comment: Some("Great place!".to_string()) },
        Review { id: 2, user_id: 2, place_id: 2, rating: 4, comment: Some("Nice place!".to_string()) },
    ])
}

#[openapi]
#[post("/")]
pub async fn create_review(review: Json<Review>) -> Json<Review> {
    // Здесь вы можете вставить отзыв в базу данных
    review
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_reviews, create_review]
}
