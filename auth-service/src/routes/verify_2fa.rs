use axum::{
    response::IntoResponse,
    http::StatusCode,
};

pub async fn verify_2fa_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}