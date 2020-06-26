use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use crate::best_prices::BestPrices;

pub fn fetch_gql() -> Value {
    json!({
        "query": "{
             bestPrices(departure: \"2020-07-21\", origin: \"POA\", destination: \"GRU\") {
                bestPrices {
                    date
                    available
                    price {amount}
                }
             }
        }"
    })
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GqlResponse {
    data: GqlFields
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GqlFields {
    best_prices: BestPrices
}

impl GqlResponse {
    pub fn best_prices(self) -> BestPrices {
        self.data.best_prices
    }
}