// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;

use std::sync::{Arc, Mutex};

use components::auth::*;
use components::gacha::*;
use components::statistics::*;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Config {}

pub struct AppState {
    token: Arc<Mutex<Option<String>>>,
    client: reqwest::Client,
}

impl AppState {
    pub fn update_token(&self, new_token: String) {
        let mut token = self.token.lock().unwrap();
        *token = Some(new_token);
    }

    pub fn get_token(&self) -> Option<String> {
        let token = self.token.lock().unwrap();
        token.clone()
    }
}

#[derive(Default)]
pub struct DatabaseState {
    connection: Arc<Mutex<Option<rusqlite::Connection>>>,
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        #[cfg(debug)]
        println!("[DEBUG] {}", $msg);
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(debug)]
        println!("[DEBUG] {}", format!($fmt, $($arg)*));
    };
}

fn main() {
    let app_state = AppState {
        token: Arc::new(Mutex::new(None)),
        client: reqwest::Client::new(),
    };

    tauri::Builder::default()
        .setup(|app|{
            let data_dir = app.path_resolver().app_data_dir().unwrap();
            if let Err(e) = std::fs::create_dir_all(&data_dir) {
                panic!("Failed to create data directory: {}", e);
            }
            Ok(())
        })
        .manage(app_state)
        .manage(DatabaseState::default())
        .invoke_handler(tauri::generate_handler![
            save_auth_credentials,
            get_auth_credentials,
            login,
            get_all_gacha,
            get_gacha_in_pool,
            calculate_statistics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
