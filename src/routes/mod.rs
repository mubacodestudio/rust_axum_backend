mod custom_header;
mod custom_middleware;
mod example_body_json;
mod example_post;
mod extract_header;
mod path_variable;
mod query_params;
mod shared_middleware;

use axum::{
    headers::Authorization,
    http::Method,
    routing::{get, post},
    Extension, Router, TypedHeader,
};
use example_post::{example_get, example_post};
use tower_http::cors::{Any, CorsLayer};

use custom_header::custom_header;
use custom_middleware::custom_middleware;
use example_body_json::example_body_json;
use extract_header::extract_header;
use path_variable::path_variable;
use query_params::query_params;
use shared_middleware::shared_middleware;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_route() -> Router {
    let cros = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Hello from Shared Data".to_owned(),
    };
    Router::new()
        .route("/", get(example_get).post(example_post))
        .route("/post-json", post(example_body_json))
        .route("/path-variable/:id/:slug", get(path_variable))
        .route("/query-params", get(query_params))
        .route("/extract-header", get(extract_header))
        .route("/custom-header", get(custom_header))
        .route("/shared-middleware", get(shared_middleware))
        .route("custom-middleware", get(custom_middleware))
        .layer(cros)
        .layer(Extension(shared_data))
}
