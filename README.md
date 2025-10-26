# Rust REST API with MSSQL

A simple REST API built with Rust, using Actix-web framework and Tiberius for Microsoft SQL Server connectivity. This project demonstrates basic CRUD operations with proper error handling, logging, and database integration.

## Features

- **Fast and Safe**: Built with Rust for memory safety and performance
- **Async/Await**: Fully asynchronous using Tokio runtime
- **MSSQL Integration**: Direct connection to Microsoft SQL Server using Tiberius driver
- **JSON API**: RESTful endpoints with JSON request/response handling
- **Logging**: Comprehensive logging with configurable levels
- **Error Handling**: Proper error handling with informative responses

## Tech Stack

- **Rust** - Systems programming language
- **Actix-web** - High-performance web framework
- **Tiberius** - Pure Rust MSSQL driver
- **Tokio** - Async runtime
- **Serde** - Serialization framework
- **Dotenvy** - Environment variable management

## Project Structure

```
src/
├── main.rs          # Application entry point and server setup
├── db.rs            # Database connection and configuration
├── handlers.rs      # HTTP request handlers (endpoints)
└── models.rs        # Data models and structures
Cargo.toml           # Rust project configuration and dependencies
.env.example         # Environment variables template
README.md            # Project documentation
```

## API Endpoints

### GET /health
Health check endpoint to verify server status.

**Response:**
```json
"Server is running!"
```

### GET /users
Retrieve all users from the database.

**Response:**
```json
[
  {
    "id": 1,
    "username": "john_doe",
    "email": "john@example.com",
    "full_name": "John Doe",
    "created_at": null,
    "updated_at": null
  }
]
```

## Database Schema

The application expects a `users` table with the following schema:

```sql
CREATE TABLE users (
    id INT IDENTITY(1,1) PRIMARY KEY,
    username NVARCHAR(50) NOT NULL UNIQUE,
    email NVARCHAR(100) NOT NULL UNIQUE,
    full_name NVARCHAR(100) NOT NULL,
    created_at DATETIME2 DEFAULT GETUTCDATE(),
    updated_at DATETIME2 DEFAULT GETUTCDATE()
);
```

## Setup and Installation

### Prerequisites

- Rust (latest stable version)
- Microsoft SQL Server (local or remote)
- Database with `users` table created

### Installation Steps

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd rust-rest-apis
   ```

2. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Update database connection:**
   Edit the `.env` file with your MSSQL connection details:
   ```
   DB_HOST=your_database_host
   DB_PORT=1433
   DB_NAME=your_database_name
   DB_USER=your_username
   DB_PASSWORD=your_password
   ```

4. **Install dependencies and build:**
   ```bash
   cargo build
   ```

5. **Run the application:**
   ```bash
   # With logging
   RUST_LOG=info cargo run
   
   # Or simply
   cargo run
   ```

6. **Test the API:**
   ```bash
   # Health check
   curl http://localhost:8080/health
   
   # Get users
   curl http://localhost:8080/users
   ```

## Development

### Running with Logs

Enable detailed logging by setting the `RUST_LOG` environment variable:

```bash
# Windows CMD
set RUST_LOG=info && cargo run

# Windows PowerShell
$env:RUST_LOG="info"; cargo run

# Linux/macOS
RUST_LOG=info cargo run
```

Log levels available: `trace`, `debug`, `info`, `warn`, `error`

### Building for Production

```bash
# Build optimized release version
cargo build --release

# Run the release version
./target/release/rust-rest-apis
```

## Configuration

The application can be configured through environment variables:

- `DB_HOST`: Database server hostname or IP address
- `DB_PORT`: Database server port (default: 1433)
- `DB_NAME`: Target database name
- `DB_USER`: Database username
- `DB_PASSWORD`: Database password
- `RUST_LOG`: Logging level (default: none)

Server configuration (host/port) is currently hardcoded in `main.rs` but can be made configurable.

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK`: Successful requests
- `500 Internal Server Error`: Database or server errors

Error responses include JSON messages for debugging.

## Future Enhancements

- [ ] Environment-based server configuration (host/port)
- [ ] Connection pooling for better performance
- [ ] Additional CRUD operations (POST, PUT, DELETE)
- [ ] Input validation and sanitization
- [ ] Authentication and authorization
- [ ] Proper datetime handling for MSSQL DateTime2 fields
- [ ] Database migrations
- [ ] Unit and integration tests
- [ ] Docker containerization
- [ ] API documentation with OpenAPI/Swagger

## License

[Specify your license here]

## Contributing

[Add contribution guidelines here]