use serde::{Deserialize, Serialize};
use yew::prelude::*;
use chrono::prelude::*;
use crate::app::{Airline, Msg};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Recommendations {
    data: Vec<RecommendedFlight>,
}

#[derive(Properties, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedFlight {
    recommended_flight_code: String,
    flights: Vec<Flight>
}

#[derive(Properties, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Flight {
    flight_code: String,
    flight_duration: String,
    stops: i32,
    arrival: OriginDestination,
    departure: OriginDestination,
    segments: Vec<Segment>,
    cabins: Vec<Cabin>
}

#[derive(Properties, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OriginDestination {
    city_name: String,
    airport_name: String,
    airport_code: String,
    date_time: String,
}

#[derive(Properties, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Segment { 
    flight_number: String,
    equipment: Equipment
}

#[derive(Properties, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Equipment { 
    name: String,
    code: String
}

#[derive(Properties, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cabin { 
    code: String,
    display_price: f64,
    availability_count: i32
}

impl Recommendations {
    pub fn view(&self, link: &ComponentLink<Airline>, filter_cabin: &str) -> Html {
        html!{
            <div>
                <div class="cabins">
                    <div class="cabin" onclick=link.callback(move |_| Msg::Cabin("Y".to_string()))>
                        {"Economy"}</div>
                    <div class="cabin" onclick=link.callback(move |_| Msg::Cabin("W".to_string()))>
                        {"Premium Economy"}</div>
                </div>
                <div class="flight-container"> {
                    self.data[0].clone().flights.into_iter()
                    .map(|r|
                        html!{
                            <div class="flight"> 
                                <div class="flight-cell origins-destinations">
                                    <div class="origin-destination"> 
                                        <div class="time"> {{
                                            let date = Utc.datetime_from_str(&r.departure.date_time[..16],"%Y-%m-%dT%H:%M");
                                            date.unwrap().format("%H:%M").to_string()
                                        }} </div>
                                        <div class="airport"> {r.departure.airport_code} </div>
                                    </div>
                                    <div class="arrow">{">"}</div>
                                    <div class="origin-destination">
                                        <div class="time"> {{
                                            let date = Utc.datetime_from_str(&r.arrival.date_time[..16],"%Y-%m-%dT%H:%M");
                                            date.unwrap().format("%H:%M").to_string()
                                        }} </div>
                                        <div class="airport"> {r.arrival.airport_code} </div>
                                    </div>
                                </div>
                                <div class="flight-cell duration"> {
                                    r.flight_duration.replace("PT","").replace("H", "h ").replace("M", "min")
                                } </div>
                                <div class="flight-cell stops"> {
                                    if r.stops == 0 {
                                        html!{<p>{"Direto"}</p>}
                                    } else {
                                        html!{<p>{r.stops.to_string()}</p>}
                                    }   
                                } </div>
                                <div class="flight-cell price"> {{
                                    let cabin = r.cabins.into_iter()
                                        .filter(|c| c.availability_count > 0 && &c.code == filter_cabin)
                                        .collect::<Vec<Cabin>>();
                                    match cabin.first() {
                                        Some(c) => format!("R$ {:?}", c.display_price),
                                        None => String::from("N/A")
                                    }
                                }}</div>
                            </div>
                        }
                    )
                    .collect::<Html>()
                }
                </div>
            </div>
        }
    }
}
