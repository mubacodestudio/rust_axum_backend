use axum::{TypedHeader, headers::UserAgent};


pub async fn extract_header(TypedHeader(header_value): TypedHeader<UserAgent>) -> String {
    header_value.to_string()
}