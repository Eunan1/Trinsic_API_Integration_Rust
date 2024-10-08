use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use dotenv::dotenv;
use std::env;
use log::error;

// Struct to represent the session creator
pub struct CreateSession {
    client: Client,
    authorization: String,
    api_endpoint: String,
}

// Implement CreateSession functionality
impl CreateSession {
    // Constructor to create a new session object
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Load environment variables from the .env file
        dotenv().ok();
        // Initialize new client
        let client = Client::new();

        // Get authorization token from env and handle errors
        let authorization = match get_authorization_token() {
            Ok(token) => token,
            Err(e) => {
                error!("Failed to get authorization token: {}", e);
                return Err(e);
            }
        };

        // Get API endpoint from environment variables
        let api_endpoint = match env::var("TRINSIC_API_ENDPOINT") {
            Ok(endpoint) => endpoint,
            Err(e) => {
                error!("Missing TRINSIC_API_ENDPOINT in environment variables: {}", e);
                return Err(e.into());
            }
        };
        Ok(CreateSession { client, authorization, api_endpoint })
    }

    // Function to create a session
    pub async fn create_session(&self) -> Result<CreateSessionResponse, Box<dyn Error>> {
        let request_body = CreateSessionRequest {
            launch_provider_directly: false,
        };

        let response = match self
            .client
            .post(&self.api_endpoint)
            .bearer_auth(&self.authorization)
            .json(&request_body)
            .send()
            .await
            {
                Ok(res) => res,
                Err(e) => {
                    error!("Failed to send create session request: {}", e);
                    return Err(e.into());
                }
            };

        // Attempt to parse the response and catch errors
        match response.json::<CreateSessionResponse>().await {
            Ok(session_response) => Ok(session_response),
            Err(e) => {
                error!("Failed to parse session response: {}", e);
                Err(e.into())
            }
        }
    }
}

// Struct for the request body
#[derive(Serialize)]
pub struct CreateSessionRequest {
    #[serde(rename = "launchProviderDirectly")]
    launch_provider_directly: bool,
}

// Struct for the response
#[derive(Deserialize)]
pub struct CreateSessionResponse {
    pub session: Session,
    #[serde(rename = "launchUrl")]
    pub launch_url: Option<String>,
    #[serde(rename = "clientToken")]
    pub client_token: Option<String>,
}

// Struct for the session object
#[derive(Deserialize)]
pub struct Session {
    pub id: String,
    pub state: String,
    pub fail_code: Option<String>,
}

// Fetch the authorization token from an environment variable
fn get_authorization_token() -> Result<String, Box<dyn Error>> {
    dotenv().ok();
    env::var("TRINSIC_AUTH_TOKEN").map_err(|_| {
        error!("Missing authorization token in environment variables.");
        "Missing authorization token".into()
    })
}