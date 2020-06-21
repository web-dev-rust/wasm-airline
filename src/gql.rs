use serde_json::{json, Value};

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
