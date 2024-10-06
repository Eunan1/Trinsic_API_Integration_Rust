use std::error::Error;
use tokio_postgres::Client;

// Accept the client argument
pub async fn setup_database(client: &Client) -> Result<(), Box<dyn Error>> {
    // This query will create the `session_data` table only if it does not exist already.
    client.execute(
        "CREATE TABLE IF NOT EXISTS session_data (
            id SERIAL PRIMARY KEY,
            fail_code TEXT,
            launch_url TEXT NOT NULL,
            client_token TEXT NOT NULL
        )", 
        &[],
    ).await?;

    println!("Table `session_data` is ready.");
    Ok(())
}
