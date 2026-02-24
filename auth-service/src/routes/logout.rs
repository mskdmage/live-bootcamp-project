use axum::{http::StatusCode, response::IntoResponse, extract::State};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
    AppState,
    domain::Token,
};

pub async fn logout_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = match Token::parse(&cookie.value()) {
        Ok(token) => token,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    // Validate token first (this checks if it's banned)
    match validate_token(&token, &state.banned_token_store).await {
        Ok(_) => (),
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    }

    // Add token to banned token store
    let mut banned_token_store = state.banned_token_store.write().await;
    match banned_token_store.add_token(&token).await {
        Ok(_) => (),
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    }

    let updated_jar = jar.remove(JWT_COOKIE_NAME);

    (updated_jar, Ok(StatusCode::OK))
}