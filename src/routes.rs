use derive_more::Display;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, Display, PartialEq)]
pub enum AppRoute {
    #[to = "/!"]
    #[display(fmt = "/")]
    Index,

    #[to = "/ws_experiment"]
    #[display(fmt = "/ws_experiment")]
    WsExperiment,

    #[to = "/game/list"]
    #[display(fmt = "/game/list")]
    ListGames,

    #[to = "/game/create"]
    #[display(fmt = "/game/create")]
    CreateGame,

    #[to = "/game/join/{game_id}?as={username}"]
    #[display(fmt = "/game/{}?as={}", game_id, username)]
    JoinGame { game_id: String, username: String },

    #[to = "/game/play/{game_id}?as={player_id}"]
    #[display(fmt = "/game/play/{}?as={}", game_id, player_id)]
    PlayGame { game_id: String, player_id: String },

    #[to = "/not_found{*}"]
    #[display(fmt = "/not_found{}", _0)]
    NotFound(String),
}

#[allow(unused)]
pub type NavBtn = RouterButton<AppRoute>;

#[allow(unused)]
pub type NavLink = RouterAnchor<AppRoute>;

pub trait Breadcrumb {
    fn breadcrumb_components(&self) -> Vec<(&'static str, AppRoute)>;

    fn render_breadcrumb(&self) -> Html {
        let render_component = |name, route, class| match route {
            route => html! { <li class=class><NavLink route=route>{ name }</NavLink></li> },
        };

        let mut components = self.breadcrumb_components();
        if let (Some((last_name, last_route)), rest) = (components.pop(), components) {
            html! {
               <nav class="breadcrumb" aria-label="breadcrumbs">
                   <ul>
                       { render_component(crate::constants::SITE_NAME, AppRoute::Index, None) }
                       { for rest.into_iter().map(|(n, r)| render_component(n, r, None)) }
                       { render_component(last_name, last_route, Some("is-active")) }
                   </ul>
               </nav>
            }
        } else {
            html! {}
        }
    }
}

impl Breadcrumb for AppRoute {
    fn breadcrumb_components(&self) -> Vec<(&'static str, AppRoute)> {
        match self {
            AppRoute::Index => vec![("Index", self.clone())],
            AppRoute::WsExperiment => vec![("WebSocket experiment", self.clone())],
            AppRoute::ListGames => {
                vec![("Games", AppRoute::ListGames), ("List games", self.clone())]
            }
            AppRoute::CreateGame => vec![
                ("Games", AppRoute::ListGames),
                ("Create game", self.clone()),
            ],
            AppRoute::JoinGame { .. } => vec![
                ("Games", AppRoute::ListGames),
                ("Joining game", self.clone()),
            ],
            AppRoute::PlayGame { .. } => {
                vec![("Games", AppRoute::ListGames), ("Play game", self.clone())]
            }
            AppRoute::NotFound(_) => vec![("Not found", self.clone())],
        }
    }
}
