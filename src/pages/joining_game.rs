use anyhow::{Error, Result};
use derive_more::From;
use log::*;
use yew::prelude::*;

use crate::agents::game_ws_mgr::*;
use crate::services::game_server::*;

pub struct JoiningGame {
    link: ComponentLink<Self>,
    game_server: GameServerService,
    game_ws_mgr: Dispatcher<GameWsMgr>,
    state: JoinState,
}

enum JoinState {
    WantToJoinGame {
        game_id: String,
        username: String,
    },
    JoiningGame {
        game_id: String,
        username: String,
        fetch_task: FetchTask,
    },
    JoinedGameNoWebSocket {
        game_id: String,
        username: String,
        player_id: String,
    },
    JoinedGameWebSocketPending {
        game_id: String,
        username: String,
        player_id: String,
    },
    JoinedGameWithWebSocket {
        game_id: String,
        username: String,
        player_id: String,
    },
    JoinFailed {
        game_id: String,
        username: String,
        error: Error,
    },

    // This variant is almost a dummy one, temporarily used when changing state.
    // Having this as the result of a transition means that the code panicked.
    FatalError,
}

#[derive(Debug, From)]
pub enum Msg {
    #[from]
    Command(Command),
    #[from]
    Event(Event),
}

#[derive(Debug)]
pub enum Command {
    Progress,
}

#[derive(Debug)]
pub enum Event {
    JoinRoundResponse(Result<JoinRoundResponse>),
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub game_id: String,
    pub username: String,
}

impl Component for JoiningGame {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Command::Progress);
        JoiningGame {
            link,
            game_server: GameServerService::new(),
            game_ws_mgr: GameWsMgr::dispatcher(),

            state: JoinState::WantToJoinGame {
                game_id: props.game_id,
                username: props.username,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.state = JoinState::WantToJoinGame {
            game_id: props.game_id,
            username: props.username,
        };
        self.link.send_message(Command::Progress);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        trace!("Update: {:?}", msg);
        match msg {
            Msg::Command(Command::Progress) => match std::mem::replace(&mut self.state, JoinState::FatalError) {
                JoinState::WantToJoinGame { game_id, username } => {
                    let task = self.game_server.join_round(
                        &game_id,
                        &username,
                        self.link.callback(Event::JoinRoundResponse),
                    );
                    self.state = JoinState::JoiningGame {
                        game_id,
                        username,
                        fetch_task: task,
                    };
                }
                JoinState::JoiningGame {
                    game_id,
                    username,
                    fetch_task,
                } => self.state = JoinState::JoinFailed { game_id, username, error: anyhow::anyhow!("JoiningGame") },
                JoinState::JoinedGameNoWebSocket {
                    game_id,
                    username,
                    player_id,
                } =>  self.state = JoinState::JoinFailed { game_id, username, error: anyhow::anyhow!("JoinedGameNoWebSocket") },
                JoinState::JoinedGameWebSocketPending {
                    game_id,
                    username,
                    player_id,
                } =>  self.state = JoinState::JoinFailed { game_id, username, error: anyhow::anyhow!("JoinedGameWebSocketPending") },
                JoinState::JoinedGameWithWebSocket {
                    game_id,
                    username,
                    player_id,
                } =>  self.state = JoinState::JoinFailed { game_id, username, error: anyhow::anyhow!("JoinedGameWithWebSocket") },
                JoinState::JoinFailed {
                    game_id,
                    username,
                    error,
                } =>  self.state = JoinState::JoinFailed { game_id, username, error: anyhow::anyhow!("JoinFailed") },
                JoinState::FatalError => self.state = JoinState::FatalError,
            },
            Msg::Event(event) => match event {
                Event::JoinRoundResponse(Ok(response)) => {
                    info!("Join succeeded: {:?}", response);
                    self.link.send_message(Command::Progress);
                }
                Event::JoinRoundResponse(Err(err)) => self.state.to_failed(err),
            },
            // Msg::ConnectWebSocket => {
            //     let addr = format!(
            //         "ws://127.0.0.1:8080/round/{}/join?username={}",
            //         self.props.game_id,
            //         self.props.username,
            //     );
            // }
        }
        true
    }

    fn view(&self) -> Html {
        match &self.state {
            JoinState::WantToJoinGame { game_id, username } => html! {
                <>
                    <h3 class="title is-size-4">{ "Joining game..." }</h3>
                    { progress_bar(1.0 / 5.0, "is-primary") }
                    <p>{ "Game ID: " }{ game_id }</p>
                    <p>{ "Player name: " }{ username }</p>
                </>
            },
            JoinState::JoiningGame {
                game_id, username, ..
            } => html! {
                <>
                    <h3 class="title is-size-4">{ "Joining game..." }</h3>
                    { progress_bar(2.0 / 5.0, "is-primary") }
                    <p>{ "Game ID: " }{ game_id }</p>
                    <p>{ "Player name: " }{ username }</p>
                </>
            },
            JoinState::JoinedGameNoWebSocket {
                game_id,
                username,
                player_id,
            } => html! {
                <>
                    <h3 class="title is-size-4">{ "Starting session..." }</h3>
                    { progress_bar(3.0 / 5.0, "is-primary") }
                    <p>{ "Game ID: " }{ game_id }</p>
                    <p>{ "Player name: " }{ username }</p>
                    <p>{ "Player ID: " }{ player_id }</p>
                </>
            },
            JoinState::JoinedGameWebSocketPending {
                game_id,
                username,
                player_id,
            } => html! {
                <>
                    <h3 class="title is-size-4">{ "Starting session..." }</h3>
                    { progress_bar(4.0 / 5.0, "is-primary") }
                    <p>{ "Game ID: " }{ game_id }</p>
                    <p>{ "Player name: " }{ username }</p>
                    <p>{ "Player ID: " }{ player_id }</p>
                </>
            },
            JoinState::JoinedGameWithWebSocket {
                game_id,
                username,
                player_id,
            } => html! {
                <>
                    <h3 class="title is-size-4">{ "Connected! Just one moment..." }</h3>
                    { progress_bar(5.0 / 5.0, "is-primary") }
                    <p>{ "Game ID: " }{ game_id }</p>
                    <p>{ "Player name: " }{ username }</p>
                    <p>{ "Player ID: " }{ player_id }</p>
                </>
            },
            JoinState::JoinFailed {
                game_id,
                username,
                error,
            } => html! {
                <>
                    <h3 class="title is-size-4">{ "Failed to connect." }</h3>
                    { progress_bar(5.0 / 5.0, "is-danger") }
                    <p>{ "Game ID: " }{ game_id }</p>
                    <p>{ "Player name: " }{ username }</p>
                    <p>{ "Error: " }{ error }</p>
                </>
            },
            JoinState::FatalError => html! {
                <>
                    <h3 class="title is-size-4">{ "Couldn't join, please retry later." }</h3>
                    { progress_bar(5.0 / 5.0, "is-danger") }
                </>
            },
        }
    }
}

/// Progress must be 0 <= x <= 1, or it will be clamped otherwise.
fn progress_bar(progress: f32, class: impl AsRef<str>) -> Html {
    let percentage = ((progress * 100.0).round() as u32).min(100).max(0);
    html! {
        <progress class=("progress", class.as_ref()) value=percentage max="100">{ percentage }{ "%" }</progress>
    }
}

impl JoinState {
    fn to_failed(&mut self, error: Error) {
        let previous: JoinState = std::mem::replace(self, JoinState::FatalError);
        *self = match previous {
            JoinState::WantToJoinGame { game_id, username } => JoinState::JoinFailed {
                game_id,
                username,
                error,
            },
            JoinState::JoiningGame {
                game_id,
                username,
                ..
            } => JoinState::JoinFailed {
                game_id,
                username,
                error,
            },
            JoinState::JoinedGameNoWebSocket {
                game_id,
                username,
                ..
            } => JoinState::JoinFailed {
                game_id,
                username,
                error,
            },
            JoinState::JoinedGameWebSocketPending {
                game_id,
                username,
                ..
            } => JoinState::JoinFailed {
                game_id,
                username,
                error,
            },
            JoinState::JoinedGameWithWebSocket {
                game_id,
                username,
                ..
            } => JoinState::JoinFailed {
                game_id,
                username,
                error,
            },
            JoinState::JoinFailed {
                game_id,
                username,
                error: previous_error,
            } => JoinState::JoinFailed {
                game_id,
                username,
                error: error.context(previous_error),
            },
            JoinState::FatalError => JoinState::FatalError,
        };
    }
}
