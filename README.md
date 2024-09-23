# MiniWebServer

This project implements a simple HTTP web server in Rust capable of handling multiple routes and serving static HTML files. It uses a thread pool to manage concurrent connections and logs incoming requests using the `env_logger` crate.

## Features

- Basic HTTP server built in Rust.
- Supports multiple routes (`/` and `/about`).
- 404 error handling for unknown routes.
- Thread pool for handling multiple connections efficiently.
- Logs incoming requests with `env_logger`.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- Optional: A web browser or a tool like `curl` to make HTTP requests.

## Installation

1. Clone the repository or download the source files:

   ```bash
   git clone https://github.com/your-username/mini-web-server.git
   cd mini-web-server

   ```

2. Add the required dependencies in your Cargo.toml file:

   ```bash
   [dependencies]
   log = "0.4"
   env_logger = "0.9"
   threadpool = "1.8"
   ```

3. Build the project using Cargo:

   ```bash
   cargo build
   ```

## Usage

1. Set the environment variable to enable logging:

   ```bash
   export RUST_LOG=info
   ```

2. Run the server:

   ```
   cargo run
   ```

3. Access the server from your browser or using curl:

Home route: http://127.0.0.1:7878/
About page: http://127.0.0.1:7878/about
404 page: Try visiting any other route to see the 404 error handling.
