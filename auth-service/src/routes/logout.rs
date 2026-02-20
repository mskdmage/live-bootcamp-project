use axum::{
    response::IntoResponse,
    http::StatusCode,
};

pub async fn logout_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}