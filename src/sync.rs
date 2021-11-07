use std::error::Error;

use crate::price_manager::db_price_manager::db_price_manager::DBPriceManager;
use crate::price_manager::price_manager::PriceManager;

use crate::price_db::mongo_price_db::mongo_price_db::{MongoPriceDB, MongoPriceDBConfig};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let db = MongoPriceDB::new(MongoPriceDBConfig::new()).await?;

    let manager = DBPriceManager::new(Box::new(db));
    sync_price_data(Box::new(manager)).await?;
    Ok(())
}

pub async fn sync_price_data(manager: Box<dyn PriceManager>) -> Result<(), Box<dyn Error>> {
    manager.update_price_data().await?;
    println!("sync price data > OK");
    Ok(())
}
