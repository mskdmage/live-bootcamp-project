use crate::helpers::{get_random_email, TestApp};
use auth_service::routes::SignupResponseBody;
use auth_service::ErrorResponseBody;
use serde_json::json;

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let payload = json!({
        "email" : random_email,
        "password" : "password123",
        "requires2FA" : true,
    });

    let response = app.post_signup(&payload).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponseBody {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponseBody>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        json!(
            {
                "email" : "",
                "password" : "password123",
                "requires2FA" : true,
            }
        ),
        json!(
            {
                "email" : "email_at_test.com",
                "password" : "password123",
                "requires2FA" : true,
            }
        ),
        json!(
            {
                "email" : random_email,
                "password" : "pass123",
                "requires2FA" : true,
            }
        ),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(response.status().as_u16(), 400);
        assert_eq!(
            response
                .json::<ErrorResponseBody>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let payload = json!({
        "email" : random_email,
        "password" : "password123",
        "requires2FA" : true,
    });

    let _ = app.post_signup(&payload).await;
    let response = app.post_signup(&payload).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponseBody>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}

#[tokio::test]
async fn post_signup_malformed_payload_returns_422() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        json!(
            {
                "password" : "password123",
                "requires2FA" : true,
            }
        ),
        json!(
            {
                "email" : random_email,
                "password" : "password123",
            }
        ),
        json!(
            {
                "email" : random_email,
                "requires2FA" : true,
            }
        ),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
