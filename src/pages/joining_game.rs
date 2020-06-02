use anyhow::Result;
use log::*;
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yew_router::route::Route;

use crate::agents::game_ws_mgr::*;
use crate::html::*;
use crate::routes::AppRoute;
use crate::services::game_server::*;

pub struct JoiningGame {
    link: ComponentLink<Self>,
    game_server: GameServerService,
    game_ws_mgr: Box<dyn Bridge<GameWsMgr>>,

    current_task: Option<Box<dyn Task>>,

    step: JoinStep,
    game_id: String,
    username: String,
}

#[derive(Debug)]
pub enum JoinStep {
    WantToJoinGame,
    JoiningGame,
    JoinedGameWebSocketPending {
        player_id: String,
    },
    JoinedGameWithWebSocket {
        player_id: String,
    },
    WaitingRedirect,
    JoinFailed {
        player_id: Option<String>,
        error: String,
    },

    /// This variant is used when executing transitions. If it is observed
    /// outside of a transition it means that the transition ended unexpectedly.
    Failed,
}

#[derive(Debug)]
pub enum Msg {
    // Commands
    JoinRound,

    // Events
    JoinRoundResponse(Result<JoinRoundResponse>),
    GameWsResponse(GameWsResponse),
    SuccessfullyJoined,
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
        error!("JoiningGame: created with {:?}", props);

        link.send_message(Msg::JoinRound);
        let game_ws_mgr_callback = link.callback(Msg::GameWsResponse);
        JoiningGame {
            link,
            game_server: GameServerService::new(),
            game_ws_mgr: GameWsMgr::bridge(game_ws_mgr_callback),
            current_task: None,
            game_id: props.game_id,
            username: props.username,
            step: JoinStep::WantToJoinGame,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        error!("JoiningGame: changed with {:?}", props);
        // TODO: JoiningGame change()
        // trace!("Changed: {:?}", props);
        // self.game_id = props.game_id;
        // self.username = props.username;
        // self.step = JoinStep::WantToJoinGame;
        // self.link.send_message(Msg::JoinRound);
        // true
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        trace!("JoiningGame: updated: {:?}", msg);
        let current_step = std::mem::replace(&mut self.step, JoinStep::Failed);
        self.step = match (current_step, msg) {
            (JoinStep::WantToJoinGame, Msg::JoinRound) => {
                self.current_task = Some(Box::new(self.game_server.join_round(
                    &self.game_id,
                    &self.username,
                    self.link.callback(Msg::JoinRoundResponse),
                )));
                JoinStep::JoiningGame
            }

            (JoinStep::JoiningGame, Msg::JoinRoundResponse(Ok(response))) => {
                let player_id = response.player_id;
                self.game_ws_mgr
                    .send(GameWsRequest::JoinRound(GameWsConnectionInfo {
                        game_id: self.game_id.clone(),
                        player_id: player_id.clone(),
                    }));
                JoinStep::JoinedGameWebSocketPending { player_id }
            }
            (step, Msg::JoinRoundResponse(Err(err))) => JoinStep::JoinFailed {
                player_id: step.into_player_id_or_none(),
                error: format!("error joining the round: {}", err),
            },

            (
                JoinStep::JoinedGameWebSocketPending { player_id },
                Msg::GameWsResponse(GameWsResponse::Connected(_)),
            ) => {
                let duration = Duration::from_secs(3);
                let callback = self.link.callback(|_| Msg::SuccessfullyJoined);
                self.current_task =
                    Some(Box::new(IntervalService::new().spawn(duration, callback)));

                JoinStep::JoinedGameWithWebSocket { player_id }
            }
            (step, Msg::GameWsResponse(GameWsResponse::ErrorOccurred)) => JoinStep::JoinFailed {
                player_id: step.into_player_id_or_none(),
                error: "Unknown error occurred while connecting.".into(),
            },
            (step, Msg::GameWsResponse(GameWsResponse::FailedToConnect(reason))) => {
                JoinStep::JoinFailed {
                    player_id: step.into_player_id_or_none(),
                    error: format!("Failed to connect: {}", reason),
                }
            }
            (step, Msg::GameWsResponse(_)) => step,

            (JoinStep::JoinedGameWithWebSocket { player_id, .. }, Msg::SuccessfullyJoined) => {
                let route: Route = AppRoute::PlayGame {
                    game_id: self.game_id.clone(),
                    player_id,
                }
                .into();
                RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute(route));
                JoinStep::WaitingRedirect
            }

            (step, command) => {
                error!("Impossible transition: {:?}", (&step, &command));
                step
            }
        };
        trace!("New state: {:?}", self.step);
        true
    }

    fn view(&self) -> Html {
        match &self.step {
            JoinStep::WantToJoinGame => html! {
                <>
                    <h3 class="title is-size-4">{ "Joining game..." }</h3>
                    <ProgressBar progress=(1.0 / 5.0) class="is-primary" />
                    <p>{ "Game ID: " }{ &self.game_id }</p>
                    <p>{ "Player name: " }{ &self.username }</p>
                </>
            },
            JoinStep::JoiningGame { .. } => html! {
                <>
                    <h3 class="title is-size-4">{ "Joining game..." }</h3>
                    <ProgressBar progress=(2.0 / 5.0) class="is-primary" />
                    <p>{ "Game ID: " }{ &self.game_id }</p>
                    <p>{ "Player name: " }{ &self.username }</p>
                </>
            },
            JoinStep::JoinedGameWebSocketPending { player_id } => html! {
                <>
                    <h3 class="title is-size-4">{ "Starting session..." }</h3>
                    <ProgressBar progress=(3.0 / 5.0) class="is-primary" />
                    <p>{ "Game ID: " }{ &self.game_id }</p>
                    <p>{ "Player name: " }{ &self.username }</p>
                    <p>{ "Player ID: " }{ player_id }</p>
                </>
            },
            JoinStep::JoinedGameWithWebSocket { player_id, .. } => html! {
                <>
                    <h3 class="title is-size-4">{ "Connected! Almost there..." }</h3>
                    <ProgressBar progress=(4.0 / 5.0) class="is-primary" />
                    <p>{ "Game ID: " }{ &self.game_id }</p>
                    <p>{ "Player name: " }{ &self.username }</p>
                    <p>{ "Player ID: " }{ player_id }</p>
                </>
            },
            JoinStep::WaitingRedirect => html! {
                <>
                    <h3 class="title is-size-4">{ "Enjoy :)" }</h3>
                    <ProgressBar progress=(5.0 / 5.0) class="is-primary" />
                    <p>{ "Game ID: " }{ &self.game_id }</p>
                    <p>{ "Player name: " }{ &self.username }</p>
                </>
            },
            JoinStep::JoinFailed { player_id, error } => html! {
                <>
                    <h3 class="title is-size-4">{ "Failed to connect." }</h3>
                    <ProgressBar progress=1.0 class="is-danger" />
                    <p>{ "Game ID: " }{ &self.game_id }</p>
                    <p>{ "Player name: " }{ &self.username }</p>
                    <p>{ "Player ID: " }{ format!("{:?}", player_id) }</p>
                    <p>{ "Error: " }{ error }</p>
                </>
            },
            JoinStep::Failed => html! {
                <>
                    <h3 class="title is-size-4">{ "Couldn't join, please retry later." }</h3>
                    <ProgressBar progress=1.0 class="is-danger" />
                </>
            },
        }
    }
}

impl JoinStep {
    fn into_player_id_or_none(self) -> Option<String> {
        match self {
            JoinStep::JoinedGameWebSocketPending { player_id, .. } => Some(player_id),
            JoinStep::JoinedGameWithWebSocket { player_id, .. } => Some(player_id),
            JoinStep::JoinFailed { player_id, .. } => player_id,
            _ => None,
        }
    }
}
