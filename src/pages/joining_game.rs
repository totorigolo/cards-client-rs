use yew::prelude::*;

use crate::components::NeqAssign;

pub struct JoiningGame {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub game_id: String,
    pub username: String,
}

impl Component for JoiningGame {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        JoiningGame {
            props
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h2 class="title is-size-4">{ "Joining game" }</h2>
                <p>{ "Game ID: " }{ &self.props.game_id }</p>
                <p>{ "Player name: " }{ &self.props.username }</p>
            </>
        }
    }
}
