// routers/notifications.rs
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub message: String,
    pub is_read: bool,
}

#[openapi]
#[get("/")]
pub async fn list_notifications() -> Json<Vec<Notification>> {
    // Пример данных
    Json(vec![
        Notification { id: 1, user_id: 1, message: "Notification 1".to_string(), is_read: false },
        Notification { id: 2, user_id: 2, message: "Notification 2".to_string(), is_read: true },
    ])
}

#[openapi]
#[post("/")]
pub async fn create_notification(notification: Json<Notification>) -> Json<Notification> {
    // Здесь вы можете вставить уведомление в базу данных
    notification
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_notifications, create_notification]
}
