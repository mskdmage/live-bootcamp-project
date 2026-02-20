use serde::Deserialize;
use axum::{
    response::IntoResponse,
    http::StatusCode,
    extract::Json,
};

pub async fn signup_handler(Json(body): Json<SignupRequestBody>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}


// Payload
#[derive(Deserialize)]
pub struct SignupRequestBody {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}