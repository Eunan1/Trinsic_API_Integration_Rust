use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use dotenv::dotenv;
use std::env;

// Struct to represent the session creator
pub struct CreateSession {
    client: Client,
    authorization: String,
}

// Implement CreateSession functionality
impl CreateSession {
    // Constructor to create a new session object
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let authorization = get_authorization_token()?;
        Ok(CreateSession { client, authorization })
    }

    // Function to create a session
    pub async fn create_session(&self) -> Result<CreateSessionResponse, Box<dyn Error>> {
        let request_body = CreateSessionRequest {
            launch_provider_directly: false,
        };

        let response = self
            .client
            .post("https://api.trinsic.id/api/v1/sessions")
            .bearer_auth(&self.authorization)
            .json(&request_body)
            .send()
            .await?;

        let session_response = response.json::<CreateSessionResponse>().await?;
        Ok(session_response)
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
    env::var("TRINSIC_AUTH_TOKEN").map_err(|_| "Missing authorization token".into())
}
