use derive_more::Display;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, Display)]
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
}

#[allow(unused)]
pub type NavBtn = RouterButton<AppRoute>;

#[allow(unused)]
pub type NavLink = RouterAnchor<AppRoute>;

pub trait Breadcrumb {
    fn breadcrumb_components(&self) -> Vec<(&'static str, Option<AppRoute>)>;

    fn render_breadcrumb(&self) -> Html {
        let render_component = |name, route: Option<AppRoute>, class| {
            match route {
                Some(route) => html! { <li class=class><NavLink route=route>{ name }</NavLink></li> },
                None => html! { <li class=class>{ name }</li> },
            }
        };

        let mut components = self.breadcrumb_components();
        if let (Some((last_name, last_route)), rest) = (components.pop(), components) {
            html! {
               <nav class="breadcrumb" aria-label="breadcrumbs">
                   <ul>
                       { render_component(crate::constants::SITE_NAME, Some(AppRoute::Index), "") }
                       { for rest.into_iter().map(|(n, r)| render_component(n, r, "")) }
                       { render_component(last_name, last_route, "is-active") }
                   </ul>
               </nav>
            }
        } else {
            html! {}
        }
    }
}

impl Breadcrumb for AppRoute {
    fn breadcrumb_components(&self) -> Vec<(&'static str, Option<AppRoute>)> {
        match self {
            AppRoute::Index => vec![("Index", None)],
            AppRoute::WsExperiment => vec![("WebSocket experiment", None)],
            AppRoute::ListGames => vec![("Games", Some(AppRoute::ListGames)), ("List games", None)],
            AppRoute::CreateGame => vec![("Games", Some(AppRoute::ListGames)), ("Create game", None)],
            AppRoute::JoinGame { .. } => vec![("Games", Some(AppRoute::ListGames)), ("Joining game", None)],
            AppRoute::PlayGame { .. } => vec![("Games", Some(AppRoute::ListGames)), ("Play game", None)],
        }
    }
}
