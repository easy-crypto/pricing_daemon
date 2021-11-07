use async_trait::async_trait;
use std::error::Error;

use super::types::Price;

#[async_trait(?Send)]
pub trait PriceDB {
    async fn insert_many(&self, price: Vec<Price>) -> Result<(), Box<dyn Error>>;
}
