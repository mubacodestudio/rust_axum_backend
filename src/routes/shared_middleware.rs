use::axum::Extension;
use super::SharedData;


pub async fn shared_middleware(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}