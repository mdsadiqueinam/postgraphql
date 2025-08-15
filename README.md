# PostGraphile Rust

A Rust library for creating GraphQL APIs with PostgreSQL, inspired by PostGraphile.

## Features

- **async-graphql**: High-performance GraphQL server library
- **tokio-postgres**: Async PostgreSQL client
- **Tokio**: Async runtime for Rust

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL database

### Installation

1. Clone this repository or use it as a library
2. Copy `.env.example` to `.env` and configure your database connection:
   ```bash
   cp .env.example .env
   ```
3. Update the `DATABASE_URL` in `.env` with your PostgreSQL connection details

### Running the Example

```bash
# Build the project
cargo build

# Run the example
cargo run

# Run tests
cargo test
```

### Usage as a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
postgraphile-rust = "0.1.0"
```

Example usage:

```rust
use postgraphile_rust::{create_db_client, create_schema};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create database client
    let db_client = create_db_client("postgresql://localhost/mydb").await?;
    
    // Create GraphQL schema
    let schema = create_schema().data(db_client);
    
    // Execute a query
    let query = r#"query { hello }"#;
    let request = async_graphql::Request::new(query);
    let response = schema.execute(request).await;
    
    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
```

## Project Structure

- `src/lib.rs` - Main library code with GraphQL schema and database helpers
- `src/main.rs` - Example binary demonstrating usage
- `Cargo.toml` - Project dependencies and metadata

## Dependencies

- **tokio-postgres** (0.7) - PostgreSQL async client
- **async-graphql** (7.0) - GraphQL server implementation
- **tokio** (1.0) - Async runtime with full features
- **serde** (1.0) - Serialization framework
- **serde_json** (1.0) - JSON serialization
- **uuid** (1.0) - UUID generation and parsing

## Development

This project is set up as both a library and binary. You can:

1. Use it as a library in other projects
2. Extend the example in `src/main.rs`
3. Add more GraphQL resolvers in `src/lib.rs`
4. Add database models and queries

## Next Steps

- Add more GraphQL resolvers
- Implement database models
- Add authentication and authorization
- Set up a web server (e.g., with warp or axum)
- Add database migrations
- Implement subscriptions
