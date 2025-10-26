/// Database module for MSSQL connectivity using Tiberius
/// 
/// This module handles database connection setup and configuration
/// for connecting to Microsoft SQL Server using the Tiberius driver.

use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use std::sync::Arc;

/// Type alias for the database client wrapped in Arc<Mutex<>> for thread-safe sharing
/// 
/// - Arc: Allows multiple references to the same data across threads
/// - Mutex: Provides thread-safe access to the database client
/// - Client<Compat<TcpStream>>: Tiberius client with async TCP stream
pub type DbClient = Arc<tokio::sync::Mutex<Client<Compat<TcpStream>>>>;

/// Initialize database connection to MSSQL Server
/// 
/// Creates a connection to the MSSQL database using configuration values from environment variables.
/// This allows for flexible configuration across different environments (development, staging, production).
/// 
/// # Environment Variables Required
/// - `DB_HOST`: Database server hostname or IP address
/// - `DB_PORT`: Database server port (typically 1433 for MSSQL)
/// - `DB_NAME`: Target database name
/// - `DB_USER`: Database username for authentication
/// - `DB_PASSWORD`: Database password for authentication
/// 
/// # Returns
/// - `Ok(DbClient)`: Successfully connected database client
/// - `Err(Box<dyn std::error::Error>)`: Connection failed or missing environment variables
/// 
/// # Example
/// ```rust
/// let client = init_db().await?;
/// ```
pub async fn init_db() -> Result<DbClient, Box<dyn std::error::Error>> {
    // Read database configuration from environment variables
    let db_host = std::env::var("DB_HOST")
        .map_err(|_| "DB_HOST environment variable not set")?;
    
    let db_port = std::env::var("DB_PORT")
        .map_err(|_| "DB_PORT environment variable not set")?
        .parse::<u16>()
        .map_err(|_| "DB_PORT must be a valid port number")?;
    
    let db_name = std::env::var("DB_NAME")
        .map_err(|_| "DB_NAME environment variable not set")?;
    
    let db_user = std::env::var("DB_USER")
        .map_err(|_| "DB_USER environment variable not set")?;
    
    let db_password = std::env::var("DB_PASSWORD")
        .map_err(|_| "DB_PASSWORD environment variable not set")?;

    let mut config = Config::new();
    
    // Configure database connection using environment variables
    config.host(&db_host);
    config.port(db_port);
    config.database(&db_name);
    
    // SQL Server authentication with credentials from environment
    config.authentication(AuthMethod::sql_server(&db_user, &db_password));
    
    // Trust the server certificate (for development only)
    // In production, use proper certificate validation
    config.trust_cert();

    log::info!("Connecting to MSSQL at {}:{} database: {}", db_host, db_port, db_name);

    // Establish TCP connection to the database server
    let tcp = TcpStream::connect(config.get_addr()).await?;
    
    // Enable TCP_NODELAY for better performance with small packets
    tcp.set_nodelay(true)?;

    // Create the Tiberius client with the TCP connection
    let client = Client::connect(config, tcp.compat_write()).await?;
    
    log::info!("Successfully connected to MSSQL database");
    
    // Wrap the client in Arc<Mutex<>> for thread-safe sharing across handlers
    Ok(Arc::new(tokio::sync::Mutex::new(client)))
}
