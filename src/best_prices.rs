use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestPrices {
    best_prices: Vec<BestPrice>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BestPrice {
    date: String,
    available: bool,
    price: Price
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Price {
    amount: f64
}
