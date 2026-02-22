use serde_json::json;
use crate::helpers::{
    TestApp,
    get_random_email,    
};
use auth_service::routes::SignupResponseBody;

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    // Expected:
    //  {
    //      "email": "DIIV26@WPmWVXNBvnTBYGNierIsY.dtu",
    //      "password": "string",
    //      "requires2FA": true
    //  }

    let payload = json!({
        "email" : random_email,
        "password" : "password123",
        "requires2FA" : true,
    });

    let response = app.post_signup(&payload).await;

    assert_eq!(
        response.status().as_u16(),
        201
    );

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
async fn post_signup_malformed_payload_returns_422() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    
    // Expected:
    //  {
    //      "email": "DIIV26@WPmWVXNBvnTBYGNierIsY.dtu",
    //      "password": "string",
    //      "requires2FA": true
    //  }

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