//! Basic usage example for tauri-bridge-macros.
//!
//! Run with: cargo run --example basic_usage

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

fn main() {
    println!("Tauri Bridge Macros - Basic Usage Example");
    println!("==========================================\n");

    println!("The #[tauri_bridge] macro generates both backend and client code:\n");

    println!("  #[tauri_bridge]");
    println!("  pub fn greet(name: &str) -> String {{");
    println!("      format!(\"Hello, {{}}!\", name)");
    println!("  }}\n");

    println!("Generates:");
    println!("  Backend:  greet() with #[tauri::command]");
    println!("  Client:   try_greet() -> Result<String, String>");
    println!("  Client:   greet() -> String (unwraps result)\n");

    let user = User {
        id: 1,
        name: "Alice".into(),
        email: Some("alice@example.com".into()),
    };
    println!("Example User: {:?}", user);

    let request = CreateUserRequest {
        name: "Bob".into(),
        email: None,
    };
    println!("Example Request: {:?}", request);
}
