use crate::hypixel::client::HypixelResponse;
use serde::Deserialize;
use std::collections::HashMap;

type Price = f64;
type Amount = u64;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BazaarOrder {
    pub amount: Amount,
    pub price_per_unit: Price,
    pub orders: Amount,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BazaarQuickStatus {
    pub sell_price: Price,
    pub sell_volume: Amount,
    pub sell_moving_week: Amount,
    pub sell_orders: Amount,
    pub buy_price: Price,
    pub buy_volume: Amount,
    pub buy_moving_week: Amount,
    pub buy_orders: Amount,
}

#[derive(Debug, Deserialize)]
pub struct BazaarProduct {
    pub product_id: String,
    pub sell_summary: Vec<BazaarOrder>,
    pub buy_summary: Vec<BazaarOrder>,
    pub quick_status: BazaarQuickStatus,
}

pub type BazaarData = HypixelResponse<HashMap<String, BazaarProduct>>;
