use axum_extra::extract::CookieJar;
use serde::Deserialize;
use axum::{
    response::IntoResponse,
    http::StatusCode,
    extract::{Json, State},
};
use crate::{AppState, utils::auth::generate_auth_cookie};
use crate::domain::{Email, Password, AuthAPIError};

pub async fn login_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequestBody>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {

    let email = match Email::parse(&body.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(&body.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = state.user_store.write().await;

    if let Err(_) = user_store
        .validate_user(&email, &password)
        .await
    {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let _user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}

#[derive(Deserialize)]
pub struct LoginRequestBody {
    email: String,
    password: String,
}