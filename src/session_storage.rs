use tokio_postgres::{Client, NoTls};
use std::error::Error;
use dotenv::dotenv;  // To load environment variables
use std::env;  // For fetching environment variables
use log::{info, error}; // Import logging macros
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
        let db_host = match env::var("DB_HOST") {
            Ok(host) => host,
            Err(e) => {
                error!("Missing DB_HOST environment variable: {}", e);
                return Err(e.into());
            }
        };
        
        let db_user = match env::var("DB_USER") {
            Ok(user) => user,
            Err(e) => {
                error!("Missing DB_USER environment variable: {}", e);
                return Err(e.into());
            }
        };

        let db_password = match env::var("DB_PASSWORD") {
            Ok(password) => password,
            Err(e) => {
                error!("Missing DB_PASSWORD environment variable: {}", e);
                return Err(e.into());
            }
        };

        let db_name = match env::var("DB_NAME") {
            Ok(name) => name,
            Err(e) => {
                error!("Missing DB_NAME environment variable: {}", e);
                return Err(e.into());
            }
        };

        // Create the connection string using the environment variables
        let connection_str = format!(
            "host={} user={} password={} dbname={}",
            db_host, db_user, db_password, db_name
        );

        // Connect to the database
        let (client, connection) = match tokio_postgres::connect(&connection_str, NoTls).await {
            Ok((client, connection)) => (client, connection),
            Err(e) => {
                error!("Failed to connect to the database: {}", e);
                return Err(e.into());
            }
        };

        // Spawn the connection to keep it alive
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Call the setup_database function to ensure the table is created
        if let Err(e) = setup_database(&client).await {
            error!("Failed to set up the database: {}", e);
            return Err(e.into());
        }

        Ok(SessionStorage { client })
    }

    // Function to store session data in the database
    pub async fn store_session_data(&self, session_data: &SessionData) -> Result<(), Box<dyn Error>> {
        // Insert session data into the table
        match self.client.execute(
            "INSERT INTO session_data (fail_code, launch_url, client_token) VALUES ($1, $2, $3)",
            &[&session_data.fail_code, &session_data.launch_url, &session_data.client_token]
        ).await {
            Ok(_) => {
                info!("Session data successfully stored in the database.");
                Ok(())
            }
            Err(e) => {
                error!("Failed to store session data: {}", e);
                Err(e.into())
            }
        }
    }

    // Function to query the database and get the most recent launch_url
    pub async fn get_most_recent_launch_url(&self) -> Result<Option<String>, Box<dyn Error>> {
        // Query the most recent launch_url from the session_data table
        match self.client
            .query_one("SELECT launch_url FROM session_data ORDER BY id DESC LIMIT 1", &[])
            .await {
            Ok(row) => {
                let launch_url: Option<String> = row.get(0);
                Ok(launch_url)
            }
            Err(e) => {
                error!("Failed to retrieve the most recent launch_url: {}", e);
                Err(e.into())
            }
        }
    }
}
