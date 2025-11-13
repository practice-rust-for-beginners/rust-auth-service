// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025  Aditya Pratap Bhuyan

use axum::routing::post;
use axum::Router;
use tower_http::services::fs::ServeDir;

use crate::{handlers::*, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .route("/reset_password", post(password_reset_handler)) // ✅ fixed name
        .route("/delete_user", post(delete_user_handler))
        .nest_service("/", ServeDir::new("static"))
        .with_state(state)
}