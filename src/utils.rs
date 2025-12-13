use actix_web::{HttpResponse, Result};
use serde::Serialize;

// Generic response function
pub fn create_response<T: Serialize>(status: &str, data: T) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": status,
        "data": data
    })))
}

// Error response function
pub fn create_error_response(status: &str, message: &str) -> Result<HttpResponse> {
    Ok(HttpResponse::BadRequest().json(serde_json::json!({
        "status": status,
        "message": message
    })))
}

// Success response function
pub fn create_success_response(message: &str) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": message
    })))
}