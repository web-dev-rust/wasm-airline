use yew_router::{route::Route, service::RouteService, Switch};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use crate::app::Airline;


#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/oneway?departure={departure}&origin={origin}&destination={destination}"]
    Oneway {departure: String, origin: String, destination: String},
    #[to = "/"]
    Index
}

pub struct Model {
    route_service: RouteService<()>,
    route: Route<()>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
}

impl Model {
    fn change_route(&self, app_route: AppRoute) -> Callback<MouseEvent> {
        self.link.callback(move |_| {
            let route = app_route.clone();
            Msg::ChangeRoute(route)
        })
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);

        Model {
            route_service,
            route,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
            Msg::ChangeRoute(route) => {
                self.route = route.into();
                self.route_service.set_route(&self.route.route, ());
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                {
                    match AppRoute::switch(self.route.clone()) {
                        Some(AppRoute::Index) => self.view_index(),
                        Some(AppRoute::Oneway{departure, origin, destination}) 
                            => html!{<Airline departure = departure origin = origin destination = destination />},
                        None => VNode::from("404")
                    }
                }
            </div>
        }
    }
}

impl Model {
    fn view_index(&self) -> Html {
        html!{
            <button onclick=&self.change_route(AppRoute::Oneway
                {departure: "2020-08-21".to_string(), 
                 origin: "POA".to_string(), 
                 destination: "GRU".to_string()}) > 
                {"Submit"}
            </button>
        }
    }
}