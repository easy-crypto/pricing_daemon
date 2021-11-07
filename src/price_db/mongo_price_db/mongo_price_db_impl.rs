use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client};
use std::error::Error;

use crate::price_db::{price_db::PriceDB, types::Price};

use super::{
    constants::DB_NAME,
    mongo_price_db::{MongoPriceDB, MongoPriceDBConfig},
};

impl MongoPriceDBConfig {
    pub fn new() -> Self {
        MongoPriceDBConfig {
            username: None,
            password: None,
            port: None,
            host: None,
        }
    }
}

impl MongoPriceDB {
    pub async fn new(config: MongoPriceDBConfig) -> Result<Self, Box<dyn Error>> {
        let host = match config.host {
            Some(h) => h,
            None => "localhost".to_owned(),
        };
        let port = match config.port {
            Some(p) => p,
            None => 27017,
        };
        let client_options = ClientOptions::parse(format!("mongodb://{}:{}", host, port)).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(DB_NAME);
        Ok(MongoPriceDB { db })
    }
}

#[async_trait(?Send)]
impl PriceDB for MongoPriceDB {
    async fn insert_many(&self, prices: Vec<Price>) -> Result<(), Box<dyn Error>> {
        let price_collection = self.db.collection::<Price>("prices");
        price_collection.insert_many(prices, None).await?;
        Ok(())
    }
}
