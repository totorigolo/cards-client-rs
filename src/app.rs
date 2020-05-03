use anyhow::Result;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

const KEY: &str = "cards-client-rs.state";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
    ws_service: WebSocketService,
    ws: WebSocketConnection,
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

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    value: String,
}

#[derive(Debug)]
pub enum Msg {
    Update(String),
    WebSocket(WsMsg),
    #[allow(unused)]
    Ignore,
}

type AsBinary = bool;

#[derive(Debug)]
pub enum WsMsg {
    Close,
    Closed,
    Connect,
    Connected,
    ErrorOccurred,
    Received(Result<WsResponse>),
    Send(AsBinary),
}

// pub enum Msg {
//     FetchData(Format, AsBinary),
//     WsAction(WsAction),
//     FetchReady(Result<DataFromFile, Error>),
//     WsReady(Result<WsResponse, Error>),
//     Ignore,
// }

impl From<WsMsg> for Msg {
    fn from(msg: WsMsg) -> Self {
        Msg::WebSocket(msg)
    }
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
// struct WsRequest {
//     #[serde(rename = "type")]
//     _type: String,
// }
pub struct WsRequest(serde_json::Value);

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
// pub struct WsResponse {
//     #[serde(rename = "type")]
//     _type: String,
// }
pub struct WsResponse(serde_json::Value);

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();

        let state = match storage.restore(KEY) {
            Json(Ok(restored_state)) => restored_state,
            _ => State {
                ..Default::default()
            },
        };

        App {
            link,
            storage,
            state,
            ws_service: WebSocketService::new(),
            ws: WebSocketConnection::None,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        trace!("Msg: {:?}", msg);
        match msg {
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::WebSocket(action) => match action {
                WsMsg::Connect => {
                    let callback = self.link.callback(|Json(data)| WsMsg::Received(data));
                    let notification = self.link.callback(|status| match status {
                        WebSocketStatus::Opened => Msg::WebSocket(WsMsg::Connected),
                        WebSocketStatus::Closed => Msg::WebSocket(WsMsg::Closed),
                        WebSocketStatus::Error => Msg::WebSocket(WsMsg::ErrorOccurred),
                    } as Msg);
                    let task = self
                        .ws_service
                        .connect(&self.state.value, callback, notification)
                        .unwrap();
                    self.ws = WebSocketConnection::Pending(task);
                }
                WsMsg::Connected => {
                    self.ws.connected();
                    trace!("{:?}", self.ws);
                }
                WsMsg::Send(binary) => {
                    if let WebSocketConnection::Connected(ws) = &mut self.ws {
                        let request = WsRequest(json!({"type": "PING"}));
                        if binary {
                            ws.send_binary(Json(&request));
                        } else {
                            ws.send(Json(&request));
                        }
                    } else {
                        error!("Tried to send on non-opened WS: {:?}", "todo");
                    }
                }
                WsMsg::Received(data) => {
                    //self.data = data.map(|data| data.value).ok();
                    info!("Received: {:?}", data);
                }
                WsMsg::Close | WsMsg::Closed => {
                    info!("WebSocket closed.");
                    self.ws = WebSocketConnection::None;
                }
                WsMsg::ErrorOccurred => {
                    info!("An error occurred on WebSocket.");
                    self.ws = WebSocketConnection::None;
                }
            },
            Msg::Ignore => return false,
        }
        self.storage.store(KEY, Json(&self.state));
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <>
                { self.view_input() }
                <br/>
                { &self.state.value }
                <br/>
                <button disabled=!self.ws.is_none()
                        onclick=self.link.callback(|_| WsMsg::Connect)>
                    { if self.ws.is_pending() { "Connecting to WebSocket..." } else { "Connect to WebSocket" } }
                </button>
                <br/>
                <button disabled=!self.ws.is_connected()
                        onclick=self.link.callback(|_| WsMsg::Send(false))>
                    { "Send To WebSocket" }
                </button>
                <br/>
                <button disabled=!self.ws.is_connected()
                        onclick=self.link.callback(|_| WsMsg::Send(true))>
                    { "Send To WebSocket [binary]" }
                </button>
                <br/>
                <button disabled=self.ws.is_none()
                        onclick=self.link.callback(|_| WsMsg::Close)>
                    { "Close WebSocket connection" }
                </button>
            </>
        }
    }
}

impl App {
    fn view_input(&self) -> Html {
        html! {
            <input class="parrot"
                   placeholder="Type anything here..."
                   value=&self.state.value
                   oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                   />
        }
    }
}

impl State {}
