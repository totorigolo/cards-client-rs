use yew::prelude::*;

use crate::routes::*;

pub struct ListGames {
    link: ComponentLink<Self>,

    game_id: String,
    username: String,
}

pub enum Msg {
    GameIdChanged(String),
    UsernameChanged(String),
}

impl Component for ListGames {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ListGames {
            link,

            game_id: String::from(""),
            username: String::from(""),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GameIdChanged(game_id) => self.game_id = game_id,
            Msg::UsernameChanged(username) => self.username = username,
        }
        true
    }

    fn view(&self) -> Html {
        use crate::html::*;

        let join_route: AppRoute = AppRoute::JoinGame {
            game_id: self.game_id.clone(),
            username: self.username.clone(),
        };

        let game_id_changed = self
            .link
            .callback(|e: InputData| Msg::GameIdChanged(e.value));
        let username_changed = self
            .link
            .callback(|e: InputData| Msg::UsernameChanged(e.value));

        html! {
            <>
                <p>{ "Join a game by entering the instance ID bellow (no list for now)." }</p>
                <br />

                { input_field("Game ID", "Enter the game ID here.", &self.game_id, game_id_changed) }
                { input_field("Player name", "Enter your player name here.", &self.username, username_changed) }

                <div class="control">
                    // TODO: Disable this when empty fields
                    <NavBtn classes="button is-primary" route=join_route>
                        { "Join game" }
                    </NavBtn>
                </div>
            </>
        }
    }
}
