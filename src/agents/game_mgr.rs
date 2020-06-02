use derive_more::From;
use log::*;
use serde::{Deserialize, Serialize};
use yew::worker::*;
use yewtil::NeqAssign;

// Re-export this for convenience
pub use yew::agent::{Dispatched, Dispatcher};

use crate::agents::game_ws_mgr::*;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct GameData {
    pub ws_server_addr: String,
}

pub struct GameMgr {
    link: AgentLink<Self>,
    subscribers: Vec<HandlerId>,

    ws_agent: Box<dyn Bridge<GameWsMgr>>,
    ws_status: WebSocketStatus,

    data: GameData,
}

#[derive(Debug, Clone)]
pub enum GameMgrRequest {
    EnsureConnected(GameWsConnectionInfo),
}

#[derive(Debug, Clone)]
pub enum GameMgrResponse {
    WebSocketStatusChanged(WebSocketStatus),
}

#[derive(From, Debug)]
pub enum Msg {
    Command(Command),
    Event(Event),
}

#[derive(Debug)]
pub enum Command {
    Update,
}

#[derive(Debug, From)]
pub enum Event {
    #[from]
    WebSocketMessage(GameWsResponse),
}

impl Agent for GameMgr {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = GameMgrRequest;
    type Output = GameMgrResponse;

    fn create(link: AgentLink<Self>) -> Self {
        link.send_message(Command::Update);
        let ws_msg_callback = link.callback(Event::WebSocketMessage);
        Self {
            link,
            subscribers: Vec::with_capacity(10),

            ws_agent: GameWsMgr::bridge(ws_msg_callback),
            ws_status: WebSocketStatus::NotConnected,

            data: GameData::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Command(command) => match command {
                Command::Update => {
                    self.ws_agent.send(GameWsRequest::GetWebSocketStatus);
                }
            },
            Msg::Event(event) => match event {
                Event::WebSocketMessage(ws_msg) => {
                    let status_changed = match ws_msg {
                        GameWsResponse::Connecting(info) => {
                            self.update_ws_status(WebSocketStatus::Pending(info))
                        }
                        GameWsResponse::Connected(info) => {
                            self.update_ws_status(WebSocketStatus::Connected(info))
                        }
                        GameWsResponse::Closed => {
                            self.update_ws_status(WebSocketStatus::NotConnected)
                        }
                        GameWsResponse::FailedToConnect(_reason) => {
                            self.update_ws_status(WebSocketStatus::NotConnected)
                        }
                        GameWsResponse::ErrorOccurred => {
                            self.update_ws_status(WebSocketStatus::NotConnected)
                        }
                        GameWsResponse::Received(ws_msg) => {
                            log::debug!("Received: {:?}", ws_msg);
                            false
                        }
                        GameWsResponse::ReceivedError(_error) => false,
                        GameWsResponse::WebSocketStatus(status) => self.update_ws_status(status),
                    };
                    if status_changed {
                        self.broadcast_to_subscribers(GameMgrResponse::WebSocketStatusChanged(
                            self.ws_status.clone(),
                        ));
                    }
                }
            },
        }
    }

    fn handle_input(&mut self, input: Self::Input, sender: HandlerId) {
        trace!("Notification received from '{:?}': {:?}", sender, input);
        match input {
            GameMgrRequest::EnsureConnected(conn_info) => {
                self.ws_agent.send(GameWsRequest::JoinRound(conn_info));
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        trace!("New connection to game manager: {:?}", id);
        self.subscribers.push(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        trace!("Notification listener disconnected: {:?}", id);
        if let Some(pos) = self.subscribers.iter().position(|x| *x == id) {
            self.subscribers.swap_remove(pos);
        }
    }
}

type Changed = bool;

impl GameMgr {
    fn update_ws_status(&mut self, status: WebSocketStatus) -> Changed {
        self.ws_status.neq_assign(status)
    }

    fn broadcast_to_subscribers(&mut self, output: GameMgrResponse) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, output.clone());
        }
    }
}
