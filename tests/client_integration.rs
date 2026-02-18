//! Client integration tests for the generated WASM client functions.
//!
//! Run with: cargo test --test client_integration --features "backend wasm-client"

#![cfg(all(feature = "backend", feature = "wasm-client"))]

use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct InvokeCall {
    pub command: String,
    pub args: serde_json::Value,
}

struct MockState {
    calls: Vec<InvokeCall>,
    next_response: Option<serde_json::Value>,
}

impl MockState {
    fn new() -> Self {
        Self {
            calls: Vec::new(),
            next_response: None,
        }
    }
}

thread_local! {
    static MOCK_STATE: RefCell<MockState> = RefCell::new(MockState::new());
}

fn set_mock_response<T: Serialize>(value: T) {
    let json = serde_json::to_value(value).unwrap();
    MOCK_STATE.with(|state| {
        state.borrow_mut().next_response = Some(json);
    });
}

fn get_invoke_calls() -> Vec<InvokeCall> {
    MOCK_STATE.with(|state| state.borrow().calls.clone())
}

fn clear_mock_state() {
    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.calls.clear();
        s.next_response = None;
    });
}

fn get_last_invoke_call() -> Option<InvokeCall> {
    get_invoke_calls().into_iter().last()
}

#[derive(Debug, Clone)]
pub struct JsValue(serde_json::Value);

impl JsValue {
    pub fn as_string(&self) -> Option<String> {
        self.0.as_str().map(|s| s.to_string())
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.0.as_bool()
    }
}

mod serde_wasm_bindgen {
    use super::*;

    pub fn to_value<T: Serialize>(value: &T) -> Result<JsValue, String> {
        serde_json::to_value(value)
            .map(JsValue)
            .map_err(|e| e.to_string())
    }

    pub fn from_value<T: for<'de> Deserialize<'de>>(value: JsValue) -> Result<T, String> {
        serde_json::from_value(value.0).map_err(|e| e.to_string())
    }
}

pub async fn invoke(command: &str, args: JsValue) -> JsValue {
    let call = InvokeCall {
        command: command.to_string(),
        args: args.0.clone(),
    };

    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.calls.push(call);
        let response = s.next_response.take().unwrap_or(serde_json::Value::Null);
        JsValue(response)
    })
}

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

// Manually expanded client code (simulates macro output for testing)

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub async fn try_call_greet(name: &str) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&GreetArgs { name })
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let result = invoke("greet", args).await;
    result
        .as_string()
        .ok_or_else(|| "Expected string response".to_string())
}

pub async fn call_greet(name: &str) -> String {
    try_call_greet(name).await.unwrap()
}

#[derive(Serialize, Deserialize)]
struct AddNumbersArgs {
    a: i32,
    b: i32,
}

pub async fn try_call_add_numbers(a: i32, b: i32) -> Result<i32, String> {
    let args = serde_wasm_bindgen::to_value(&AddNumbersArgs { a, b })
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let result = invoke("add_numbers", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to deserialize number: {}", e))
}

pub async fn call_add_numbers(a: i32, b: i32) -> i32 {
    try_call_add_numbers(a, b).await.unwrap()
}

#[derive(Serialize, Deserialize)]
struct GetUserArgs {
    id: u64,
}

pub async fn try_call_get_user(id: u64) -> Result<Option<User>, String> {
    let args = serde_wasm_bindgen::to_value(&GetUserArgs { id })
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let result = invoke("get_user", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to deserialize response: {}", e))
}

pub async fn call_get_user(id: u64) -> Option<User> {
    try_call_get_user(id).await.unwrap()
}

#[derive(Serialize, Deserialize)]
struct CreateUserArgs {
    name: String,
    email: Option<String>,
}

pub async fn try_call_create_user(name: String, email: Option<String>) -> Result<User, String> {
    let args = serde_wasm_bindgen::to_value(&CreateUserArgs { name, email })
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let result = invoke("create_user", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to deserialize response: {}", e))
}

pub async fn call_create_user(name: String, email: Option<String>) -> User {
    try_call_create_user(name, email).await.unwrap()
}

#[derive(Serialize, Deserialize)]
struct CheckStatusArgs {
    user: User,
}

pub async fn try_call_check_status(user: User) -> Result<Status, String> {
    let args = serde_wasm_bindgen::to_value(&CheckStatusArgs { user })
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let result = invoke("check_status", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to deserialize response: {}", e))
}

pub async fn call_check_status(user: User) -> Status {
    try_call_check_status(user).await.unwrap()
}

#[derive(Serialize, Deserialize)]
struct IsValidArgs<'a> {
    input: &'a str,
}

pub async fn try_call_is_valid(input: &str) -> Result<bool, String> {
    let args = serde_wasm_bindgen::to_value(&IsValidArgs { input })
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let result = invoke("is_valid", args).await;
    result
        .as_bool()
        .ok_or_else(|| "Expected bool response".to_string())
}

pub async fn call_is_valid(input: &str) -> bool {
    try_call_is_valid(input).await.unwrap()
}

pub async fn try_call_noop() -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::Value::Null)
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let _result = invoke("noop", args).await;
    Ok(())
}

pub async fn call_noop() {
    try_call_noop().await.unwrap()
}

#[derive(Serialize, Deserialize)]
struct ProcessItemsArgs {
    items: Vec<String>,
}

pub async fn try_call_process_items(items: Vec<String>) -> Result<u64, String> {
    let args = serde_wasm_bindgen::to_value(&ProcessItemsArgs { items })
        .map_err(|e| format!("Failed to serialize arguments: {}", e))?;
    let result = invoke("process_items", args).await;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to deserialize number: {}", e))
}

pub async fn call_process_items(items: Vec<String>) -> u64 {
    try_call_process_items(items).await.unwrap()
}

// Tests

#[tokio::test]
async fn test_try_call_greet_invokes_correctly() {
    clear_mock_state();
    set_mock_response("Hello, World!");

    let result = try_call_greet("World").await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, World!");

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "greet");
    assert_eq!(call.args["name"], "World");
}

#[tokio::test]
async fn test_call_greet_returns_string() {
    clear_mock_state();
    set_mock_response("Hello, Tauri!");

    let result = call_greet("Tauri").await;
    assert_eq!(result, "Hello, Tauri!");
}

#[tokio::test]
async fn test_try_call_greet_with_special_characters() {
    clear_mock_state();
    set_mock_response("Hello, 世界!");

    let result = try_call_greet("世界").await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, 世界!");

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.args["name"], "世界");
}

#[tokio::test]
async fn test_try_call_add_numbers() {
    clear_mock_state();
    set_mock_response(42i32);

    let result = try_call_add_numbers(20, 22).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "add_numbers");
    assert_eq!(call.args["a"], 20);
    assert_eq!(call.args["b"], 22);
}

#[tokio::test]
async fn test_call_add_numbers() {
    clear_mock_state();
    set_mock_response(100i32);

    let result = call_add_numbers(50, 50).await;
    assert_eq!(result, 100);
}

#[tokio::test]
async fn test_try_call_get_user_some() {
    clear_mock_state();
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: Some("alice@example.com".to_string()),
    };
    set_mock_response(Some(user.clone()));

    let result = try_call_get_user(1).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(user));

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "get_user");
    assert_eq!(call.args["id"], 1);
}

#[tokio::test]
async fn test_try_call_get_user_none() {
    clear_mock_state();
    set_mock_response(None::<User>);

    let result = try_call_get_user(999).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[tokio::test]
async fn test_try_call_create_user() {
    clear_mock_state();
    let expected_user = User {
        id: 42,
        name: "Bob".to_string(),
        email: Some("bob@example.com".to_string()),
    };
    set_mock_response(expected_user.clone());

    let result = try_call_create_user("Bob".to_string(), Some("bob@example.com".to_string())).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_user);

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "create_user");
    assert_eq!(call.args["name"], "Bob");
    assert_eq!(call.args["email"], "bob@example.com");
}

#[tokio::test]
async fn test_try_call_create_user_without_email() {
    clear_mock_state();
    let expected_user = User {
        id: 43,
        name: "Charlie".to_string(),
        email: None,
    };
    set_mock_response(expected_user.clone());

    let result = try_call_create_user("Charlie".to_string(), None).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_user);

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.args["name"], "Charlie");
    assert!(call.args["email"].is_null());
}

#[tokio::test]
async fn test_try_call_check_status() {
    clear_mock_state();
    set_mock_response(Status::Active);

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: Some("alice@example.com".to_string()),
    };

    let result = try_call_check_status(user.clone()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Status::Active);

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "check_status");
    assert_eq!(call.args["user"]["id"], 1);
    assert_eq!(call.args["user"]["name"], "Alice");
}

#[tokio::test]
async fn test_try_call_is_valid_true() {
    clear_mock_state();
    set_mock_response(true);

    let result = try_call_is_valid("valid input").await;

    assert!(result.is_ok());
    assert!(result.unwrap());

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "is_valid");
    assert_eq!(call.args["input"], "valid input");
}

#[tokio::test]
async fn test_try_call_is_valid_false() {
    clear_mock_state();
    set_mock_response(false);

    let result = try_call_is_valid("").await;

    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_try_call_noop() {
    clear_mock_state();
    set_mock_response(serde_json::Value::Null);

    let result = try_call_noop().await;

    assert!(result.is_ok());

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "noop");
}

#[tokio::test]
async fn test_call_noop() {
    clear_mock_state();
    set_mock_response(serde_json::Value::Null);

    call_noop().await;

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "noop");
}

#[tokio::test]
async fn test_try_call_process_items() {
    clear_mock_state();
    set_mock_response(3u64);

    let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let result = try_call_process_items(items).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.command, "process_items");
    assert_eq!(call.args["items"], serde_json::json!(["a", "b", "c"]));
}

#[tokio::test]
async fn test_try_call_process_items_empty() {
    clear_mock_state();
    set_mock_response(0u64);

    let result = try_call_process_items(vec![]).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let call = get_last_invoke_call().unwrap();
    assert_eq!(call.args["items"], serde_json::json!([]));
}

#[tokio::test]
async fn test_try_call_greet_wrong_response_type() {
    clear_mock_state();
    set_mock_response(42i32);

    let result = try_call_greet("World").await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Expected string response"));
}

#[tokio::test]
async fn test_try_call_is_valid_wrong_response_type() {
    clear_mock_state();
    set_mock_response("not a bool");

    let result = try_call_is_valid("test").await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Expected bool response"));
}

#[tokio::test]
async fn test_try_call_add_numbers_wrong_response_type() {
    clear_mock_state();
    set_mock_response("not a number");

    let result = try_call_add_numbers(1, 2).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to deserialize"));
}

#[tokio::test]
async fn test_try_call_get_user_invalid_response() {
    clear_mock_state();
    set_mock_response(serde_json::json!({"invalid": "structure"}));

    let result = try_call_get_user(1).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_multiple_invoke_calls_recorded() {
    clear_mock_state();

    set_mock_response("Hello!");
    let _ = try_call_greet("First").await;

    set_mock_response("Hi!");
    let _ = try_call_greet("Second").await;

    set_mock_response(10i32);
    let _ = try_call_add_numbers(5, 5).await;

    let calls = get_invoke_calls();
    assert_eq!(calls.len(), 3);

    assert_eq!(calls[0].command, "greet");
    assert_eq!(calls[0].args["name"], "First");

    assert_eq!(calls[1].command, "greet");
    assert_eq!(calls[1].args["name"], "Second");

    assert_eq!(calls[2].command, "add_numbers");
    assert_eq!(calls[2].args["a"], 5);
    assert_eq!(calls[2].args["b"], 5);
}

#[tokio::test]
async fn test_args_serialization_format() {
    clear_mock_state();
    set_mock_response("ok");

    let _ = try_call_greet("test").await;
    let greet_call = get_last_invoke_call().unwrap();

    assert!(greet_call.args.is_object());
    assert!(greet_call.args.get("name").is_some());
}

#[tokio::test]
async fn test_complex_type_serialization() {
    clear_mock_state();
    set_mock_response(Status::Pending);

    let user = User {
        id: 123,
        name: "Test User".to_string(),
        email: Some("test@example.com".to_string()),
    };

    let _ = try_call_check_status(user).await;
    let call = get_last_invoke_call().unwrap();

    let user_arg = &call.args["user"];
    assert_eq!(user_arg["id"], 123);
    assert_eq!(user_arg["name"], "Test User");
    assert_eq!(user_arg["email"], "test@example.com");
}

#[tokio::test]
async fn test_enum_response_deserialization() {
    clear_mock_state();

    for status in [Status::Active, Status::Inactive, Status::Pending] {
        clear_mock_state();
        set_mock_response(status.clone());

        let user = User {
            id: 1,
            name: "Test".to_string(),
            email: None,
        };

        let result = try_call_check_status(user).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), status);
    }
}
