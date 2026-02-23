use serde::Deserialize;
use axum::{
    response::IntoResponse,
    http::StatusCode,
    extract::{Json, State},
};
use crate::AppState;
use crate::domain::{Email, Password, AuthAPIError};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequestBody>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(&body.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&body.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.write().await;

    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let _user = user_store.get_user(&email).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct LoginRequestBody {
    email: String,
    password: String,
}