use crate::{
    price_db::{price_db::PriceDB, types::Price},
    price_manager::price_manager::PriceManager,
};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use kraken_rest_api::api::api_impl::BTCUSD;
use log::info;
use std::collections::HashMap;

use super::db_price_manager::DBPriceManager;
use kraken_rest_api::api::{api::KrakenAPI, methods::Method};
use std::error::Error;

impl DBPriceManager {
    pub fn new(db: Box<dyn PriceDB>, kraken_api: KrakenAPI) -> Self {
        DBPriceManager { db, kraken_api }
    }

    async fn fetch_prices(&self, epoch_since: u64) -> Result<Vec<Price>, Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("pair".into(), BTCUSD.into());
        params.insert("interval".into(), "1".into()); // every minute
        params.insert("since".into(), epoch_since.to_string()); // every minute

        let res = self
            .kraken_api
            .query_public::<HashMap<String, serde_json::Value>>(Method::OHLC, &params)
            .await?;
        let result = res.result.expect("Should have result");

        let data = result
            .get(BTCUSD)
            .expect("OHLC data should exist for the queried pair");

        let data = data.as_array().expect("should in array of OHLC");

        let prices = data.iter().map(|val| {
            let val = val.as_array()
            .expect("OHLC should in array format: [1609027200,\"26560.5\",\"26560.5\",\"26560.5\",\"26560.5\",\"0.0\",\"0.00000000\",0]");

            let epoch = val
                .get(0)
                .expect("has epoch")
                .as_i64()
                .expect("should be int");
            let open = val
                .get(1)
                .expect("has open")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let high = val
                .get(2)
                .expect("has high")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let low = val
                .get(3)
                .expect("has low")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let close = val
                .get(4)
                .expect("has close")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let vwap = val
                .get(6)
                .expect("has vwap")
                .as_str()
                .expect("should be string")
                .parse::<f64>()
                .expect("should be float");
            let volume = val
                .get(6)
                .expect("has vwap")
                .as_str()
                .expect("should be string")
                .parse::<f64>()
                .expect("should be float");
            let count = val
                .get(7)
                .expect("has count")
                .as_u64()
                .expect("should be int");

            let timestamp = NaiveDateTime::from_timestamp(epoch, 0);

            let price = Price {
                id: DateTime::from_utc(timestamp, Utc),
                open,
                high,
                low,
                close,
                vwap,
                volume,
                count: count as u32, 
            };
            price
        }).filter(|price| {
            price.id.timestamp() >= epoch_since as i64
        });

        Ok(prices.collect())
    }
}

#[async_trait(?Send)]
impl PriceManager for DBPriceManager {
    async fn update_price_data(&self) -> Result<(), Box<dyn Error>> {
        info!(r#"Fetch "last price time" store in the database"#);
        let next_time = self.db.get_next_price_time().await?;
        let next_time_sec = next_time.timestamp();
        info!(r#"Fetched "last price time" is {}({})"#, next_time, next_time_sec);

        info!(r#"Fetch all price data from "last price time" to now."#);
        let prices = self.fetch_prices(next_time_sec as u64).await?;

        info!(r#"Store price data in the database"#);
        info!("length is {}", prices.len());
        if prices.is_empty() {
            info!("Price database up to date.");
            return Ok(());
        }
        self.db
            .insert_many(prices)
            .await?;
        Ok(())
    }
}
