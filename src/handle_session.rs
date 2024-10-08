use crate::create_session::{CreateSession, CreateSessionResponse};
use crate::session_storage::SessionStorage;
use log::{info, error};  // Import logging macros
use std::error::Error;  // Use standard Error trait

// Define the SessionData struct to hold relevant session values
pub struct SessionData {
    pub fail_code: Option<String>,
    pub launch_url: Option<String>,
    pub client_token: Option<String>,
}

// Main logic to handle the session
pub async fn handle_session() -> Result<String, Box<dyn Error>> {
    // Step 1: Create a session object and call create_session method
    let create_session = match CreateSession::new() {
        Ok(cs) => {
            info!("Session object created successfully.");
            cs
        },
        Err(e) => {
            error!("Failed to initialize session object: {}", e);
            return Err(e);
        }
    };
    
    // Step 2: Call the session creation API
    let session_response = match create_session.create_session().await {
        Ok(response) => {
            info!("Session created successfully.");
            response
        },
        Err(e) => {
            error!("Failed to create session via API: {}", e);
            return Err(e);
        }
    };

    // Step 3: Process the session response and prepare for storage
    let session_data = process_session_response(session_response);

    // Step 4: Initialize SessionStorage and store session data in database
    let storage = match SessionStorage::new().await {
        Ok(store) => {
            info!("Database connection established.");
            store
        },
        Err(e) => {
            error!("Failed to establish a database connection: {}", e);
            return Err(e);
        }
    };

    // Step 5: Store the session data
    if let Err(e) = storage.store_session_data(&session_data).await {
        error!("Failed to store session data: {}", e);
        return Err(e);
    }
    info!("Session data stored in the database successfully.");

    // Step 6: Query the most recent launch_url from the database and return it as a plain string
    match storage.get_most_recent_launch_url().await {
        Ok(Some(launch_url)) => {
            info!("Launch URL retrieved successfully: {}", launch_url);
            Ok(launch_url)
        }
        Ok(None) => {
            error!("No launch URL found in the database.");
            Err("No launch_url found in the database".into())
        }
        Err(e) => {
            error!("Failed to retrieve launch URL: {}", e);
            Err(e)
        }
    }
}

// Function to process the session response
fn process_session_response(session: CreateSessionResponse) -> SessionData {
    // Extract values and return them in the SessionData struct
    let fail_code = session.session.fail_code.clone();
    let launch_url = session.launch_url.clone();
    let client_token = session.client_token.clone();

    // Log session details for successful processing
    info!("Processing session response: ID - {}, State - {}", session.session.id, session.session.state);

    match &fail_code {
        Some(fail_code) => info!("Session Fail Code: {}", fail_code),
        None => info!("No Fail Code returned."),
    }

    match &launch_url {
        Some(launch_url) => info!("Launch URL: {}", launch_url),
        None => info!("No Launch URL returned."),
    }

    match &client_token {
        Some(client_token) => info!("Client Token: {}", client_token),
        None => info!("No Client Token returned."),
    }


    SessionData {
        fail_code,
        launch_url,
        client_token,
    }
}
