use actix_web::{web, HttpResponse};
use tiberius::{Query, Row};

use crate::models::User;
use crate::db::DbClient;

pub async fn get_users(client: web::Data<DbClient>) -> HttpResponse {
    log::info!("GET /users endpoint called");
    
    let query = Query::new("SELECT id, username, email, full_name FROM users");
    
    // Collect results while holding the lock
    let results = {
        let mut client_guard = client.lock().await;
        log::info!("Database connection acquired");
        
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
        stream.into_results().await
    }; // client_guard is dropped here
    
    match results {
        Ok(result_sets) => {
            log::info!("Query results received, processing {} result sets", result_sets.len());
            
            // Get the first result set (since we have one query)
            let rows = result_sets.into_iter().flatten().collect::<Vec<Row>>();
            log::info!("Found {} rows", rows.len());
            
            let users: Vec<User> = rows.iter().map(|row| {
                User {
                    id: row.get::<i32, _>("id").unwrap_or(0),
                    username: row.get::<&str, _>("username").unwrap_or("").to_string(),
                    email: row.get::<&str, _>("email").unwrap_or("").to_string(),
                    full_name: row.get::<&str, _>("full_name").unwrap_or("").to_string(),
                    created_at: None, // Skip datetime conversion for now
                    updated_at: None, // Skip datetime conversion for now
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

// Add a simple health check endpoint
pub async fn health_check() -> HttpResponse {
    log::info!("Health check endpoint called");
    HttpResponse::Ok().json("Server is running!")
}
