use crate::ws::utils::string_or_float;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitstampOrderbookEvent {
    pub data: Data,
    pub channel: String,
    pub event: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BinanceOrderBookEvent {
    pub last_update_id: u64,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

// #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
// pub struct Level {
//     #[serde(with = "string_or_float")]
//     pub price: f64,
//     #[serde(with = "string_or_float")]
//     pub amount: f64,
// }

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceLevel {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
}

impl From<(f64, f64)> for PriceLevel {
    fn from(t: (f64, f64)) -> Self {
        Self {
            price: t.0,
            amount: t.1,
        }
    }
}

impl From<[&str; 2]> for PriceLevel {
    fn from(src: [&str; 2]) -> Self {
        let price: f64 = fast_float::parse(src[0]).expect("invalid price");
        let amount: f64 = fast_float::parse(src[1]).expect("invalid amount");

        Self { price, amount }
    }
}
