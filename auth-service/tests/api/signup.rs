use serde_json::json;
use crate::helpers::{
    TestApp,
    get_random_email,    
};

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