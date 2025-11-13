use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::User;


#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<String, User>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}