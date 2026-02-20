use axum::{
    response::IntoResponse,
    http::StatusCode,
};

pub async fn login_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}