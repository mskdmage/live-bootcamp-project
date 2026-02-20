use axum::{
    response::IntoResponse,
    http::StatusCode,
};

pub async fn signup_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}