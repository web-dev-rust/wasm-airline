use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
};
use yew::format::{Text, Json};
use crate::gql::fetch_gql;


pub struct Airline {
    fetch: FetchService,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    fetching: bool,
    graphql_url: String,
    graphql_response: Option<String>
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
          self.fetching = false;
      }
}

pub enum Msg {
    FetchGql(Option<Text>),
}

impl Component for Airline {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Airline {
            fetch: FetchService::new(),
            link: link,
            fetch_task: None,
            fetching: false,
            graphql_url: "http://172.21.1.2:4000/graphql".to_string(),
            graphql_response: None
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.fetching = true;
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
                        Some(val)
                    },
                    _ => {
                        self.fetching = false;
                        None
                    }
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        if self.fetching {
            html! {
                <div class="loading">
                    {"Loading..."}
                </div>
            } 
        } else {
            html! {
                <div>
                    <p>{ 
                        if let Some(data) = &self.graphql_response {
                            data
                        } else {
                            "Failed to fetch"
                        }
                    }</p>
                </div>
            }
        }
    }
}
