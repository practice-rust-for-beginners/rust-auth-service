// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025  Aditya Pratap Bhuyan

mod handlers;
mod models;
mod routes;
mod state;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Router,
    serve,
};
use routes::create_router;
use state::AppState;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Shared in-memory state
    let state = AppState::new();
    let app = create_router(state.clone()); // important clone

    // Set up listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");

    println!("🚀 Server running at http://{addr}");

    // Serve requests using axum::serve() (works on axum 0.7+)
    serve(listener, app).await.expect("Server crashed");
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}