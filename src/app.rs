use log::*;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::prelude::*;

use crate::components;
use crate::pages;
use crate::routes::*;

const KEY: &str = "cards-client-rs.state";

pub struct App {
    #[allow(unused)]
    link: ComponentLink<Self>,

    storage: StorageService,
    state: State,
    _router_agent: Box<dyn Bridge<RouteAgent<()>>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {}

#[derive(Debug)]
pub enum Msg {
    // ChangeRoute(AppRoute),
    RouteChanged(Route<()>),
    #[allow(unused)]
    Ignore,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();

        let state = match storage.restore(KEY) {
            Json(Ok(restored_state)) => restored_state,
            _ => State::default(),
        };

        let router_agent = RouteAgent::bridge(link.callback(Msg::RouteChanged));

        App {
            link,
            storage,
            state,
            _router_agent: router_agent,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => {
                debug!("Route changed: {}", route);
            }
            Msg::Ignore => return false,
        }
        self.storage.store(KEY, Json(&self.state));
        true
    }

    fn view(&self) -> Html {
        trace!("Rendered.");

        html! {
            <>
                <components::Notifications />

                <div class="section">
                    <div class="container navbar-container">
                        <div class="box">
                            <components::Navbar />
                        </div>
                    </div>
                </div>

                <div class="section site-content">
                    <div class="container">
                        <div class="box">
                            <Router<AppRoute>
                                render = Router::render(|route: AppRoute| {
                                    let page = match &route {
                                        AppRoute::Index => html!{ <pages::Index /> },
                                        AppRoute::WsExperiment => html!{ <pages::WsExperiment /> },
                                        AppRoute::Game(GameRoute::List) => html!{ <pages::ListGames /> },
                                        AppRoute::Game(GameRoute::Create) => html!{ <pages::CreateGame /> },
                                        AppRoute::Game(p @ GameRoute::Play { .. }) => html!{ "todo" },
                                    };
                                    html! {
                                        <>
                                            { route.render_breadcrumb() }
                                            { page }
                                        </>
                                    }
                                })
                                redirect = Router::redirect(|_: Route| AppRoute::Index)
                                />
                        </div>
                    </div>
                </div>

                <footer class="footer">
                    <div class="content has-text-centered">
                        <p>{ "Wonderful footer" }</p>
                    </div>
                </footer>
            </>
        }
    }
}

impl App {}

impl State {}
