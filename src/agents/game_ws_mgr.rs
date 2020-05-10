#![allow(unused_imports)] // TODO: Clean imports

use anyhow::Result;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::worker::*;

// Re-export this for convenience
pub use yew::agent::{Dispatched, Dispatcher};

pub struct GameWsMgr {
    link: AgentLink<Self>,
    subscribers: Vec<HandlerId>,

    ws_service: WebSocketService,
    ws: WebSocketConnection,

    ws_history: Vec<String>, // TODO: Try using Cows
}

#[derive(Debug)]
pub enum WebSocketConnection {
    None,
    Pending(WebSocketTask),
    Connected(WebSocketTask),
}

impl WebSocketConnection {
    fn is_none(&self) -> bool {
        match &self {
            Self::None => true,
            _ => false,
        }
    }

    fn is_pending(&self) -> bool {
        match &self {
            Self::Pending(_) => true,
            _ => false,
        }
    }

    fn is_connected(&self) -> bool {
        match &self {
            Self::Connected(_) => true,
            _ => false,
        }
    }

    fn connected(&mut self) {
        *self = match std::mem::replace(self, WebSocketConnection::None) {
            WebSocketConnection::Pending(ws) => WebSocketConnection::Connected(ws),
            ws => {
                error!("Ignoring incoherent connected message, status is {:?}.", ws);
                ws
            }
        };
    }
}

#[derive(Debug)]
pub enum Msg {
    FailedToConnect(String),
    WsNotification(WebSocketStatus),
    WsReceived(Result<WsResponse>), // TODO: Try use Cow or Rc
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameWsRequest {
    ConnectSocket { address: String },
    CloseSocket,
    Send(WsRequest),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameWsResponse {
    Closed,
    Connecting,
    Connected,
    FailedToConnect(String),
    ErrorOccurred,
    Received(WsResponse),
    ReceivedError(String), // TODO: Refine error type
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Deserialize, Debug)]
pub struct WsRequest(serde_json::Value);

/// This type is an expected response from a websocket connection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsResponse(serde_json::Value);

impl Agent for GameWsMgr {
    type Reach = Context;
    type Message = Msg;
    type Input = GameWsRequest;
    type Output = GameWsResponse;

    fn create(link: AgentLink<Self>) -> Self {
        GameWsMgr {
            link,
            subscribers: Vec::with_capacity(10), // TODO: Tune capacities
            ws_service: WebSocketService::new(),
            ws: WebSocketConnection::None,
            ws_history: Vec::with_capacity(500),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        trace!("Agent update: {:?}", msg);
        match msg {
            Msg::FailedToConnect(reason) => {
                self.broadcast_to_subscribers(GameWsResponse::FailedToConnect(reason))
            }
            Msg::WsNotification(status) => {
                let out = match status {
                    WebSocketStatus::Opened => GameWsResponse::Connected,
                    WebSocketStatus::Closed => GameWsResponse::Closed,
                    WebSocketStatus::Error => GameWsResponse::ErrorOccurred,
                };
                self.broadcast_to_subscribers(out);
            }
            Msg::WsReceived(data) => {
                let out = match data {
                    Ok(data) => GameWsResponse::Received(data),
                    Err(err) => GameWsResponse::ReceivedError(err.to_string()),
                };
                self.broadcast_to_subscribers(out);
            }
        }
    }

    fn handle_input(&mut self, input: Self::Input, _sender: HandlerId) {
        trace!("Received in GameWsMgr from {:?}: {:?}", _sender, input);
        match input {
            GameWsRequest::ConnectSocket { address } => {
                let callback = self.link.callback(|Json(data)| Msg::WsReceived(data));
                let notification = self.link.callback(Msg::WsNotification);
                match self.ws_service.connect(&address, callback, notification) {
                    Ok(task) => {
                        self.ws_history
                            .push(format!("Connecting to {}...", address));
                        self.ws = WebSocketConnection::Pending(task);

                        self.broadcast_to_subscribers(GameWsResponse::Connecting);
                    }
                    Err(e) => {
                        let err = format!("WebSocket connection failed: {}", e);
                        error!("{}", err);
                        self.ws_history.push(err.clone());
                        trace!("Before send_message");
                        self.link.send_message(Msg::FailedToConnect(err));
                        trace!("After send_message");
                    }
                }
            }
            GameWsRequest::CloseSocket => {
                self.ws_history.push(format!("Closed"));
                self.ws = WebSocketConnection::None;
            }
            GameWsRequest::Send(data) => {
                if let WebSocketConnection::Connected(ws) = &mut self.ws {
                    ws.send(Json(&data));
                    self.ws_history.push(format!("> {}", data.0));
                } else {
                    error!("Tried to send on non-opened WebSocket. Ignoring.");
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        trace!("New connection: {:?}", id);
        if !self.subscribers.contains(&id) {
            self.subscribers.push(id);
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        // Remove the subscriber (efficiently, hence swap_remove)
        if let Some(pos) = self.subscribers.iter().position(|x| *x == id) {
            self.subscribers.swap_remove(pos);
            trace!("Subscriber disconnected: {:?}", id);
        } else {
            warn!("Disconnection but no associated subscriber.");
        }
    }
}

impl GameWsMgr {
    fn broadcast_to_subscribers(&self, output: GameWsResponse) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, output.clone());
        }
    }
}
