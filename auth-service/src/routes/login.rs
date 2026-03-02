use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode, TwoFACodeStoreError},
    utils::auth::generate_auth_cookie,
    AppState,
};

// HANDLER

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

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    match user.requires_2fa {
        true => handle_2fa_enabled(&email, jar, &state).await,
        false => handle_2fa_disabled(&email, jar).await,
    }
}

// HELPER FUNCTIONS

async fn handle_2fa_enabled(
    email: &Email,
    jar: CookieJar,
    state: &AppState,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponseBody>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    match two_fa_code_store
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        .map_err(|e| match e {
            TwoFACodeStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
            _ => AuthAPIError::InvalidCredentials,
        }) {
        Ok(_) => (
            jar,
            Ok((
                StatusCode::PARTIAL_CONTENT,
                Json(LoginResponseBody::TwoFactor(TwoFactorAuthResponseBody {
                    message: "2FA required".to_owned(),
                    login_attempt_id: login_attempt_id.as_ref().to_owned(),
                })),
            )),
        ),
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}

async fn handle_2fa_disabled(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponseBody>), AuthAPIError>,
) {
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (
        updated_jar,
        Ok((StatusCode::OK, Json(LoginResponseBody::RegularAuth))),
    )
}

// PAYLOADS

#[derive(Deserialize)]
pub struct LoginRequestBody {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponseBody {
    RegularAuth,
    TwoFactor(TwoFactorAuthResponseBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorAuthResponseBody {
    pub message: String,
    pub login_attempt_id: String,
}
