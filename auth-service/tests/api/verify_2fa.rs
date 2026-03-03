use serde_json::json;
use crate::helpers::{get_random_email, TestApp};
use auth_service::domain::Email;
use auth_service::utils::constants::JWT_COOKIE_NAME;

#[tokio::test]
async fn post_verify_2fa_returns_ok_on_success() {
    let app = TestApp::new().await;
    let random_email = get_random_email();


    let create_user_payload = json!(
        {
            "email": random_email,
            "password": "Password123",
            "requires2FA": true
        }
    );

    let create_user_response = app.post_signup(&create_user_payload).await;
    assert_eq!(create_user_response.status().as_u16(), 201);

    let login_payload = json!(
        {
            "email": random_email,
            "password": "Password123",
        }
    );

    let login_response = app.post_login(&login_payload).await;
    assert_eq!(login_response.status().as_u16(), 206);

    let (login_attempt_id, _) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await.unwrap();

    let two_fa_code = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await
        .unwrap()
        .1;

    let payload = json!(
        {
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref().to_owned(),
            "2FACode": two_fa_code.as_ref().to_owned()
        }
    );

    let response = app.post_verify_2fa(&payload).await;

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
    assert_eq!(response.status().as_u16(), 200);

    // Second attempt with the same combination should now fail (one-time use)
    let response = app.post_verify_2fa(&payload).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn post_verify_2fa_returns_400_on_invalid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();


    let create_user_payload = json!(
        {
            "email": random_email,
            "password": "Password123",
            "requires2FA": true
        }
    );

    let create_user_response = app.post_signup(&create_user_payload).await;
    assert_eq!(create_user_response.status().as_u16(), 201);

    let login_payload = json!(
        {
            "email": random_email,
            "password": "Password123",
        }
    );

    let login_response = app.post_login(&login_payload).await;
    assert_eq!(login_response.status().as_u16(), 206);

    let (login_attempt_id, _) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await.unwrap();

    let _two_fa_code = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await
        .unwrap()
        .1;

    let payload = json!(
        {
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref().to_owned(),
            "2FACode": "123456"
        }
    );

    let response = app.post_verify_2fa(&payload).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn post_verify_2fa_returns_422_on_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();


    let create_user_payload = json!(
        {
            "email": random_email,
            "password": "Password123",
            "requires2FA": true
        }
    );

    let create_user_response = app.post_signup(&create_user_payload).await;
    assert_eq!(create_user_response.status().as_u16(), 201);

    let login_payload = json!(
        {
            "email": random_email,
            "password": "Password123",
        }
    );

    let login_response = app.post_login(&login_payload).await;
    assert_eq!(login_response.status().as_u16(), 206);

    let two_fa_code = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await
        .unwrap()
        .1;

    let payload = json!(
        {
            "email": random_email,
            "2FACode": two_fa_code.as_ref().to_owned()
        }
    );

    let response = app.post_verify_2fa(&payload).await;
    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let create_user_payload = json!({
        "email": random_email,
        "password": "Password123",
        "requires2FA": true
    });
    let create_user_response = app.post_signup(&create_user_payload).await;
    assert_eq!(create_user_response.status().as_u16(), 201);

    let login_payload = json!({
        "email": random_email,
        "password": "Password123",
    });
    let login_response = app.post_login(&login_payload).await;
    assert_eq!(login_response.status().as_u16(), 206);

    let (login_attempt_id, _two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await
        .unwrap();

    let payload = json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_ref().to_owned(),
        "2FACode": "000000"
    });
    let response = app.post_verify_2fa(&payload).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let create_user_payload = json!({
        "email": random_email,
        "password": "Password123",
        "requires2FA": true
    });
    let create_user_response = app.post_signup(&create_user_payload).await;
    assert_eq!(create_user_response.status().as_u16(), 201);

    let login_payload = json!({
        "email": random_email,
        "password": "Password123",
    });

    app.post_login(&login_payload).await;
    let (first_attempt_id, first_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(&random_email).unwrap())
        .await
        .unwrap();

    app.post_login(&login_payload).await;

    let payload = json!({
        "email": random_email,
        "loginAttemptId": first_attempt_id.as_ref().to_owned(),
        "2FACode": first_code.as_ref().to_owned()
    });
    let response = app.post_verify_2fa(&payload).await;
    assert_eq!(response.status().as_u16(), 401);
}