use yew::prelude::*;

use crate::components::NeqAssign;

pub struct NotFound {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub route: String,
}

impl Component for NotFound {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NotFound { props }
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
                <h2 class="title is-size-4">{ "Route not found" }</h2>
                <p>{ "URL: " }{ &self.props.route }</p>
            </>
        }
    }
}
