mod create_session;
mod handle_session;
mod session_storage;
mod setup_db;



use warp::Filter;
use crate::handle_session::handle_session;
use std::convert::Infallible;


#[tokio::main]
async fn main() {
    // Define a route to handle requests for creating a session
    let create_session_route = warp::path("create_session")
        .and(warp::get())  // Handle GET requests
        .and_then(create_session_handler);  // Call the handler function

    // Start the Warp server
    println!("Listening on http://localhost:8000");
    warp::serve(create_session_route)
        .run(([127, 0, 0, 1], 8000))  // Listen on localhost:8000
        .await;
}

// Define the handler that calls handle_session and returns the launch_url
async fn create_session_handler() -> Result<impl warp::Reply, Infallible> {
    match handle_session().await {
        Ok(launch_url) => {
            println!("Sending Launch URL to the frontend: {}", launch_url);
            let json_reply = warp::reply::json(&launch_url);  // Create JSON response
            Ok(warp::reply::with_status(json_reply, warp::http::StatusCode::OK))  // Return JSON with status 200
        }
        Err(_) => {
            let error_message = warp::reply::json(&"Error creating session");
            Ok(warp::reply::with_status(error_message, warp::http::StatusCode::INTERNAL_SERVER_ERROR))  // Return JSON error with status 500
        }
    }
}
