use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::DatabaseState;

const DB_FILE: &str = "gachadb";
const GACHA_TABLE: &str = "gacha";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GachaChar {
    pub name: String,
    pub rarity: u8,
    #[serde(rename = "isNew")]
    pub is_new: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gacha {
    #[serde(rename = "ts")]
    pub timestamp: i64,
    pub pool: String,
    pub chars: Vec<GachaChar>,
}

impl DatabaseState {
    pub fn init_db(&self, app: &AppHandle) -> rusqlite::Result<()> {
        let data_dir = app.path_resolver().app_data_dir().unwrap();
        let db_file = data_dir.join(DB_FILE);
        let db_file = db_file.to_str().unwrap();
        let s_db = Connection::open(db_file)?;
        let create_table_stmt = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            ts INTEGER NOT NULL PRIMARY KEY,
            pool TEXT NOT NULL,
            chars TEXT NOT NULL
        )",
            GACHA_TABLE
        );
        s_db.execute(&create_table_stmt, params![])?;

        let mut conn = self.connection.lock().unwrap();
        *conn = Some(s_db);

        Ok(())
    }

    pub fn spawn_write_thread(&self, recv: Receiver<Vec<Gacha>>) -> JoinHandle<()> {
        let conn = self.connection.clone();
        thread::spawn(move || {
            let conn = conn.lock().unwrap();
            let conn = conn.as_ref().unwrap();
            while let Ok(gacha) = recv.recv() {
                let stmt = format!(
                    "REPLACE INTO {} (ts, pool, chars) VALUES (?1, ?2, ?3)",
                    GACHA_TABLE
                );
                let mut stmt = conn.prepare(&stmt).unwrap();
                for gacha in gacha {
                    let chars = serde_json::to_string(&gacha.chars).unwrap();
                    stmt.execute(params![gacha.timestamp, gacha.pool, chars])
                        .unwrap();
                }
            }
        })
    }

    pub fn get_all_gacha(&self) -> rusqlite::Result<Vec<Gacha>> {
        let conn = self.connection.lock().unwrap();
        let conn = conn.as_ref().unwrap();
        let mut stmt = conn.prepare(&format!("SELECT * FROM {} ORDER BY ts DESC", GACHA_TABLE))?;
        let gacha = stmt
            .query_map(params![], |row| {
                let chars: String = row.get(2)?;
                Ok(Gacha {
                    timestamp: row.get(0)?,
                    pool: row.get(1)?,
                    chars: serde_json::from_str(&chars).unwrap(),
                })
            })?
            .collect::<rusqlite::Result<Vec<Gacha>>>()?;

        Ok(gacha)
    }

    pub fn get_all_gacha_in_pool(&self, pool: &str) -> rusqlite::Result<Vec<Gacha>> {
        let conn = self.connection.lock().unwrap();
        let conn = conn.as_ref().unwrap();
        let mut stmt = conn.prepare(&format!(
            "SELECT * FROM {} WHERE pool = ?1 ORDER BY ts DESC",
            GACHA_TABLE
        ))?;
        let gacha = stmt
            .query_map(params![pool], |row| {
                let chars: String = row.get(2)?;
                Ok(Gacha {
                    timestamp: row.get(0)?,
                    pool: row.get(1)?,
                    chars: serde_json::from_str(&chars).unwrap(),
                })
            })?
            .collect::<rusqlite::Result<Vec<Gacha>>>()?;

        Ok(gacha)
    }
}
