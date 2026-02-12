use serde::{Deserialize, Serialize};
use validator::Validate;

/// User model for database queries - returned from SELECT operations
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// Request model for creating a new user - with validation
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Request model for updating a user - with validation
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}
