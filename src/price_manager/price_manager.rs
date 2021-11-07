use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait PriceManager {
    async fn update_price_data(&self) -> Result<(), Box<dyn Error>>;
}
