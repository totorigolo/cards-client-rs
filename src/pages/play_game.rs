use yew::prelude::*;

pub struct PlayGame;

pub enum Msg {}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub game_id: String,
    pub player_id: String,
}

impl Component for PlayGame {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { "Play game" }
            </>
        }
    }
}
