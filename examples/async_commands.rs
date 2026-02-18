//! Async commands example.
//!
//! Run with: cargo run --example async_commands

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn fetch_user(id: u64) -> Result<User, String> {
    if id == 0 {
        return Err("Invalid user ID".to_string());
    }
    Ok(User {
        id,
        name: format!("User {}", id),
        email: format!("user{}@example.com", id),
        created_at: "2024-01-01T00:00:00Z".to_string(),
    })
}

pub async fn search_users(query: String, limit: u32, offset: u32) -> Vec<User> {
    (0..limit.min(10))
        .map(|i| User {
            id: offset as u64 + i as u64,
            name: format!("{} Result {}", query, i),
            email: format!("result{}@example.com", i),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        })
        .collect()
}

pub async fn fetch_external_data(url: String) -> Result<String, String> {
    Ok(format!("Response from: {}", url))
}

pub async fn get_users_page(page: u32, per_page: u32) -> ApiResponse<Vec<User>> {
    let users: Vec<User> = (0..per_page)
        .map(|i| {
            let id = (page * per_page + i) as u64;
            User {
                id,
                name: format!("User {}", id),
                email: format!("user{}@example.com", id),
                created_at: "2024-01-01T00:00:00Z".to_string(),
            }
        })
        .collect();

    ApiResponse {
        success: true,
        data: Some(users),
        error: None,
    }
}

pub async fn create_user(name: String, email: String) -> Result<User, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if !email.contains('@') {
        return Err("Invalid email address".to_string());
    }
    Ok(User {
        id: 12345,
        name,
        email,
        created_at: "2024-01-15T10:30:00Z".to_string(),
    })
}

fn main() {
    println!("Async Commands Example");
    println!();
    println!("This example shows async function patterns.");
    println!("In a real app, use #[tauri_bridge] on these functions.");
    println!();
    println!("Generated client functions:");
    println!("  try_fetch_user(id) -> Result<User, String>");
    println!("  fetch_user(id) -> User (unwraps result)");
}
