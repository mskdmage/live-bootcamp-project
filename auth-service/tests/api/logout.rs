use auth_service::{utils::constants::JWT_COOKIE_NAME};
use reqwest::Url;
use serde_json::json;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
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

    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 200);

}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
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

    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 200);

    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
}