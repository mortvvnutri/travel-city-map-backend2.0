// routers/routes.rs
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Route {
    pub id: i32,
    pub user_id: i32,
    pub name: Option<String>,
    pub creation_date: Option<String>,
    pub public_transport_info: Option<String>,
}

#[openapi]
#[get("/")]
pub async fn list_routes() -> Json<Vec<Route>> {
    // Пример данных
    Json(vec![
        Route { id: 1, user_id: 1, name: Some("Route 1".to_string()), creation_date: Some("2023-01-01T00:00:00Z".to_string()), public_transport_info: Some("Bus 42".to_string()) },
        Route { id: 2, user_id: 2, name: Some("Route 2".to_string()), creation_date: Some("2023-01-02T00:00:00Z".to_string()), public_transport_info: Some("Metro line 2".to_string()) },
    ])
}

#[openapi]
#[post("/")]
pub async fn create_route(route: Json<Route>) -> Json<Route> {
    // Здесь вы можете вставить маршрут в базу данных
    route
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_routes, create_route]
}
