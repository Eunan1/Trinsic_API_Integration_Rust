use crate::create_session::{CreateSession, CreateSessionResponse};
use crate::session_storage::SessionStorage;
use std::error::Error;

// Define the SessionData struct to hold relevant session values
pub struct SessionData {
    pub fail_code: Option<String>,
    pub launch_url: Option<String>,
    pub client_token: Option<String>,
}

// Main logic to handle the session
pub async fn handle_session() -> Result<String, Box<dyn Error>> {
    // Step 1: Create a session object and call create_session method
    // Create a session object and call the create_session method
    let create_session = CreateSession::new()?;
    let session_response = create_session.create_session().await?;

    
    // Step 2: Process the session response and prepare for storage
    let session_data = process_session_response(session_response);

    // Step 3: Initialize SessionStorage and store session data in database
    let storage = SessionStorage::new().await?;
    storage.store_session_data(&session_data).await?;

    // (4) Query the most recent launch_url from the database and return it as a plain string
    match storage.get_most_recent_launch_url().await {
        Ok(Some(launch_url)) => {
            println!("Launch URL retrieved: {}", launch_url); 
            Ok(launch_url)
        },  // Return the launch_url directly
        
        Ok(None) => Err("No launch_url found in the database".into()),  // Proper error handling for no result
        Err(e) => Err(e),  // Box the error only if it's not already boxed
    }
}

// Function to process the session response
fn process_session_response(session: CreateSessionResponse) -> SessionData {
    // Extract values and return them in the SessionData struct
    let fail_code = session.session.fail_code.clone();
    let launch_url = session.launch_url.clone();
    let client_token = session.client_token.clone();

    // Print for debugging
    println!("Session ID: {}", session.session.id);
    println!("Session State: {}", session.session.state);

    match &fail_code {
        Some(fail_code) => println!("Session Fail Code: {}", fail_code),
        None => println!("No Fail Code."),
    }

    match &launch_url {
        Some(launch_url) => println!("Launch URL: {}", launch_url),
        None => println!("No Launch URL returned."),
    }

    match &client_token {
        Some(client_token) => println!("Client Token: {}", client_token),
        None => println!("No Client Token returned."),
    }

    SessionData {
        fail_code,
        launch_url,
        client_token,
    }
}
