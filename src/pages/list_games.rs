use yew::prelude::*;

use crate::html::*;
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

                <TextInputField
                    label="Game ID"
                    placeholder="Enter the ID of the round to join."
                    value=&self.game_id
                    oninput=game_id_changed
                    />
                <TextInputField
                    label="Player name"
                    placeholder="Enter your player name here."
                    value=&self.username
                    oninput=username_changed
                    />

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
