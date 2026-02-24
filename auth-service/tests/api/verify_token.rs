use serde_json::json;
use crate::helpers::{get_random_email, TestApp};

use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponseBody};

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_payload = json!(
        {
            "email": random_email,
            "password": "password123",
            "requires2FA": false
        }
    );

    let login_payload = json!(
        {
            "email": random_email,
            "password": "password123",
        }
    );

    let signup_response = app.post_signup(&signup_payload).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    let login_response = app.post_login(&login_payload).await;
    assert_eq!(login_response.status().as_u16(), 200);
    
    let auth_cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let verify_token_payload = json!(
        {
            "token": auth_cookie.value(),
        }
    );

    let verify_token_response = app.post_verify_token(&verify_token_payload).await;
    assert_eq!(verify_token_response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let verify_token_payload = json!(
        {
            "token": "invalid_token",
        }
    );

    let verify_token_response = app.post_verify_token(&verify_token_payload).await;
    assert_eq!(verify_token_response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let payload = json!(
        {
            "toke": "malformed_payload",
        }
    );

    let response = app.post_verify_token(&payload).await;
    assert_eq!(response.status().as_u16(), 422);
}