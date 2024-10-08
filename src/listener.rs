use warp::Filter;
use std::convert::Infallible;
use log::{info, error};  // Use logging macros
use crate::handle_session::handle_session;

// Function to setup and run the listener
pub async fn run_listener() {
    // Define a route to handle requests for creating a session
    let create_session_route = warp::path("create_session")
        .and(warp::get())  // Handle GET requests
        .and_then(create_session_handler);  // Call the handler function

    // Log when the server starts listening on localhost:8000
    info!("Server listening on http://localhost:8000");
    warp::serve(create_session_route)
        .run(([127, 0, 0, 1], 8000))  // Listen on localhost:8000
        .await;
}

// Define the handler that calls handle_session and returns the launch_url
async fn create_session_handler() -> Result<impl warp::Reply, Infallible> {
    match handle_session().await {
        Ok(launch_url) => {
            // Log success
            info!("Session created successfully, sending Launch URL to the frontend: {}", launch_url);
            
            // Create JSON response with status 200
            let json_reply = warp::reply::json(&launch_url);
            Ok(warp::reply::with_status(json_reply, warp::http::StatusCode::OK))
        }
        Err(e) => {
            // Log the error
            error!("Error creating session: {}", e);
            
            // Return JSON error with status 500
            let error_message = warp::reply::json(&"Error creating session");
            Ok(warp::reply::with_status(error_message, warp::http::StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
