//! Complex types example - demonstrates structs, enums, and nested types.
//!
//! Run with: cargo run --example complex_types

use serde::{Deserialize, Serialize};
use tauri_bridge::tauri_bridge;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: Option<String>,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Guest,
    User,
    Moderator,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub author_id: u64,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub status: PostStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostStatus {
    Draft,
    PendingReview,
    Published { published_at: u64 },
    Rejected { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
}

#[tauri_bridge]
pub fn create_user(username: String, email: Option<String>, role: Role) -> User {
    User {
        id: 1,
        username,
        email,
        role,
    }
}

#[tauri_bridge]
pub fn get_user(id: u64) -> Option<User> {
    if id == 1 {
        Some(User {
            id: 1,
            username: "alice".to_string(),
            email: Some("alice@example.com".to_string()),
            role: Role::Admin,
        })
    } else {
        None
    }
}

#[tauri_bridge]
pub fn check_permission(user: User, required_role: Role) -> bool {
    let user_level = match user.role {
        Role::Guest => 0,
        Role::User => 1,
        Role::Moderator => 2,
        Role::Admin => 3,
    };
    let required_level = match required_role {
        Role::Guest => 0,
        Role::User => 1,
        Role::Moderator => 2,
        Role::Admin => 3,
    };
    user_level >= required_level
}

#[tauri_bridge]
pub fn create_post(author_id: u64, title: String, content: String, tags: Vec<String>) -> Post {
    Post {
        id: 1,
        author_id,
        title,
        content,
        tags,
        status: PostStatus::Draft,
    }
}

#[tauri_bridge]
pub fn update_post_status(post: Post, new_status: PostStatus) -> Post {
    Post {
        status: new_status,
        ..post
    }
}

#[tauri_bridge]
pub fn get_posts(page: u32, per_page: u32) -> PaginatedResponse<Post> {
    let posts = vec![
        Post {
            id: 1,
            author_id: 1,
            title: "Hello World".to_string(),
            content: "This is my first post!".to_string(),
            tags: vec!["intro".to_string(), "hello".to_string()],
            status: PostStatus::Published {
                published_at: 1699900000,
            },
        },
        Post {
            id: 2,
            author_id: 1,
            title: "Second Post".to_string(),
            content: "More content here...".to_string(),
            tags: vec!["update".to_string()],
            status: PostStatus::PendingReview,
        },
    ];
    PaginatedResponse {
        items: posts,
        page,
        per_page,
        total: 2,
    }
}

#[tauri_bridge]
pub fn search_by_tags(tags: Vec<&str>) -> Vec<Post> {
    let all_posts = vec![Post {
        id: 1,
        author_id: 1,
        title: "Hello World".to_string(),
        content: "This is my first post!".to_string(),
        tags: vec!["intro".to_string(), "hello".to_string()],
        status: PostStatus::Published {
            published_at: 1699900000,
        },
    }];
    all_posts
        .into_iter()
        .filter(|post| tags.iter().any(|tag| post.tags.contains(&tag.to_string())))
        .collect()
}

#[tauri_bridge]
pub fn validate_user(user: User) -> Result<User, String> {
    if user.username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    if let Some(ref email) = user.email
        && !email.contains('@')
    {
        return Err("Invalid email format".to_string());
    }
    Ok(user)
}

fn main() {
    let user = create_user(
        "bob".to_string(),
        Some("bob@example.com".to_string()),
        Role::Moderator,
    );
    println!("Created user: {:?}", user);

    println!(
        "Can moderate: {}",
        check_permission(user.clone(), Role::Moderator)
    );
    println!("Can admin: {}", check_permission(user.clone(), Role::Admin));

    let post = create_post(
        user.id,
        "My First Post".to_string(),
        "Hello, this is the content!".to_string(),
        vec!["rust".to_string(), "tauri".to_string()],
    );
    println!("Created post: {:?}", post);

    let published = update_post_status(
        post,
        PostStatus::Published {
            published_at: 1699900000,
        },
    );
    println!("Published: {:?}", published);

    let page = get_posts(1, 10);
    println!("Got {} posts", page.items.len());

    match validate_user(user) {
        Ok(u) => println!("Valid: {:?}", u),
        Err(e) => println!("Error: {}", e),
    }
}
