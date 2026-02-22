use serde::{Serialize, Deserialize};
use axum::{
    response::IntoResponse,
    http::StatusCode,
    extract::{State, Json},
};
use crate::{
    app_state::AppState,
    domain::User,
};

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(body): Json<SignupRequestBody>,
) -> impl IntoResponse {
    
    let new_user = User::new(&body.email, &body.password, body.requires_2fa);
    let mut user_store = state.user_store.write().await;

    let _ = user_store.add_user(new_user).unwrap();

    let response = Json(
        SignupResponseBody {
            message: "User created successfully!".to_owned()
        }
    );
    
    (StatusCode::CREATED, response)
}


// Payload
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