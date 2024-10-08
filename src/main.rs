mod create_session;
mod handle_session;
mod session_storage;
mod setup_db;
mod listener;  // Import the listener module

#[tokio::main]
async fn main() {
    // Start the listener that listens for requests from the frontend
    println!("Starting the listener...");
    listener::run_listener().await;
}
