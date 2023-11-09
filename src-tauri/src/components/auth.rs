use std::{fs, io::Write};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use super::HandlerResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub phone: String,
    pub password: String,
}

pub fn get_saved_cred_file(app: &AppHandle) -> String {
    let data_dir = app.path_resolver().app_data_dir().unwrap();
    let cred_file = data_dir.join("auth.cred");
    cred_file.to_str().unwrap().to_string()
}

#[tauri::command]
pub fn save_auth_credentials(app: AppHandle, credential: Credential) -> HandlerResult<()> {
    let mut file = std::fs::File::create(get_saved_cred_file(&app))?;

    let data = serde_json::to_string(&credential)?.as_bytes().to_vec();
    file.write_all(data.as_slice())?;

    Ok(())
}

#[tauri::command]
pub fn get_auth_credentials(app: AppHandle) -> HandlerResult<Option<Credential>> {
    let cred_file_path = get_saved_cred_file(&app);
    if !fs::metadata(&cred_file_path).is_ok() {
        return Ok(None);
    }

    let file = std::fs::File::open(cred_file_path)?;
    let credential = serde_json::from_reader(file).ok();

    Ok(credential)
}
