use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    domain::{AuthAPIError, Token},
    utils::auth::validate_token,
    AppState,
};

// HANDLER

pub async fn verify_token_handler(
    State(state): State<AppState>,
    Json(body): Json<VerifyTokenRequestBody>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = match Token::parse(&body.token) {
        Ok(token) => token,
        Err(_) => return Err(AuthAPIError::InvalidToken),
    };

    match validate_token(&token, &state.banned_token_store).await {
        Ok(_) => (),
        Err(_) => return Err(AuthAPIError::InvalidToken),
    }

    Ok(StatusCode::OK.into_response())
}

// PAYLOADS

#[derive(Deserialize)]
pub struct VerifyTokenRequestBody {
    pub token: String,
}
