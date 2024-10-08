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
  - Creates a Postgres table if it doesn't already exist
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
2. Create a new .env file
```bash
touch .env
```
3. In the .env file add the following fields (Replace with your actual credentials).
```bash
TRINSIC_AUTH_TOKEN=your_trinsic_auth_token

DB_HOST=your_db_host
DB_USER=your_db_user
DB_PASSWORD=your_db_password
DB_NAME=your_db_name
TRINSIC_API_ENDPOINT=trinsi_api_endpoint
```

4. Run the project
```bash
cargo run
```

### Interacting with the API
1. Create a Session: When the button on the frontend is pressed, a GET request is sent to the server, triggering the session process:
  - It sends a POST request to the Trinsic API.
  - Receives the JSON response.
  - Stores the session details in the database.
2. Retrieve the Latest Launch URL: Once the session is created, you can retrieve the LaunchUrl via a GET request to the server. This will return the most recently stored LaunchUrl from the database.


## Security
- API Token Handling: The API authentication token required for communication with the Trinsic API is not hardcoded in this repository.Instead, it is stored in environment variables using a .env file (which is added to .gitignore).
- Database Security: Make sure your Postgres instance is secured with proper credentials and network restrictions.


## Disclaimer
This project is not affiliated with Trinsic and is not an official SDK. It is a personal project to interact with the Trinsic API from a Rust backend. Use at your own discretion and ensure you are complying with [Trinsic’s Terms of Service](https://trinsic.id/terms/).




