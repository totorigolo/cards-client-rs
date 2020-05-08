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

// pub trait Routable {
//     fn get_route_name(&self) -> String;
//
//     fn get_route(&self) -> AppRoute;
// }
//
// impl Routable for AppRoute {
//     fn get_route_name(&self) -> String {
//         match self {
//             AppRoute::Index => "Index",
//             AppRoute::WsExperiment => "WebSocket experiment",
//             AppRoute::Game(_) => "Game",
//         }.into()
//     }
//
//     fn get_route(&self) -> AppRoute {
//         self.clone()
//     }
// }
//
// impl Routable for GameRoute {
//     fn get_route_name(&self) -> String {
//         match self {
//             GameRoute::List => "List games",
//             GameRoute::Create => "Create game",
//             GameRoute::Play { .. } => "Play game",
//         }.into()
//     }
//
//     fn get_route(&self) -> AppRoute {
//         self.clone().into()
//     }
// }
//
// pub trait ToNavBtn: Routable {
//     fn to_nav_btn<C: ToString>(&self, classes: C) -> Html {
//         html! {
//             <NavBtn classes=classes.to_string() route=self.get_route()>
//                 { self.get_route_name() }
//             </NavBtn>
//         }
//     }
// }
//
// pub trait ToNavLink: Routable {
//     fn to_nav_link<C: ToString>(&self, classes: C) -> Html {
//         html! {
//             <NavLink classes=classes.to_string() route=self.get_route()>
//                 { self.get_route_name() }
//             </NavLink>
//         }
//     }
// }
//
// impl<T: Routable> ToNavBtn for T {}
// impl<T: Routable> ToNavLink for T {}

pub trait BreadcrumbComponent {
    fn render_in_breadcrumb(&self) -> bool{
        true
    }

    fn breadcrumb_name(&self) -> &'static str;
    fn breadcrumb_route(&self) -> AppRoute;

    fn breadcrumb_child(&self) -> Option<&dyn BreadcrumbComponent>{
        None
    }
}

impl BreadcrumbComponent for AppRoute {
    fn breadcrumb_name(&self) -> &'static str {
        match self {
            AppRoute::Index => "Index",
            AppRoute::WsExperiment => "WebSocket experiment",
            AppRoute::Game(_) => "Game",
        }
    }

    fn breadcrumb_route(&self) -> AppRoute {
        self.clone()
    }

    fn breadcrumb_child(&self) -> Option<&dyn BreadcrumbComponent> {
        match self {
            AppRoute::Game(game_route) => Some(game_route),
            _ => None,
        }
    }
}

impl BreadcrumbComponent for GameRoute {
    fn breadcrumb_name(&self) -> &'static str {
        match self {
            GameRoute::List => "List games",
            GameRoute::Create => "Create game",
            GameRoute::Play { .. } => "Play game",
        }
    }

    fn breadcrumb_route(&self) -> AppRoute {
        self.clone().into()
    }
}

pub trait Breadcrumb {
    fn breadcrumb_components(&self) -> Vec<(&'static str, AppRoute)>;

    fn render_breadcrumb(&self) -> Html {
        let components = self.breadcrumb_components();
        if let Some(((last_name, last_route), rest)) = components.split_last() {
            html! {
                <nav class="breadcrumb" aria-label="breadcrumbs">
                    <ul>
                        <li><a href="#">{ "The Game" }</a></li>
                        { for rest.iter().map(|(n, r)| html! { <NavLink route=r>{ n }</NavLink> }) }
                        <li class="is-active"><NavLink route=last_route>{ last_name }</NavLink></li>
                    </ul>
                </nav>
             }
        } else { html! {} }
    }
}

impl<T: BreadcrumbComponent> Breadcrumb for T {
    fn breadcrumb_components(&self) -> Vec<(&'static str, AppRoute)> {
        let mut components = vec![];

        let mut child = Some(self as &dyn BreadcrumbComponent);
        while child.is_some() {
            components.push((child.unwrap().breadcrumb_name(), child.unwrap().breadcrumb_route()));
            child = child.unwrap().breadcrumb_child();
        }

        components
    }
}
