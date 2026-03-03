pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

use app_state::AppState;
use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{post, Router},
    serve::Serve,
    Json,
};
use domain::AuthAPIError;
use routes::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            // TODO: Replace [YOUR_DROPLET_IP] with your Droplet IP address
            "http://24.199.83.112:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let assets_dir = ServeDir::new("assets");

        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(signup_handler))
            .route("/login", post(login_handler))
            .route("/logout", post(logout_handler))
            .route("/verify-2fa", post(verify_2fa_handler))
            .route("/verify-token", post(verify_token_handler))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponseBody {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Authentication Failed")
            }
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::InvalidInput => (StatusCode::BAD_REQUEST, "Invalid input"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponseBody {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
