use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Datelike};
use serde::Serialize;

use crate::components::HandlerExecutionError;

use super::{gacha::SingleGacha, HandlerResult};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalStatistics {
    pub total: u64,
    pub stars_count: [u64; 4],
    pub stars_percentage: [f64; 4], // 0 to 100.00
    pub all_pools: Vec<String>,
    pub pools_count: Vec<u64>,
    pub water_place: Vec<u64>,
    pub months_count: Vec<u64>,
    pub all_months: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolStatistics {
    pub all_pools: Vec<String>,
    pub pool_name: String,
    pub total: u64,
    pub stars_count: [u64; 4],
    pub stars_percentage: [f64; 4], // 0 to 100.00
    pub water_place: u64,
    pub months_count: Vec<u64>,
    pub all_months: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Statistics {
    Total(TotalStatistics),
    Pool(PoolStatistics),
}

impl Statistics {
    pub fn new(
        stars_count: [u64; 4],
        pools: Vec<String>,
        all_pools: Vec<String>,
        pools_count: Vec<u64>,
        water_place: Vec<u64>,
        months_count: Vec<u64>,
        all_months: Vec<String>,
        is_total: bool,
    ) -> Statistics {
        let total = stars_count.iter().sum();
        let mut stars_percentage = [0.0; 4];
        for i in 0..4 {
            stars_percentage[i] = stars_count[i] as f64 / total as f64 * 100.0;
        }
        if is_total {
            Statistics::Total(TotalStatistics {
                total,
                stars_count,
                stars_percentage,
                all_pools,
                pools_count,
                water_place,
                months_count,
                all_months,
            })
        } else {
            Statistics::Pool(PoolStatistics {
                pool_name: pools[0].clone(),
                total,
                all_pools,
                stars_count,
                stars_percentage,
                water_place: water_place[0],
                months_count,
                all_months,
            })
        }
    }
}

#[tauri::command]
pub fn calculate_statistics(
    gacha: Vec<SingleGacha>,
    pool: Option<String>, // None means all pools
) -> HandlerResult<Statistics> {
    println!("calculate_statistics: {:?}", pool);

    let mut stars_count = [0; 4];
    let mut pools = Vec::new();
    let mut all_pools = HashSet::new();
    let mut pools_count = HashMap::new();
    let mut water_place = HashMap::new();
    let mut months_count = HashMap::new();

    let mut gacha = gacha.to_vec();
    gacha.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    for gacha in gacha {
        all_pools.insert(gacha.pool.clone());

        if pool.is_some() && gacha.pool != pool.clone().unwrap() {
            continue;
        }

        stars_count[gacha.character.rarity as usize - 2] += 1;
        pools_count
            .entry(gacha.pool.clone())
            .and_modify(|x| *x += 1)
            .or_insert(1);
        if gacha.character.rarity != 5 {
            water_place
                .entry(gacha.pool.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        } else {
            water_place
                .entry(gacha.pool.clone())
                .and_modify(|x| *x = 0)
                .or_insert(0);
        }
        
        // get month of gacha
        let date = DateTime::from_timestamp(gacha.timestamp, 0).ok_or(HandlerExecutionError::TimeStampParseError(gacha.timestamp))?;
        let month = format!("{}-{}", date.year(), date.month());
        months_count
            .entry(month)
            .and_modify(|x| *x += 1)
            .or_insert(1);

        if !pools.contains(&gacha.pool) {
            pools.push(gacha.pool.clone());
        }
    }

    let all_pools = all_pools.into_iter().collect();
    let pools_count = pools.iter().map(|x| pools_count[x]).collect();
    let water_place = pools.iter().map(|x| water_place[x]).collect();

    let all_months = months_count.keys().cloned().collect();
    let months_count = months_count.values().cloned().collect();

    Ok(Statistics::new(
        stars_count,
        pools,
        all_pools,
        pools_count,
        water_place,
        months_count,
        all_months,
        pool.is_none(),
    ))
}
