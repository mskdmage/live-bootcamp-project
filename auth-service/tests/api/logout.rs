use auth_service::{domain::Token, utils::constants::JWT_COOKIE_NAME};
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

    let auth_cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let token_value = auth_cookie.value();
    let token = Token::parse(token_value).expect("Failed to parse token");

    let banned_token_store = app.banned_token_store.read().await;
    assert_eq!(
        banned_token_store.is_token_banned(&token).await,
        Ok(false),
        "Token should not be banned before logout"
    );
    drop(banned_token_store);

    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 200);

    let banned_token_store = app.banned_token_store.read().await;
    assert_eq!(
        banned_token_store.is_token_banned(&token).await,
        Ok(true),
        "Token should be banned after logout"
    );
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
