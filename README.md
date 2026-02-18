# 🌉 Tauri Bridge

[![Crates.io](https://img.shields.io/crates/v/tauri-bridge.svg)](https://crates.io/crates/tauri-bridge)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Generate type-safe Tauri commands and WASM client bindings from a single function definition.

## 📦 Installation

Add `tauri-bridge` to your `Cargo.toml`:

```toml
[dependencies]
tauri-bridge = "1.o"

# For backend (Tauri app)
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tauri = { version = "2", features = ["default"] }

# For WASM client
[target.'cfg(target_arch = "wasm32")'.dependencies]
serde = { version = "1", features = ["derive"] }
serde_wasm_bindgen = "0.6"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
```

Enable the appropriate features:

```toml
[features]
backend = []       # Enable for Tauri backend
wasm-client = []   # Enable for WASM frontend
```

## 🚀 Quick Start

### 1. Define Your Commands

Create a shared module with your bridged functions:

```rust
use tauri_bridge::tauri_bridge;

#[tauri_bridge]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Tauri.", name)
}

#[tauri_bridge]
pub async fn fetch_user(id: u64) -> Result<User, String> {
    // Your async logic here
    Ok(User { id, name: "Alice".into() })
}

#[tauri_bridge]
pub fn calculate(a: i32, b: i32, operation: Operation) -> i32 {
    match operation {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
    }
}
```

### 2. Register Backend Commands

In your Tauri app's `main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            fetch_user,
            calculate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Call from WASM Frontend

In your WASM frontend, provide the invoke function and use the generated bindings:

```rust
use your_lib_crate::greet;

// Use the generated client functions
async fn example() {
    // Type-safe calls with automatic serialization
    let greeting: String = call_greet("World").await;
    
    // Or use the Result-returning variant
    match try_call_fetch_user(42).await {
        Ok(user) => println!("Got user: {:?}", user),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Enums work seamlessly
    let result = call_calculate(10, 5, Operation::Add).await;
}
```

## 📚 Examples

### Basic Types

```rust
#[tauri_bridge]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tauri_bridge]
pub fn is_valid(input: &str) -> bool {
    !input.is_empty()
}

#[tauri_bridge]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
```

### Async Functions

```rust
#[tauri_bridge]
pub async fn fetch_data(url: String) -> Result<String, String> {
    // Async operations work seamlessly
    reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}
```

### Complex Types

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Active,
    Inactive,
    Pending { reason: String },
}

#[tauri_bridge]
pub fn create_user(name: String, email: Option<String>) -> User {
    User {
        id: rand::random(),
        name,
        email,
    }
}

#[tauri_bridge]
pub fn get_status(user: User) -> Status {
    if user.email.is_some() {
        Status::Active
    } else {
        Status::Pending { reason: "Email required".into() }
    }
}
```

## 🧪 Testing

Run all tests with:

```bash
# Unit tests only
cargo test

# With all features (includes client integration tests)
cargo test --all-features
```
