/// Data models for the REST API
/// 
/// This module contains the data structures used throughout the application.
/// All models implement Serialize for JSON response serialization.

use serde::Serialize;

/// User data model representing a user in the system
/// 
/// This struct maps to the 'users' table in the MSSQL database with the following schema:
/// - id: INT IDENTITY(1,1) PRIMARY KEY
/// - username: NVARCHAR(50) NOT NULL UNIQUE
/// - email: NVARCHAR(100) NOT NULL UNIQUE  
/// - full_name: NVARCHAR(100) NOT NULL
/// - created_at: DATETIME2 DEFAULT GETUTCDATE()
/// - updated_at: DATETIME2 DEFAULT GETUTCDATE()
/// 
/// # Fields
/// - `id`: Unique identifier for the user (auto-generated)
/// - `username`: Unique username for login purposes
/// - `email`: User's email address (must be unique)
/// - `full_name`: User's complete name for display
/// - `created_at`: Timestamp when user was created (optional for API responses)
/// - `updated_at`: Timestamp when user was last updated (optional for API responses)
/// 
/// # Serialization
/// This struct automatically serializes to JSON for API responses using Serde.
/// 
/// # Example JSON Output
/// ```json
/// {
///   "id": 1,
///   "username": "john_doe",
///   "email": "john@example.com",
///   "full_name": "John Doe",
///   "created_at": "2025-10-26T10:30:00",
///   "updated_at": "2025-10-26T10:30:00"
/// }
/// ```
#[derive(Serialize)]
pub struct User {
    /// Unique user identifier
    pub id: i32,
    
    /// Unique username for authentication
    pub username: String,
    
    /// User's email address
    pub email: String,
    
    /// User's full display name
    pub full_name: String,
    
    /// Optional timestamp when user was created
    /// Note: Currently skipped in API responses due to datetime conversion complexity
    pub created_at: Option<String>,
    
    /// Optional timestamp when user was last updated
    /// Note: Currently skipped in API responses due to datetime conversion complexity
    pub updated_at: Option<String>,
}
