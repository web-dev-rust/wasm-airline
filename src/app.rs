use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response}
};
use yew::format::{Text, Json};
use yew::virtual_dom::VNode;
use yew_router::{router::Router, Switch};
use serde_json::from_str;
use crate::gql::{GqlResponse, fetch_gql};

#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/oneway?departure={departure}&origin={origin}&destination={destination}"]
    Oneway {departure: String, origin: String, destination: String},
    #[to = "/hello"]
    Hello,
    #[to = "/"]
    NotFound
}

pub struct Model {}

pub struct Airline {
    fetch: FetchService,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    graphql_url: String,
    graphql_response: Option<GqlResponse>,
    filter_cabin: String,
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
    Fetching(bool),
    Cabin(String),
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
            graphql_response: None,
            filter_cabin: String::from("Y"),
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
            },
            Msg::Cabin(c) => {
                self.filter_cabin = c
            },
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
                                <div> { data.clone().recommendations().view(&self.link, &self.filter_cabin) } </div>
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


impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool { 
        false
     }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode { 
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::NotFound => VNode::from("404"),
                        AppRoute::Hello => html!{<p> {"Hello"} </p>},
                        AppRoute::Oneway {departure, origin, destination} => html!{
                            <p>{
                                format!("{}/{}/{}", departure, origin, destination)
                            }</p>
                        }
                    }
                })
            />
        }
    }
}