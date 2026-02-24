use serde::Deserialize;
use axum::{
    response::IntoResponse,
    http::StatusCode,
    extract::Json,
};
use crate::domain::{AuthAPIError, Token};
use crate::utils::auth::validate_token;

pub async fn verify_token_handler(
    Json(body): Json<VerifyTokenRequestBody>,
) -> Result<impl IntoResponse, AuthAPIError> {
    
    let token = match Token::parse(&body.token) {
        Ok(token) => token,
        Err(_) => return Err(AuthAPIError::InvalidToken),
    };

    // match validate_token(&token.as_ref()).await {
    //     Ok(_) => (),
    //     Err(_) => return Err(AuthAPIError::InvalidToken),
    // }
        
    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct VerifyTokenRequestBody {
    pub token: String,
}