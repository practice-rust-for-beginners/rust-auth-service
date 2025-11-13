use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::models::*;
use crate::state::AppState;

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Json<String> {
    let mut users = state.users.lock().unwrap();
    if users.contains_key(&payload.username) {
        return Json("User already exists".to_string());
    }
    let pw_hash = hash(&payload.password, DEFAULT_COST).unwrap();
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username.clone(),
        password_hash: pw_hash,
    };
    users.insert(payload.username.clone(), user);
    Json("User created successfully".to_string())
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<String> {
    let users = state.users.lock().unwrap();
    if let Some(user) = users.get(&payload.username) {
        if verify(&payload.password, &user.password_hash).unwrap() {
            return Json("Login successful".to_string());
        } else {
            return Json("Invalid credentials".to_string());
        }
    }
    Json("No such user".to_string())
}

pub async fn password_reset_handler(
    State(state): State<AppState>,
    Json(payload): Json<PasswordResetRequest>,
) -> Json<String> {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(&payload.username) {
        user.password_hash = hash(&payload.new_password, DEFAULT_COST).unwrap();
        return Json("Password reset successful".to_string());
    }
    Json("User not found".to_string())
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<String> {
    let mut users = state.users.lock().unwrap();
    if users.remove(&payload.username).is_some() {
        return Json("User deleted".to_string());
    }
    Json("User not found".to_string())
}