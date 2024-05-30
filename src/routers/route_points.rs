// routers/route_points.rs
use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RoutePoint {
    pub id: i32,
    pub user_id: i32,
    pub route_id: i32,
    pub place_id: i32,
    pub sequence_number: i32,
}

#[openapi]
#[get("/")]
pub async fn list_route_points() -> Json<Vec<RoutePoint>> {
    // Пример данных
    Json(vec![
        RoutePoint { id: 1, user_id: 1, route_id: 1, place_id: 1, sequence_number: 1 },
        RoutePoint { id: 2, user_id: 1, route_id: 1, place_id: 2, sequence_number: 2 },
    ])
}

#[openapi]
#[post("/")]
pub async fn create_route_point(route_point: Json<RoutePoint>) -> Json<RoutePoint> {
    // Здесь вы можете вставить точку маршрута в базу данных
    route_point
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_route_points, create_route_point]
}
