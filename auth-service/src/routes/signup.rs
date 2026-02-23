use serde::{Serialize, Deserialize};
use axum::{
    response::IntoResponse,
    http::StatusCode,
    extract::{State, Json},
};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User, UserStoreError, Email, Password},
};

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(body): Json<SignupRequestBody>,
) -> Result<impl IntoResponse, AuthAPIError> {

    let email = Email::parse(&body.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(&body.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    
    let new_user = User::new(email, password, body.requires_2fa);
    let mut user_store = state.user_store.write().await;

    user_store.add_user(new_user).await.map_err(|e| match e {
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        _ => AuthAPIError::UnexpectedError
    })?;

    let response = Json(
        SignupResponseBody {
            message: "User created successfully!".to_owned()
        }
    );
    
    Ok(
        (StatusCode::CREATED, response)
    )
}

#[derive(Deserialize)]
pub struct SignupRequestBody {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponseBody {
    pub message: String,
}