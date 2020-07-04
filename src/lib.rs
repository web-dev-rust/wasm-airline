#![recursion_limit="1024"]
mod app;
mod gql;
mod best_prices;
mod reccomendation;
mod index;

use wasm_bindgen::prelude::*;
use yew::prelude::App;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<index::Model>::new().mount_as_body();
}