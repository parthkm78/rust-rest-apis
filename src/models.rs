use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
