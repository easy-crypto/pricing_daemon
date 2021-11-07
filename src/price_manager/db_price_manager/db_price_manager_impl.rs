use crate::{
    price_db::{price_db::PriceDB, types::Price},
    price_manager::price_manager::PriceManager,
};
use async_trait::async_trait;
use chrono::Utc;

use super::db_price_manager::DBPriceManager;
use std::error::Error;

impl DBPriceManager {
    pub fn new(db: Box<dyn PriceDB>) -> Self {
        DBPriceManager { db }
    }
}

#[async_trait(?Send)]
impl PriceManager for DBPriceManager {
    async fn update_price_data(&self) -> Result<(), Box<dyn Error>> {
        //self.db
        //    .insert_many(vec![Price {
        //        open: 100f32,
        //        close: 100f32,
        //        high: 100f32,
        //        low: 100f32,
        //        date_time: Utc::now(),
        //    }])
        //    .await?;
        Ok(())
    }
}
