use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use crate::best_prices::BestPrices;
use crate::reccomendation::Recommendations;

pub fn fetch_gql() -> Value {
    json!({
        "query": "{
            recommendations(departure: \"2020-06-28\", 
                origin: \"POA\", 
                destination: \"GRU\") {
                data{
                recommendedFlightCode
                flights {
                    flightCode
                    flightDuration
                    stops
                    arrival {
                        cityName
                        airportName
                        airportCode
                        dateTime
                    }
                    departure {
                        cityName
                        airportName
                        airportCode
                        dateTime
                    }
                    segments {
                    flightNumber
                    equipment {
                        name
                        code
                    }
                    }
                    cabins {
                        code
                        displayPrice
                        availabilityCount
                    }
                }
                }
            }
            bestPrices(departure: \"2020-06-28\", origin: \"POA\", destination: \"GRU\") {
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
    best_prices: BestPrices,
    recommendations: Recommendations,
}

impl GqlResponse {
    pub fn best_prices(self) -> BestPrices {
        self.data.best_prices
    }

    pub fn recommendations(self) -> Recommendations {
        self.data.recommendations
    }
}