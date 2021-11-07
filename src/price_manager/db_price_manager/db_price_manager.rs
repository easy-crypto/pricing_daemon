use crate::price_db;

pub struct DBPriceManager {
    pub db: Box<dyn price_db::price_db::PriceDB>,
}
