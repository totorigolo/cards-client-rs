use log::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::components;
use crate::pages;
use crate::routes::*;
use crate::state::State;

const KEY: &str = "cards-client-rs.state";

pub struct App {
    #[allow(unused)]
    link: ComponentLink<Self>,

    storage: StorageService,
    state: Rc<RefCell<State>>,
    _router_agent: Box<dyn Bridge<RouteAgent<()>>>,
}

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
            state: Rc::new(RefCell::new(state)),
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
        let state_to_write = &*self.state.borrow();
        self.storage.store(KEY, Json(state_to_write));
        true
    }

    fn view(&self) -> Html {
        let state_inner = self.state.clone();
        let state = move || state_inner.clone();

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
                                render = Router::render(move |route: AppRoute| {
                                    let page = match &route {
                                        AppRoute::Index => html!{ <pages::Index /> },
                                        AppRoute::WsExperiment => html!{ <pages::WsExperiment state=state() /> },
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
