use crate::price_db;
use kraken_rest_api::api::api::KrakenAPI;

pub struct DBPriceManager {
    pub db: Box<dyn price_db::price_db::PriceDB>,
    pub kraken_api: KrakenAPI,
}
