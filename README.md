# Web-Server

A lightweight web server built with Rust using the axum framework. This project provides a basic HTTP server that responds to GET requests on the root endpoint (/) with a simple message.

Features





Handles GET requests at http://localhost:3000/



Returns a static response: "Hello, Rust Web Server!"



Built with axum for high-performance routing



Uses tokio for asynchronous runtime

Requirements





Rust (version 1.56 or higher)



Cargo (Rust's package manager, included with Rust)

Setup Instructions





Clone the repository (if applicable):

git clone <repository-url>
cd simple-webserver



Ensure Rust is installed: Run the following to verify:

rustc --version
cargo --version

If Rust is not installed, download and install it from rust-lang.org.



Add dependencies: The project uses axum and tokio. The Cargo.toml file should include:

[package]
name = "simple-webserver"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.6"
tokio = { version = "1.0", features = ["full"] }



Place the server code: Ensure the src/main.rs file contains the server code (as provided in the project).

Running the Server





Navigate to the project directory:

cd simple-webserver



Build and run the server:

cargo run



Open a browser or use a tool like curl to access the server at http://localhost:3000. You should see:

Hello, Rust Web Server!

Usage





The server listens on http://localhost:3000.



Currently, it responds to GET requests at the root endpoint (/).



To extend functionality, add more routes or handlers in src/main.rs.

Contributing

Feel free to submit issues or pull requests to enhance the project. Contributions are welcome!

License

This project is licensed under the MIT License.
