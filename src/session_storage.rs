use tokio_postgres::{Client, NoTls};
use std::error::Error;
use dotenv::dotenv;  // To load environment variables
use std::env;  // For fetching environment variables
use crate::setup_db::setup_database;  // Import the setup_db module to ensure the table exists
use crate::handle_session::SessionData;  // Import SessionData struct from handle_session.rs


// Struct to represent the session storage, with a database client
pub struct SessionStorage {
    client: Client,
}

impl SessionStorage {
    // Function to create a new SessionStorage instance, including database setup
    pub async fn new() -> Result<Self, Box<dyn Error>> {

        // Load environment variables from the .env file
        dotenv().ok();

        // Fetch the required environment variables
        let db_host = env::var("DB_HOST")?;
        let db_user = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_name = env::var("DB_NAME")?;

        // Create the connection string using the environment variables
        let connection_str = format!(
            "host={} user={} password={} dbname={}",
            db_host, db_user, db_password, db_name
        );

        // Connect to the database
        let (client, connection) = tokio_postgres::connect(&connection_str, NoTls).await?;

        // Spawn the connection to keep it alive
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Call the setup_database function to ensure the table is created
        setup_database(&client).await?;

        Ok(SessionStorage { client })
    }

    // Function to store session data in the database
    pub async fn store_session_data(&self, session_data: &SessionData) -> Result<(), Box<dyn Error>> {
        // Insert session data into the table
        self.client.execute(
            "INSERT INTO session_data (fail_code, launch_url, client_token) VALUES ($1, $2, $3)",
            &[&session_data.fail_code, &session_data.launch_url, &session_data.client_token]
        ).await?;

        println!("Session data stored.");
        Ok(())
    }

    // Function to query the database and get the most recent launch_url
    pub async fn get_most_recent_launch_url(&self) -> Result<Option<String>, Box<dyn Error>> {
        // Query the most recent launch_url from the session_data table
        let row = self.client
            .query_one("SELECT launch_url FROM session_data ORDER BY id DESC LIMIT 1", &[])
            .await?;

        let launch_url: Option<String> = row.get(0);
        Ok(launch_url)
    
    }
}
