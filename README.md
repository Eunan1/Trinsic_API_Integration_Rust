# Trinsic API Rust Integration

This project provides a Rust backend to integrate with the [Trinsic API](https://trinsic.id/). The project includes methods for initiating sessions, handling API responses, and exposing attributes through an API to be consumed by a frontend.

Note: This is a personal project and is not an official SDK provided by Trinsic. It was built due to the lack of a native Rust SDK for server-side integration with Trinsic's API.


## Features

**Create Session:** Sends a POST request to the Trinsic API to start a session and fetch data to start the user authentication prcoess, most importantly: LaunchUrl and ClientToken.

**Database Interaction:** Creates a new database if it doesn't already exist to store FailCode, LaunchUrl, and ClientToken. Stores the information in the database, and when this is complete fetches the most recent LaunchUrl entry into the database.

**API Endpoint:** Listens for incoming requests on localhost and returns the most recent LaunchUrl when triggered.


## Project Structure
The project is composed of the different modules explained below:

- create_session.rs
  - Handles sending a POST request to the Trinsic API.
  - Receives a JSON response containing FailCode, ClientToken, and LaunchUrl.
  - Auth token is required in the request header, but it is not exposed in this repository (see security details below).

- setup_db.rs:
  - Creates the necessary Postgres table layout if it doesn't already exist.
  - Prepares the database to store session data: ID (Primary Key), FailCode, LaunchUrl, and ClientToken.

- session_storage.rs:
  - Contains method to setup a new database and login to it using credentials in the .env file.
  - Contains methods to insert the session data and retrieve the most recent LaunchUrl entry from the table.

- handle_session.rs:
  - This orchestrates the full session process, calling the previous modules sequentially.
  - Each method is called one after the other. The next method can only start after the last one has completed, using the await function.
  - Sends the POST request to the Trinsic API and returns parsed JSON data.
  - Creates a Postgres table if it doesn't already exist.
  - Stores the returned JSON data into the Postgres table.
  - Queries the table and returns the most recent LaunchUrl entry into the table.

- listener.rs
  - Listens for incoming GET requests and calls handle_session.rs to execute the session workflow.

- main.rs
  - Acts as the entry point to the server.



## Usage
### Prerequisites 
- Rust installed on your machine.
- Install Postgres and run an instance locally or remotely.
- A Trinsic account to receive necessary authentication tokens.

### Environment Setup
1. Clone my repository
```bash
git clone https://github.com/Eunan1/trinsic-api-rust-integration.git
cd trinsic-api-rust-integration
```
2. Install and run a Postgres instance locally or remotely.
  - Help with installing PostgreSQL on any machine can be found here: [Download PostgreSQL](https://www.postgresql.org/download/).
  - Help with running a PostgreSQL service and checking for tables can be found here: [PostgreSQL Getting Started](https://www.postgresql.org/docs/current/tutorial-start.html).
3. Create a new .env file
```bash
touch .env
```
4. In the .env file add the following fields (Replace with your actual credentials).
```bash
TRINSIC_AUTH_TOKEN=your_trinsic_auth_token
TRINSIC_API_ENDPOINT=trinsic_api_endpoint

DB_HOST=your_db_host
DB_USER=your_db_user
DB_PASSWORD=your_db_password
DB_NAME=your_db_name

```

5. Run the project
```bash
cargo run
```

### Interacting with the API
1. Create a Session: When a GET request is sent to the server at listener.rs, this calls handle_session.rs which starts the Trinsic session creation and management workflow:
  - A POST request is sent to the Trinsic API.
  - Receives the JSON response.
  - Returns necessary information in a readble format.
  - Creates the table to store this information
  - Stores the desired session details in the database.
  - Queries the database to get the most recent LaunchUrl entered into the table.

2. Retrieve the Latest Launch URL: Use the curl command below to send a simple GET request to the server and test to see if it returns the LaunchURL.
  ```bash
  curl http://localhost:8000/create_session
  ```


## Security
- API Token Handling: The API authentication token required for communication with the Trinsic API is not hardcoded in this repository.Instead, it is stored in environment variables using a .env file (which is added to .gitignore).
- Database Security: Make sure your Postgres instance is secured with proper credentials and network restrictions.


## Disclaimer
This project is not affiliated with Trinsic and is not an official SDK. It is a personal project to interact with the Trinsic API from a Rust backend. Use at your own discretion and ensure you are complying with [Trinsic’s Terms of Service](https://trinsic.id/terms/).




