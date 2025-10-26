// Module declarations for database, handlers, and models
mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenvy::dotenv;

/// Main entry point for the Rust REST API application
/// 
/// This application provides a RESTful API for user management using:
/// - Actix-web as the web framework
/// - Tiberius for MSSQL database connectivity
/// - Environment-based configuration
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logging (controlled by RUST_LOG environment variable)
    env_logger::init();

    // Initialize database connection pool
    // This will panic if database connection fails - appropriate for startup
    let client = db::init_db().await.expect("Failed to connect to database");

    log::info!("Starting server at http://127.0.0.1:8080");

    // Create and configure the HTTP server
    HttpServer::new(move || {
        App::new()
            // Share database client across all handlers
            .app_data(web::Data::new(client.clone()))
            // Add request logging middleware
            .wrap(Logger::default())
            // API Routes
            .route("/health", web::get().to(handlers::health_check))  // Health check endpoint
            .route("/users", web::get().to(handlers::get_users))     // Get all users endpoint
    })
    .bind("127.0.0.1:8080")?  // Bind to localhost on port 8080
    .run()                    // Start the server
    .await
}
