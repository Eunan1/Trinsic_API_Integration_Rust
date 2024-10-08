use std::error::Error;
use tokio_postgres::Client;
use log::{info, error}; // Import log macros

// Accept the client argument
pub async fn setup_database(client: &Client) -> Result<(), Box<dyn Error>> {
    // This query will create the `session_data` table only if it does not exist already.
    match client.execute(
        "CREATE TABLE IF NOT EXISTS session_data (
            id SERIAL PRIMARY KEY,
            fail_code TEXT,
            launch_url TEXT NOT NULL,
            client_token TEXT NOT NULL
        )", 
        &[],
    ).await {
        Ok(_) => {
            info!("Table `session_data` is ready.");
            Ok(())
        }
        Err(e) => {
            error!("Failed to set up `session_data` table: {}", e);
            Err(e.into())
        }
    }
}
