use actix_web::{web, HttpResponse, Result};
use crate::models::{User, RegisterUserSchema, LoginUserSchema, UpdateUserSchema};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

// Mock in-memory storage for users (in a real application, you would use a database)
static mut USERS: Vec<User> = vec![];

// Get all users
pub async fn get_users() -> Result<HttpResponse> {
    let users: Vec<User> = unsafe { USERS.clone() };
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": users
    })))
}

// Get a single user by ID
pub async fn get_user(path: web::Path<String>) -> Result<HttpResponse> {
    let id_str = path.into_inner();
    let user_id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().json(json!({
            "status": "fail",
            "message": "Invalid user ID format"
        })))
    };

    let users: Vec<User> = unsafe { USERS.clone() };
    if let Some(user) = users.iter().find(|u| u.id == user_id) {
        Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": user
        })))
    } else {
        Ok(HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "User not found"
        })))
    }
}

// Create a new user
pub async fn create_user(user_data: web::Json<RegisterUserSchema>) -> Result<HttpResponse> {
    let new_user = User {
        id: Uuid::new_v4(),
        name: Some(user_data.name.clone()),
        email: Some(user_data.email.clone()),
        password: Some(user_data.password.clone()), // In a real app, you would hash this
        role: Some("user".to_string()),
        photo: None,
        verified: Some(false),
        favourite: Some(vec![]),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    unsafe {
        USERS.push(new_user.clone());
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": new_user
    })))
}

// Update a user
pub async fn update_user(path: web::Path<String>, user_data: web::Json<UpdateUserSchema>) -> Result<HttpResponse> {
    let id_str = path.into_inner();
    let user_id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().json(json!({
            "status": "fail",
            "message": "Invalid user ID format"
        })))
    };

    unsafe {
        if let Some(index) = USERS.iter().position(|u| u.id == user_id) {
            let user = &mut USERS[index];
            
            if let Some(ref name) = user_data.name {
                user.name = Some(name.clone());
            }
            if let Some(ref email) = user_data.email {
                user.email = Some(email.clone());
            }
            if let Some(ref role) = user_data.role {
                user.role = Some(role.clone());
            }
            if let Some(verified) = user_data.verified {
                user.verified = Some(verified);
            }
            if let Some(ref favourite) = user_data.favourite {
                user.favourite = Some(favourite.clone());
            }
            
            user.updated_at = Some(Utc::now());
            
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "data": user.clone()
            })))
        } else {
            Ok(HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": "User not found"
            })))
        }
    }
}

// Delete a user
pub async fn delete_user(path: web::Path<String>) -> Result<HttpResponse> {
    let id_str = path.into_inner();
    let user_id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::BadRequest().json(json!({
            "status": "fail",
            "message": "Invalid user ID format"
        })))
    };

    unsafe {
        if let Some(index) = USERS.iter().position(|u| u.id == user_id) {
            USERS.remove(index);
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "User deleted successfully"
            })))
        } else {
            Ok(HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": "User not found"
            })))
        }
    }
}