use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response}
};
use yew::format::{Text, Json};
use serde_json::from_str;
use crate::gql::{GqlResponse, fetch_gql};


pub struct Airline {
    fetch: FetchService,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    graphql_url: String,
    graphql_response: Option<GqlResponse>
}


impl Airline {
    pub fn fetch_data(&mut self) {
        let request = fetch_gql();
  
          let callback = self.link.callback(
              move |response: Response<Text>| {
                  let (meta, data) = response.into_parts();
                  if meta.status.is_success() {
                      Msg::FetchGql(Some(data))
                  } else {
                      Msg::FetchGql(None)
                  }
              },
          );
  
          let request = Request::builder()
              .method("POST")
              .header("content-type", "application/json")
              .uri(self.graphql_url.clone())
              .body(Json(&request))
              .unwrap();

          let task = self.fetch.fetch(request, callback).unwrap();
          self.fetch_task = Some(task);
          Msg::Fetching(false);
      }
}

pub enum Msg {
    FetchGql(Option<Text>),
    Fetching(bool)
}

impl Component for Airline {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Airline {
            fetch: FetchService::new(),
            link: link,
            fetch_task: None,
            fetching: true,
            graphql_url: "http://localhost:4000/graphql".to_string(),
            graphql_response: None
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            Msg::Fetching(true);
          self.fetch_data(); 
        }
     }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool { 
        false
     }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchGql(data) => {
                self.graphql_response = match data {
                    Some(Ok(val)) => {
                        self.fetching = false;
                        let resp: GqlResponse = from_str(&val).unwrap();
                        Some(resp)
                    },
                    _ => {
                        self.fetching = false;
                        None
                    }
                }
            },
            Msg::Fetching(fetch) => {
                self.fetching = fetch;
            }
        }
        true
    }

    fn view(&self) -> Html {
        if self.fetching {
            html! {
                <div class="loading-margin">
                    <div class="loader"></div>
                </div>
            } 
        } else {
            html! {
                <div class="body">
                    { 
                        if let Some(data) = &self.graphql_response {
                            html!{<div>
                                <div> {data.clone().best_prices().view()} </div>
                                <div> {
                                    data.clone().recommendations().view()
                                } </div>
                             </div> }
                        } else {
                            html!{
                                <p class="failed-fetch">
                                    {"Failed to fetch"}
                                </p>
                            }
                        }
                    }
                </div>
            }
        }
    }
}
