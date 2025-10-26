/// HTTP request handlers for the REST API
/// 
/// This module contains all the HTTP endpoint handlers that process incoming requests
/// and return appropriate responses. Each handler is an async function that takes
/// request parameters and returns an HttpResponse.

use actix_web::{web, HttpResponse};
use tiberius::{Query, Row};

use crate::models::User;
use crate::db::DbClient;

/// GET /users - Retrieve all users from the database
/// 
/// This endpoint fetches all users from the MSSQL 'users' table and returns them as JSON.
/// It demonstrates basic database querying with proper error handling and logging.
/// 
/// # Parameters
/// - `client`: Shared database client wrapped in Arc<Mutex<>> for thread-safe access
/// 
/// # Returns
/// - `200 OK`: JSON array of user objects on success
/// - `500 Internal Server Error`: JSON error message on database or processing failure
/// 
/// # Example Response
/// ```json
/// [
///   {
///     "id": 1,
///     "username": "john_doe",
///     "email": "john@example.com", 
///     "full_name": "John Doe",
///     "created_at": null,
///     "updated_at": null
///   }
/// ]
/// ```
/// 
/// # Error Handling
/// - Database connection failures return 500 with error message
/// - Query execution failures are logged and return 500
/// - Result processing failures are logged and return 500
/// 
/// # Notes
/// - DateTime fields (created_at, updated_at) are currently set to null due to 
///   datetime conversion complexity with Tiberius
/// - Database connection is properly released after query execution
/// - All operations are logged for debugging purposes
pub async fn get_users(client: web::Data<DbClient>) -> HttpResponse {
    log::info!("GET /users endpoint called");
    
    // Prepare SQL query to fetch user data (excluding datetime fields for now)
    let query = Query::new("SELECT id, username, email, full_name FROM users");
    
    // Execute query while holding database connection lock
    // Use a separate scope to ensure the lock is released promptly
    let results = {
        let mut client_guard = client.lock().await;
        log::info!("Database connection acquired");
        
        // Execute the query and get a stream of results
        let stream = match query.query(&mut *client_guard).await {
            Ok(stream) => {
                log::info!("Query executed successfully");
                stream
            },
            Err(e) => {
                log::error!("DB error: {}", e);
                return HttpResponse::InternalServerError().json("Database query failed");
            }
        };
        
        // Convert the stream to concrete results
        stream.into_results().await
    }; // client_guard is automatically dropped here, releasing the database connection
    
    // Process the query results
    match results {
        Ok(result_sets) => {
            log::info!("Query results received, processing {} result sets", result_sets.len());
            
            // Extract rows from the first (and only) result set
            // flatten() converts Vec<Vec<Row>> to a flat iterator of Row
            let rows = result_sets.into_iter().flatten().collect::<Vec<Row>>();
            log::info!("Found {} rows", rows.len());
            
            // Convert database rows to User structs
            let users: Vec<User> = rows.iter().map(|row| {
                User {
                    // Extract column values with fallback defaults for safety
                    id: row.get::<i32, _>("id").unwrap_or(0),
                    username: row.get::<&str, _>("username").unwrap_or("").to_string(),
                    email: row.get::<&str, _>("email").unwrap_or("").to_string(),
                    full_name: row.get::<&str, _>("full_name").unwrap_or("").to_string(),
                    // Skip datetime conversion for now due to Tiberius complexity
                    created_at: None,
                    updated_at: None,
                }
            }).collect();
            
            log::info!("Returning {} users", users.len());
            HttpResponse::Ok().json(users)
        },
        Err(e) => {
            log::error!("Failed to fetch rows: {}", e);
            HttpResponse::InternalServerError().json("Failed to process query results")
        }
    }
}

/// GET /health - Health check endpoint
/// 
/// A simple endpoint to verify that the server is running and responding to requests.
/// This is useful for load balancers, monitoring systems, and deployment health checks.
/// 
/// # Returns
/// - `200 OK`: JSON message confirming server is running
/// 
/// # Example Response
/// ```json
/// "Server is running!"
/// ```
/// 
/// # Notes
/// - This endpoint does not check database connectivity
/// - For a full health check including database, consider adding database ping
/// - Always returns success unless the server is completely down
pub async fn health_check() -> HttpResponse {
    log::info!("Health check endpoint called");
    HttpResponse::Ok().json("Server is running!")
}
