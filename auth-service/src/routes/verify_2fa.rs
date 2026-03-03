use serde::Deserialize;
use axum::{extract::{Json, State}, http::StatusCode};
use crate::{domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode, TwoFACodeStoreError}, app_state::AppState};
use axum_extra::extract::CookieJar;
use crate::utils::auth::generate_auth_cookie;

// HANDLER

pub async fn verify_2fa_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<Verify2FARequestBody>,
) -> (CookieJar, Result<StatusCode, AuthAPIError>)  {

    let email = match Email::parse(&body.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidInput)),
    };

    let login_attempt_id = match LoginAttemptId::parse(body.login_attempt_id) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return (jar, Err(AuthAPIError::InvalidInput)),
    };

    let two_fa_code = match TwoFACode::parse(body.two_fa_code) {
        Ok(two_fa_code) => two_fa_code,
        Err(_) => return (jar, Err(AuthAPIError::InvalidInput)),
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    
    let (stored_login_attempt_id, stored_two_fa_code) = match two_fa_code_store
        .get_code(&email)
        .await
        .map_err(|e| match e {
            TwoFACodeStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
            _ => AuthAPIError::IncorrectCredentials,
        }) {
        Ok((stored_login_attempt_id, stored_two_fa_code)) => (stored_login_attempt_id, stored_two_fa_code),
        Err(e) => return (jar, Err(e)),
    };

    if stored_login_attempt_id != login_attempt_id || stored_two_fa_code != two_fa_code {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if let Err(e) = two_fa_code_store
        .remove_code(&email)
        .await
        .map_err(|e| match e {
            TwoFACodeStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
            _ => AuthAPIError::IncorrectCredentials,
        }) {
        return (jar, Err(e));
    }

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (
        updated_jar,
        Ok(StatusCode::OK),
    )
}

// PAYLOADS

#[derive(Deserialize)]
pub struct Verify2FARequestBody {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}