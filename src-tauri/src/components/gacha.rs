use std::sync::mpsc;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::{AppState, DatabaseState};

use super::{
    auth::Credential,
    internal_db::{Gacha, GachaChar},
    HandlerExecutionError, HandlerResult,
};

const TOKEN_URL: &str = "https://as.hypergryph.com/user/auth/v1/token_by_phone_password";
const GACHA_URL: &str = "https://ak.hypergryph.com/user/api/inquiry/gacha";

#[derive(Debug, Serialize)]
struct GetTokenRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Debug)]
pub struct TokenResponse {
    pub status: bool,
    pub token: String,
}

impl<'de> Deserialize<'de> for TokenResponse {
    fn deserialize<D>(deserializer: D) -> Result<TokenResponse, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let status = map
            .remove("status")
            .ok_or(serde::de::Error::missing_field("status"))?
            .as_i64()
            .ok_or(serde::de::Error::missing_field("status"))?;
        let status = status == 0;
        let data = map
            .remove("data")
            .ok_or(serde::de::Error::missing_field("data"))?;
        let data = data
            .as_object()
            .ok_or(serde::de::Error::missing_field("data"))?;
        let token = data
            .get("token")
            .ok_or(serde::de::Error::missing_field("token"))?
            .as_str()
            .ok_or(serde::de::Error::missing_field("token"))?
            .to_string();
        Ok(TokenResponse { status, token })
    }
}

async fn get_token(
    client: &Client,
    get_token_request: GetTokenRequest,
) -> reqwest::Result<TokenResponse> {
    client
        .post(TOKEN_URL)
        .json(&get_token_request)
        .send()
        .await?
        .json()
        .await
}

#[tauri::command]
pub async fn login(
    app_state: tauri::State<'_, AppState>,
    credential: Credential,
) -> HandlerResult<()> {
    let client = &app_state.client;
    let get_token_request = GetTokenRequest {
        phone: credential.phone,
        password: credential.password,
    };
    let token_response = get_token(client, get_token_request).await?;
    if !token_response.status {
        return Err(HandlerExecutionError::UserError("Login failed".to_string()));
    }
    app_state.update_token(token_response.token);
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub current: u32,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct GachaData {
    pub list: Vec<Gacha>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct GachaResponse {
    pub code: u32,
    pub data: GachaData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SingleGacha {
    pub timestamp: i64,
    pub pool: String,
    pub character: GachaChar,
}

impl SingleGacha {
    pub fn from_gacha(gacha: &Gacha) -> Vec<SingleGacha> {
        let mut single_gacha = Vec::new();
        for character in &gacha.chars {
            single_gacha.push(SingleGacha {
                timestamp: gacha.timestamp,
                pool: gacha.pool.clone(),
                character: character.clone(),
            });
        }
        single_gacha
    }
}

async fn get_gacha(app_state: &AppState, page: u32) -> HandlerResult<GachaResponse> {
    let token = app_state
        .get_token()
        .ok_or(HandlerExecutionError::UserError(
            "Not logged in".to_string(),
        ))?;
    let client = &app_state.client;
    let gacha_response = client
        .get(GACHA_URL)
        .query(&[("token", token), ("page", format!("{}", page))])
        .send()
        .await?
        .json::<GachaResponse>()
        .await?;

    Ok(gacha_response)
}

#[tauri::command]
pub async fn get_all_gacha(
    app_state: tauri::State<'_, AppState>,
    database_state: tauri::State<'_, DatabaseState>,
    app: AppHandle,
) -> HandlerResult<Vec<SingleGacha>> {
    database_state.init_db(&app)?;

    let mut page = 1;
    let mut total_page = 0;

    let (sender, receiver) = mpsc::channel::<Vec<Gacha>>();

    let thread = database_state.spawn_write_thread(receiver);

    loop {
        let gacha_response = get_gacha(&app_state, page).await?;
        let gacha_data = gacha_response.data;

        if total_page == 0 {
            total_page = gacha_data.pagination.total / 10;
            if gacha_data.pagination.total % 10 != 0 {
                total_page += 1;
            }
        }

        sender.send(gacha_data.list)?;

        if gacha_data.pagination.current == total_page {
            break;
        }
        page += 1;
    }

    drop(sender);

    thread.join().unwrap();

    let all_gacha = database_state.get_all_gacha()?;
    let ret = all_gacha
        .iter()
        .map(|x| SingleGacha::from_gacha(x))
        .flatten()
        .collect();

    Ok(ret)
}

#[tauri::command]
pub fn get_gacha_in_pool(
    database_state: tauri::State<'_, DatabaseState>,
    pool: String,
) -> HandlerResult<Vec<SingleGacha>> {
    let all_gacha = database_state.get_all_gacha_in_pool(&pool)?;
    let ret = all_gacha
        .iter()
        .map(|x| SingleGacha::from_gacha(x))
        .flatten()
        .collect();

    Ok(ret)
}
