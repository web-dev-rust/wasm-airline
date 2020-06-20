use yew::prelude::*;

pub struct Airline {}

pub enum Msg {}

impl Component for Airline {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Airline {}
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool { todo!() }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello Julia" }</p>
            </div>
        }
    }
}