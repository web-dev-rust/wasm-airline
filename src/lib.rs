mod app;
mod gql;
mod best_prices;

use wasm_bindgen::prelude::*;
use yew::prelude::App;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::Airline>::new().mount_as_body();
}