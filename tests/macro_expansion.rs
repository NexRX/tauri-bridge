//! Macro expansion tests - verifies generated code compiles correctly.

use tauri_bridge::tauri_bridge;

#[tauri_bridge]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri_bridge]
pub fn get_version() -> String {
    "1.0.0".to_string()
}

#[tauri_bridge]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tauri_bridge]
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[tauri_bridge]
pub fn maybe_greet(name: Option<String>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => "Hello, stranger!".to_string(),
    }
}

#[tauri_bridge]
pub fn sum_numbers(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

#[tauri_bridge]
pub fn log_message(message: &str) {
    println!("LOG: {}", message);
}

#[tauri_bridge]
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[tauri_bridge]
pub fn concat(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

#[cfg(all(feature = "backend", not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn test_greet_function_exists() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_get_version_exists() {
        assert_eq!(get_version(), "1.0.0");
    }

    #[test]
    fn test_add_function_exists() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_is_even_function() {
        assert!(is_even(4));
        assert!(!is_even(3));
    }

    #[test]
    fn test_maybe_greet_function() {
        assert_eq!(maybe_greet(Some("Alice".to_string())), "Hello, Alice!");
        assert_eq!(maybe_greet(None), "Hello, stranger!");
    }

    #[test]
    fn test_sum_numbers_function() {
        assert_eq!(sum_numbers(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_numbers(vec![]), 0);
    }

    #[test]
    fn test_log_message_compiles() {
        log_message("test message");
    }

    #[test]
    fn test_divide_function() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err("Cannot divide by zero".to_string()));
    }

    #[test]
    fn test_concat_function() {
        assert_eq!(concat("Hello, ", "World!"), "Hello, World!");
    }

    #[test]
    fn test_macro_generates_valid_code() {
        let _ = greet("test");
    }
}

#[cfg(not(all(feature = "backend", not(target_arch = "wasm32"))))]
mod tests {
    #[test]
    fn test_macro_generates_valid_code() {
        // Test file compiled successfully
    }
}
