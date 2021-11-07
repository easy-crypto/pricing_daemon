use async_trait::async_trait;
use bson::doc;
use chrono::Duration;
use chrono::Utc;
use mongodb::options::FindOptions;
use mongodb::{options::ClientOptions, Client};
use std::error::Error;

use crate::price_db::{price_db::PriceDB, types::Price};
use futures::stream::TryStreamExt;
use log::info;

use super::mongo_price_db::{MongoPriceDB, MongoPriceDBConfig};

static PRICE_COLLECTION_NAME: &str = "prices";

impl MongoPriceDBConfig {
    pub fn new(username: &str, password: &str, host: &str, db_name: &str) -> Self {
        MongoPriceDBConfig {
            username: Some(username.to_owned()),
            password: Some(password.to_owned()),
            host: Some(host.to_owned()),
            db_name: Some(db_name.to_owned()),
        }
    }
}

impl MongoPriceDB {
    pub async fn new(config: MongoPriceDBConfig) -> Result<Self, Box<dyn Error>> {
        let host = config.host.expect("MongoDB host not set");
        let username = config.username.expect("MongoDB username not set");
        let password = config.password.expect("MongoDB password not set");
        let db_name = config.db_name.expect("MongoDB database name not set");

        let client_options = ClientOptions::parse(format!(
            "mongodb+srv://{}:{}@{}/{}?retryWrites=true&w=majority",
            username, password, host, db_name
        ))
        .await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&db_name);
        Ok(MongoPriceDB { db })
    }

    async fn clean_up(&self) -> Result<(), Box<dyn Error>> {
        let price_collection = self.db.collection::<Price>(PRICE_COLLECTION_NAME);
        let dt = Utc::now() - Duration::days(30);
        let delete_query = doc! { "_id": { "$lt": dt.to_string() }};
        let res = price_collection
            .delete_many(delete_query.clone(), None)
            .await?;
        info!(
            "Clean up old data older than {:?}. count: {}",
            delete_query, res.deleted_count
        );
        Ok(())
    }
}

#[async_trait(?Send)]
impl PriceDB for MongoPriceDB {
    async fn insert_many(&self, prices: Vec<Price>) -> Result<(), Box<dyn Error>> {
        let price_collection = self.db.collection::<Price>(PRICE_COLLECTION_NAME);
        // remove any data that is one month old.
        self.clean_up().await?;

        let res = price_collection.insert_many(prices, None).await?;
        info!("Inserted Ids: {:?}", res.inserted_ids);
        Ok(())
    }

    async fn get_next_price_time(&self) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn Error>> {
        let price_collection = self.db.collection::<Price>("prices");
        let filter = doc! {};
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(1)
            .build();
        let cursor = price_collection.find(filter, find_options).await?;
        let v: Vec<Price> = cursor.try_collect().await?;
        match v.first() {
            Some(price) => Ok(price.id + Duration::minutes(1)),
            None => Ok(Utc::now() - Duration::days(30)),
        }
    }
}
