#![allow(unused_imports)] // TODO: Clean imports

use anyhow::Result;
use derive_more::From;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::websocket::{
    WebSocketService, WebSocketStatus as YewWebSocketStatus, WebSocketTask,
};
use yew::worker::*;

// Re-export this for convenience
pub use yew::agent::{Bridge, Bridged, Dispatched, Dispatcher};

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

#[derive(Debug)]
pub enum Msg {
    FailedToConnect(String),
    WsNotification(YewWebSocketStatus),
    WsReceived(Result<WsResponse>), // TODO: Try use Cow or Rc
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameWsRequest {
    CloseSocket,
    JoinRound { game_id: String, player_id: String },
    Send(WsRequest),
    GetWebSocketStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, From)]
pub enum GameWsResponse {
    Closed,
    Connected,
    Connecting,
    ErrorOccurred,
    FailedToConnect(String),
    Received(WsResponse),
    ReceivedError(String), // TODO: Refine error type
    #[from]
    WebSocketStatus(WebSocketStatus),
}

/// Represents the state of the WebSocket. Differs from WebSocketConnection in
/// that this is sent to subscribers, when WebSocketConnection holds the actual
/// connection.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WebSocketStatus {
    NotConnected,
    Pending,
    Connected,
}

impl From<&WebSocketConnection> for WebSocketStatus {
    fn from(conn: &WebSocketConnection) -> Self {
        match conn {
            WebSocketConnection::None => WebSocketStatus::NotConnected,
            WebSocketConnection::Pending(_) => WebSocketStatus::Pending,
            WebSocketConnection::Connected(_) => WebSocketStatus::Connected,
        }
    }
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Deserialize, Debug)]
pub struct WsRequest(pub serde_json::Value);

/// This type is an expected response from a websocket connection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsResponse(pub serde_json::Value);

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
                let current_ws = std::mem::replace(&mut self.ws, WebSocketConnection::None);
                self.ws = match (current_ws, &status) {
                    (WebSocketConnection::Pending(ws), YewWebSocketStatus::Opened) => {
                        WebSocketConnection::Connected(ws)
                    }
                    _ => WebSocketConnection::None,
                };
                let out = match status {
                    YewWebSocketStatus::Opened => GameWsResponse::Connected,
                    YewWebSocketStatus::Closed => GameWsResponse::Closed,
                    YewWebSocketStatus::Error => GameWsResponse::ErrorOccurred,
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

    fn handle_input(&mut self, input: Self::Input, sender: HandlerId) {
        trace!("Received in GameWsMgr from {:?}: {:?}", sender, input);
        match input {
            GameWsRequest::JoinRound { game_id, player_id } => {
                let url = format!("/api/round/{}/join?playerId={}", game_id, player_id);

                let url = web_sys::Url::new_with_base(
                    &url,
                    &web_sys::window().unwrap().location().href().unwrap(),
                )
                .expect("TODO");
                url.set_protocol("ws");
                let url = url.href();
                error!("URL: {:?}", url);
                // url.protocol = url.protocol.replace('http', 'ws');
                // url.href // => ws://www.example.com:9999/path/to/websocket

                warn!("Connecting using URL: {}", url);

                let callback = self.link.callback(|Json(data)| Msg::WsReceived(data));
                let notification = self.link.callback(Msg::WsNotification);
                match self.ws_service.connect(&url, callback, notification) {
                    Ok(task) => {
                        self.ws_history.push(format!("Connecting to {}...", url));
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
            GameWsRequest::GetWebSocketStatus => {
                self.link
                    .respond(sender, WebSocketStatus::from(&self.ws).into());
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
