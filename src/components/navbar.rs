use yew::prelude::*;

use crate::routes::*;

pub struct Navbar {
    is_active: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    BurgerClicked,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            is_active: false,
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::BurgerClicked => {
                self.is_active = !self.is_active;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let is_active = if self.is_active { "is-active" } else { "" };
        html! {
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <NavLink route=AppRoute::Index classes="navbar-item">
                        <p>{ crate::constants::SITE_NAME }</p>
                    </NavLink>

                    <a role="button"
                        aria-label="menu" aria-expanded="false"
                        class=("navbar-burger burger", is_active)
                        onclick=self.link.callback(|_: MouseEvent| Msg::BurgerClicked)
                        >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>

                <div class=("navbar-menu", is_active)>
                    <NavLink classes="navbar-item" route=AppRoute::CreateGame>
                        { "Start a game" }
                    </NavLink>
                    <NavLink classes="navbar-item" route=AppRoute::ListGames>
                        { "List games" }
                    </NavLink>
                    <NavLink classes="navbar-item" route=AppRoute::WsExperiment>
                        { "WebSocket test" }
                    </NavLink>

                    <div class="navbar-start">
                        <div class="navbar-item has-dropdown is-hoverable">
                            <a class="navbar-link">
                                { "Help" }
                            </a>

                            <div class="navbar-dropdown">
                                <a class="navbar-item">
                                    { "User guide" }
                                </a>
                                <a class="navbar-item">
                                    { "About" }
                                </a>
                                <hr class="navbar-divider" />
                                <a class="navbar-item">
                                    { "Report an issue" }
                                </a>
                            </div>
                        </div>
                    </div>

                    <div class="navbar-end">
                        <div class="navbar-item">
                            <div class="buttons">
                                <a class="button is-primary">
                                    <strong>{ "Share" }</strong>
                                </a>
                                <a class="button is-light" >
                                    { "Settings" }
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}
