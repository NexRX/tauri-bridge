//! Tauri integration tests.
//!
//! Run with: cargo test --test tauri_integration --features backend

#![cfg(all(feature = "backend", not(target_arch = "wasm32")))]

use serde::{Deserialize, Serialize};
use tauri_bridge::tauri_bridge;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[tauri_bridge]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri_bridge]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[tauri_bridge]
pub fn get_user(id: u64) -> Option<User> {
    if id > 0 {
        Some(User {
            id,
            name: format!("User{}", id),
            email: Some(format!("user{}@example.com", id)),
        })
    } else {
        None
    }
}

#[tauri_bridge]
pub fn create_user(name: String, email: Option<String>) -> User {
    User { id: 1, name, email }
}

#[tauri_bridge]
pub fn validate_input(input: &str) -> Result<String, String> {
    if input.is_empty() {
        Err("Input cannot be empty".to_string())
    } else if input.len() > 100 {
        Err("Input too long".to_string())
    } else {
        Ok(format!("Valid: {}", input))
    }
}

#[tauri_bridge]
pub fn check_status(user: User) -> Status {
    if user.email.is_some() {
        Status::Active
    } else {
        Status::Pending
    }
}

#[tauri_bridge]
pub fn process_users(users: Vec<User>) -> ApiResponse<Vec<String>> {
    let names: Vec<String> = users.into_iter().map(|u| u.name).collect();
    ApiResponse {
        success: true,
        data: Some(names),
        error: None,
    }
}

#[tauri_bridge]
pub fn echo_bool(value: bool) -> bool {
    value
}

#[tauri_bridge]
pub fn count_items(items: Vec<String>) -> u64 {
    items.len() as u64
}

#[tauri_bridge]
pub fn noop() {}

#[tauri_bridge]
pub async fn async_greet(name: String) -> String {
    format!("Async hello, {}!", name)
}

#[tauri_bridge]
pub async fn async_fetch_user(id: u64) -> Result<User, String> {
    if id == 0 {
        Err("Invalid user ID".to_string())
    } else {
        Ok(User {
            id,
            name: format!("AsyncUser{}", id),
            email: Some(format!("async{}@example.com", id)),
        })
    }
}

#[test]
fn test_greet_command() {
    assert_eq!(greet("Tauri"), "Hello, Tauri!");
}

#[test]
fn test_add_numbers_command() {
    assert_eq!(add_numbers(5, 3), 8);
    assert_eq!(add_numbers(-10, 10), 0);
    assert_eq!(add_numbers(0, 0), 0);
}

#[test]
fn test_get_user_command() {
    let user = get_user(1).unwrap();
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "User1");
    assert!(user.email.is_some());
    assert!(get_user(0).is_none());
}

#[test]
fn test_create_user_command() {
    let user = create_user("Alice".to_string(), Some("alice@example.com".to_string()));
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, Some("alice@example.com".to_string()));

    let user_no_email = create_user("Bob".to_string(), None);
    assert_eq!(user_no_email.name, "Bob");
    assert!(user_no_email.email.is_none());
}

#[test]
fn test_validate_input_command() {
    assert_eq!(validate_input("hello").unwrap(), "Valid: hello");
    assert_eq!(validate_input("").unwrap_err(), "Input cannot be empty");
    assert_eq!(
        validate_input(&"x".repeat(101)).unwrap_err(),
        "Input too long"
    );
}

#[test]
fn test_check_status_command() {
    let active_user = User {
        id: 1,
        name: "Alice".to_string(),
        email: Some("alice@example.com".to_string()),
    };
    assert_eq!(check_status(active_user), Status::Active);

    let pending_user = User {
        id: 2,
        name: "Bob".to_string(),
        email: None,
    };
    assert_eq!(check_status(pending_user), Status::Pending);
}

#[test]
fn test_process_users_command() {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: None,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: None,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: None,
        },
    ];
    let response = process_users(users);
    assert!(response.success);
    assert_eq!(response.data.unwrap(), vec!["Alice", "Bob", "Charlie"]);
}

#[test]
fn test_echo_bool_command() {
    assert!(echo_bool(true));
    assert!(!echo_bool(false));
}

#[test]
fn test_count_items_command() {
    assert_eq!(count_items(vec!["a".into(), "b".into(), "c".into()]), 3);
    assert_eq!(count_items(vec![]), 0);
}

#[test]
fn test_noop_command() {
    noop();
}

#[tokio::test]
async fn test_async_greet_command() {
    assert_eq!(
        async_greet("AsyncWorld".to_string()).await,
        "Async hello, AsyncWorld!"
    );
}

#[tokio::test]
async fn test_async_fetch_user_command() {
    let user = async_fetch_user(42).await.unwrap();
    assert_eq!(user.id, 42);
    assert_eq!(user.name, "AsyncUser42");

    assert_eq!(async_fetch_user(0).await.unwrap_err(), "Invalid user ID");
}

#[test]
fn test_ipc_serialization_flow() {
    use serde_json::json;

    let args = json!({"name": "IpcUser", "email": "ipc@example.com"});
    let name: String = args["name"].as_str().unwrap().to_string();
    let email: Option<String> = args["email"].as_str().map(|s| s.to_string());

    let user = create_user(name, email);
    let response = serde_json::to_value(&user).unwrap();

    assert_eq!(response["id"], 1);
    assert_eq!(response["name"], "IpcUser");
    assert_eq!(response["email"], "ipc@example.com");
}

#[test]
fn test_complex_type_ipc_flow() {
    use serde_json::json;

    let args = json!({
        "users": [
            {"id": 1, "name": "Alice", "email": null},
            {"id": 2, "name": "Bob", "email": "bob@example.com"}
        ]
    });
    let users: Vec<User> = serde_json::from_value(args["users"].clone()).unwrap();
    let response = process_users(users);

    let json_response = serde_json::to_value(&response).unwrap();
    assert_eq!(json_response["success"], true);
    assert_eq!(json_response["data"], json!(["Alice", "Bob"]));
}

#[test]
fn test_result_type_ipc_flow() {
    let ok_result = serde_json::to_value(validate_input("valid")).unwrap();
    assert_eq!(ok_result["Ok"], "Valid: valid");

    let err_result = serde_json::to_value(validate_input("")).unwrap();
    assert_eq!(err_result["Err"], "Input cannot be empty");
}

#[test]
fn test_unicode_handling() {
    assert_eq!(greet("世界"), "Hello, 世界!");
    assert_eq!(greet("🦀 Rust"), "Hello, 🦀 Rust!");
    assert_eq!(greet("مرحبا"), "Hello, مرحبا!");
}

#[test]
fn test_empty_string_handling() {
    assert_eq!(greet(""), "Hello, !");
}

#[test]
fn test_large_input_handling() {
    let large_name = "x".repeat(10000);
    let result = greet(&large_name);
    assert!(result.starts_with("Hello, "));
    assert!(result.len() > 10000);
}

#[test]
fn test_special_characters_handling() {
    assert_eq!(greet("Test<>&\"'"), "Hello, Test<>&\"'!");
    assert_eq!(
        greet("Line1\nLine2\tTabbed"),
        "Hello, Line1\nLine2\tTabbed!"
    );
}

#[test]
fn test_commands_can_be_registered() {
    fn accepts_handler<F: Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static>(
        _: F,
    ) {
    }

    accepts_handler(tauri::generate_handler![
        greet,
        add_numbers,
        get_user,
        create_user,
        validate_input,
        check_status,
        process_users,
        echo_bool,
        count_items,
        noop,
        async_greet,
        async_fetch_user,
    ]);
}
