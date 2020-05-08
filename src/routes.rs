use derive_more::Display;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, Display)]
pub enum AppRoute {
    #[to = "/!"]
    #[display(fmt = "/")]
    Index,

    #[to = "/ws_experiment"]
    #[display(fmt = "/ws_experiment")]
    WsExperiment,

    #[to = "/game{*:rest}"]
    #[display(fmt = "/game{}", _0)]
    Game(GameRoute),
}

#[derive(Switch, Debug, Clone, Display)]
pub enum GameRoute {
    #[to = "/list"]
    #[display(fmt = "/list")]
    List,

    #[to = "/create"]
    #[display(fmt = "/create")]
    Create,

    #[to = "/join/{game_id}/{player_id}"]
    #[display(fmt = "/join/{}/{}", game_id, player_id)]
    Play { game_id: String, player_id: String },
}

#[allow(unused)]
pub type NavBtn = RouterButton<AppRoute>;

#[allow(unused)]
pub type NavLink = RouterAnchor<AppRoute>;

impl From<GameRoute> for AppRoute {
    fn from(game: GameRoute) -> AppRoute {
        AppRoute::Game(game)
    }
}
