use axum::{
    extract::{Json, State},
    http::StatusCode,
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
) -> (CookieJar, Result<(StatusCode, Json<LoginResponseBody>), AuthAPIError>) {
    
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

    {
        let mut two_fa_code_store = state.two_fa_code_store.write().await;

        if let Err(e) = two_fa_code_store
            .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
            .await
        {
            let api_err = match e {
                TwoFACodeStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
                _ => AuthAPIError::InvalidCredentials,
            };
            return (jar, Err(api_err));
        }
    }

    {
        let email_client = state.email_client.write().await;

        if let Err(_) = email_client
            .send_email(email, "2FA code", two_fa_code.as_ref())
            .await
        {
            return (jar, Err(AuthAPIError::UnexpectedError));
        }
    }

    let body = LoginResponseBody::TwoFactor(TwoFactorAuthResponseBody {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    });

    (
        jar,
        Ok((StatusCode::PARTIAL_CONTENT, Json(body))),
    )
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
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
