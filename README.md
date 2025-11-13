
# 🦀 Rust Auth Service

> A REST‑based user authentication service built in **Rust** using **Axum 0.7** and an **in‑memory database**, including a simple HTML UI for signup, login, password reset, and landing page.

**Maintainer:** [Aditya Pratap Bhuyan](https://linkedin.com/in/adityabhuyan)  
**Repository:** https://github.com/practice-rust-for-beginners/rust-auth-service  
**License:** GNU GPL v3 (or later)

---

## 📂 Project Structure

```
rust-auth-service/
├── Cargo.toml
├── .gitignore
├── LICENSE
├── README.md
├── src/
│   ├── main.rs
│   ├── handlers.rs        # REST endpoints
│   ├── models.rs          # Data models (User, requests)
│   ├── routes.rs          # Router setup
│   └── state.rs           # Shared in-memory state
└── static/
    ├── index.html         # Login form
    ├── signup.html        # Signup form
    ├── reset_password.html# Password reset form
    └── landing.html       # Post‑login landing page
```

---

## ⚙️ Features

- ✅ **User Signup** (`/signup`)
- 🔑 **User Login** (`/login`)
- 🔁 **Password Reset** (`/reset_password`)
- ❌ **User Delete** (`/delete_user`)
- 🧱 **In‑memory DB** using Rust `HashMap`
- 🌐 **HTML UI** served via `/static`
- 🐛 **Unit tests** using `tokio::test`
- 🪪 **GNU GPL v3 License**

---

## 🚀 Getting Started

### 1️⃣ Install Rust Toolchain
```bash
curl https://sh.rustup.rs -sSf | sh
```

### 2️⃣ Clone the repository
```bash
git clone https://github.com/practice-rust-for-beginners/rust-auth-service.git
cd rust-auth-service
```

### 3️⃣ Build and run
```bash
cargo run
```

Server output:
```
🚀 Server running at http://127.0.0.1:8080
```

Open [http://localhost:8080](http://localhost:8080) in your browser.

---

## 🌐 User Interface

| Page | File | Purpose |
|------|------|----------|
| `/index.html` | `static/index.html` | Login |
| `/signup.html` | `static/signup.html` | Register new users |
| `/reset_password.html` | `static/reset_password.html` | Reset forgotten password |
| `/landing.html` | `static/landing.html` | Simple landing page shown after successful login |

After login success, browser JavaScript redirects automatically to `/landing.html`.

---

## 🧠 In‑Memory Database

All user information is stored in an in‑memory HashMap wrapped in a thread‑safe `Arc<Mutex<...>>`.  
Once the process stops, users are erased (no persistence).

---

## 📡 REST API

| Endpoint | Method | Description |
|-----------|--------|-------------|
| `/signup` | POST | Create a new user |
| `/login` | POST | Verify credentials |
| `/reset_password` | POST | Reset existing password |
| `/delete_user` | POST | Delete user from DB |

**Example JSON requests**

```bash
# Signup
curl -X POST http://localhost:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"pass"}'

# Login
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"pass"}'

# Reset password
curl -X POST http://localhost:8080/reset_password \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","new_password":"newpass"}'

# Delete user
curl -X POST http://localhost:8080/delete_user \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"newpass"}'
```

---

## 🧪 Testing

### 🧩 Unit Tests
Each handler can be tested using `tokio::test`.  
Example (in `handlers.rs`):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use crate::state::AppState;
    use tokio;

    #[tokio::test]
    async fn signup_and_login_flow() {
        let state = AppState::new();

        let signup = signup_handler(
            State(state.clone()),
            Json(SignupRequest { username: "bob".into(), password: "pwd".into() }),
        )
        .await;
        assert!(signup.0.contains("successfully"));

        let login = login_handler(
            State(state.clone()),
            Json(LoginRequest { username: "bob".into(), password: "pwd".into() }),
        )
        .await;
        assert!(login.0.contains("successful"));
    }
}
```

Run:
```bash
cargo test
```

---

## 🧱 Tech Stack

| Layer | Technology | Notes |
|--------|-------------|-------|
| **Language** | Rust 2021 | Safe and fast backend |
| **Framework** | [Axum 0.7](https://docs.rs/axum) | Async web framework |
| **Passwords** | [bcrypt 0.15](https://crates.io/crates/bcrypt) | Secure hashing |
| **IDs** | [uuid 1.x](https://crates.io/crates/uuid) | Unique identifiers |
| **Web** | HTML + Fetch API | Minimal static UI |
| **Concurrency** | `tokio`, `Arc<Mutex<>>` | Async runtime |

---

## 🧠 Common Issues & Fixes

| Symptom | Root Cause | Solution |
|----------|------------|----------|
| Signup succeeds but login fails | Each route had its own state instance | Ensure `AppState` is cloned and shared across router |
| Can't compile `"ServeDir"` | Use feature `fs` in `tower-http` | `tower-http = { version = "0.5", features = ["cors","trace","fs"] }` |
| `"Server"` import errors | Use `axum::serve(listener, app)` on Axum 0.7 | Remove direct `hyper` usage |
| Password reset not updating | Forgot to hash new password | Use `user.password_hash = hash(...)` in handler |

---

## 🔒 Security Notes

- For demonstration only; no persistent storage or session management.  
- For production, add one of:
  - Database (`sqlx`, `diesel`)
  - Session/Cookie or JWT auth (`axum-extra`, `jsonwebtoken`)
  - HTTPS reverse‑proxy (e.g., Nginx/Caddy)

---

## 🧾 Example Workflow

1. Launch server (`cargo run`)
2. Open [http://localhost:8080](http://localhost:8080)
3. **Signup** a new user
4. **Login** → redirected to **landing page**
5. **Logout** → returns to login

---

## 💻 Development Scripts

| Task | Command |
|------|----------|
| Run in dev mode | `cargo run` |
| Build release executable | `cargo build --release` |
| Run all tests | `cargo test` |
| Clean artifacts | `cargo clean` |

---

## 🧹 .gitignore Highlights

```
# Cargo
target/
Cargo.lock

# IDEs
.idea/
.vscode/
*.iml

# OS
.DS_Store
Thumbs.db
```

---

## 🪪 License

**GNU General Public License v3.0 (or later)**  
Copyright © 2025 [Aditya Pratap Bhuyan](https://linkedin.com/in/adityabhuyan)

This project is free software: you can redistribute it and/or modify it under the terms of the **GNU GPL v3** as published by the Free Software Foundation.  
See the [LICENSE](./LICENSE) file for full terms.

---

## 🧭 Attribution

Created and maintained by **Aditya Pratap Bhuyan**  
→ [LinkedIn Profile ↗](https://linkedin.com/in/adityabhuyan)  
→ [GitHub Organization ↗](https://github.com/practice-rust-for-beginners)

---

### 🧡 Happy Rusting!
