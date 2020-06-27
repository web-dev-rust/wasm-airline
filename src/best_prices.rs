use yew::prelude::*;
use yew::virtual_dom::VNode;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestPrices {
    best_prices: Vec<BestPrice>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BestPrice {
    date: String,
    available: bool,
    price: Option<Price>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Price {
    amount: f64
}

impl BestPrices {
    pub fn view(&self) -> VNode {
        let carrousel = self.best_prices
            .clone()
            .into_iter()
            .map(|bp| html!{
                <div class="cell">
                    {
                        if bp.available {
                            html!{
                                <div class="full-cell">
                                    {
                                        {
                                            let date = Utc.datetime_from_str(&format!("{} 00:00:00", bp.date), "%Y-%m-%d %H:%M:%S");
                                            date.unwrap().format("%a %e %b").to_string()
                                        }
                                    } <br/>
                                    {format!("R$ {}", bp.price.unwrap().amount).replace(".", ",")}
                                </div>
                            }
                        } else {
                            html!{
                                <div class="empty-cell">
                                    {
                                        {
                                            let date = Utc.datetime_from_str(&format!("{} 00:00:00", bp.date), "%Y-%m-%d %H:%M:%S");
                                            date.unwrap().format("%a %e %b").to_string()
                                        }
                                     } <br/>
                                    {"N/A"}
                                </div>
                            }
                        }
                    }
                </div>
            }).collect::<Html>();

        html!{
            <div class="carrousel"> 
                {carrousel}
            </div>
        }
    }
}