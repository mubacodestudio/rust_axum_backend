use axum::{Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BodyJson {
    message: String,
}

#[derive(Serialize)]
pub struct BodyJsonResponse {
    message: String,
    id: i32,
}


pub async fn example_body_json(Json(body) : Json<BodyJson>) -> Json<BodyJsonResponse> {
    Json(BodyJsonResponse {message: body.message, id: 16})
}