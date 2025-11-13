//! Integration tests for rust-auth-service
//! Validates all REST endpoints (signup, login, reset_password, delete_user).
//!
//! Run these tests with: `cargo test`

use axum::extract::State;
use axum::Json;
use rust_auth_service::{
    handlers::{
        delete_user_handler, login_handler, password_reset_handler, signup_handler,
    },
    models::{LoginRequest, PasswordResetRequest, SignupRequest},
    state::AppState,
};

#[tokio::test]
async fn test_user_signup_and_login_success() {
    let state = AppState::new();

    // 1️⃣ Signup
    let signup = signup_handler(
        State(state.clone()),
        Json(SignupRequest {
            username: "test1".to_string(),
            password: "1234".to_string(),
        }),
    )
    .await;
    assert!(
        signup.0.contains("success"),
        "Signup failed => {:?}",
        signup.0
    );

    // 2️⃣ Login
    let login = login_handler(
        State(state.clone()),
        Json(LoginRequest {
            username: "test1".to_string(),
            password: "1234".to_string(),
        }),
    )
    .await;
    assert!(
        login.0.contains("Login successful"),
        "Login failed => {:?}",
        login.0
    );
}

#[tokio::test]
async fn test_duplicate_signup_should_fail() {
    let state = AppState::new();

    // Create same user twice
    let first = signup_handler(
        State(state.clone()),
        Json(SignupRequest {
            username: "test2".into(),
            password: "pass123".into(),
        }),
    )
    .await;
    let second = signup_handler(
        State(state.clone()),
        Json(SignupRequest {
            username: "test2".into(),
            password: "pass123".into(),
        }),
    )
    .await;

    assert!(first.0.contains("success"), "First signup failed: {}", first.0);
    assert!(
        second.0.contains("exists") || second.0.contains("already"),
        "Duplicate signup not rejected: {}",
        second.0
    );
}

#[tokio::test]
async fn test_wrong_password_login_should_fail() {
    let state = AppState::new();

    // Signup first
    signup_handler(
        State(state.clone()),
        Json(SignupRequest {
            username: "wrongpass_user".to_string(),
            password: "password".to_string(),
        }),
    )
    .await;

    // Try login with wrong password
    let res = login_handler(
        State(state.clone()),
        Json(LoginRequest {
            username: "wrongpass_user".to_string(),
            password: "incorrect".to_string(),
        }),
    )
    .await;

    assert!(
        res.0.contains("Incorrect") || res.0.contains("Invalid"),
        "Expected login failure, got: {}",
        res.0
    );
}

#[tokio::test]
async fn test_password_reset_flow() {
    let state = AppState::new();

    // 1️⃣ Signup
    signup_handler(
        State(state.clone()),
        Json(SignupRequest {
            username: "reset_user".into(),
            password: "oldpass".into(),
        }),
    )
    .await;

    // 2️⃣ Reset password
    let reset = password_reset_handler(
        State(state.clone()),
        Json(PasswordResetRequest {
            username: "reset_user".into(),
            new_password: "newpass".into(),
        }),
    )
    .await;

    assert!(
        reset.0.contains("successful"),
        "Password reset failed: {}",
        reset.0
    );

    // 3️⃣ Try login with old password (should fail)
    let old_login = login_handler(
        State(state.clone()),
        Json(LoginRequest {
            username: "reset_user".into(),
            password: "oldpass".into(),
        }),
    )
    .await;
    assert!(
        old_login.0.contains("Incorrect") || old_login.0.contains("Invalid"),
        "Old password should no longer work: {}",
        old_login.0
    );

    // 4️⃣ Try login with new password (should pass)
    let new_login = login_handler(
        State(state.clone()),
        Json(LoginRequest {
            username: "reset_user".into(),
            password: "newpass".into(),
        }),
    )
    .await;
    assert!(
        new_login.0.contains("Login successful"),
        "Login with new password failed: {}",
        new_login.0
    );
}

#[tokio::test]
async fn test_delete_user_flow() {
    let state = AppState::new();

    // 1️⃣ Signup
    signup_handler(
        State(state.clone()),
        Json(SignupRequest {
            username: "todelete".into(),
            password: "12345".into(),
        }),
    )
    .await;

    // 2️⃣ Delete user
    let delete = delete_user_handler(
        State(state.clone()),
        Json(LoginRequest {
            username: "todelete".into(),
            password: "12345".into(),
        }),
    )
    .await;
    assert!(
        delete.0.contains("deleted"),
        "Delete failed => {:?}",
        delete.0
    );

    // 3️⃣ Ensure login fails now
    let login = login_handler(
        State(state.clone()),
        Json(LoginRequest {
            username: "todelete".into(),
            password: "12345".into(),
        }),
    )
    .await;
    assert!(
        login.0.contains("not found"),
        "User should be deleted, got: {}",
        login.0
    );
}
