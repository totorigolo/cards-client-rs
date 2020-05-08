use anyhow::Result;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use crate::agents::notifications::*;

pub struct WsExperiment {
    link: ComponentLink<Self>,
    notification_bus: Dispatcher<NotificationBus>,

    ws_server_addr: String,
    ws_service: WebSocketService,
    ws: WebSocketConnection,

    msg_history: Vec<String>,
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

pub enum Msg {
    WsServerAddrUpdated(String),
    WebSocket(WsMsg),
}

#[derive(Debug)]
pub enum WsMsg {
    Close,
    Closed,
    Connect,
    Connected,
    ErrorOccurred,
    Received(Result<WsResponse>),
    Send(WsRequest),
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
pub struct WsRequest(serde_json::Value);

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse(serde_json::Value);

impl From<WsMsg> for Msg {
    fn from(msg: WsMsg) -> Self {
        Msg::WebSocket(msg)
    }
}

impl NotificationSender for WsExperiment {
    fn notification_bus(&mut self) -> &mut Dispatcher<NotificationBus> {
        &mut self.notification_bus
    }
}


impl Component for WsExperiment {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        WsExperiment {
            link,
            notification_bus: NotificationBus::dispatcher(),

            ws_server_addr: String::from(""),
            ws_service: WebSocketService::new(),
            ws: WebSocketConnection::None,
            msg_history: vec![],
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsServerAddrUpdated(addr) => {
                println!("Input: {}", addr);
                self.ws_server_addr = addr;
            }
            Msg::WebSocket(action) => match action {
                WsMsg::Connect => {
                    let callback = self.link.callback(|Json(data)| WsMsg::Received(data));
                    let notification = self.link.callback(|status| match status {
                        WebSocketStatus::Opened => Msg::WebSocket(WsMsg::Connected),
                        WebSocketStatus::Closed => Msg::WebSocket(WsMsg::Closed),
                        WebSocketStatus::Error => Msg::WebSocket(WsMsg::ErrorOccurred),
                    } as Msg);
                    match self
                        .ws_service
                        .connect(&self.ws_server_addr, callback, notification)
                    {
                        Ok(task) => {
                            self.ws = WebSocketConnection::Pending(task);
                        }
                        Err(e) => error!("Failed to open WS: {}", e),
                    }
                }
                WsMsg::Connected => {
                    self.notify_info("WebSocket connected.");
                    self.ws.connected();
                }
                WsMsg::Send(data) => {
                    trace!("Sending on WS: {:?}", data);
                    if let WebSocketConnection::Connected(ws) = &mut self.ws {
                        ws.send(Json(&data));
                        self.msg_history.push(format!("> {}", data.0));
                    } else {
                        error!("Tried to send on non-opened WS.");
                    }
                }
                WsMsg::Received(data) => {
                    info!("Received: {:?}", data);
                    match data {
                        Ok(response) => self.msg_history.push(format!("< {}", response.0)),
                        Err(e) => self.msg_history.push(format!("< ERROR: {}", e)),
                    }
                }
                WsMsg::Close | WsMsg::Closed => {
                    self.notify_info("WebSocket closed.");
                    self.ws = WebSocketConnection::None;
                }
                WsMsg::ErrorOccurred => {
                    self.notify_error("An error occurred on WebSocket.");
                    self.ws = WebSocketConnection::None;
                }
            },
        }
        true
    }

    fn view(&self) -> Html {
        let mk_ping = || WsRequest(json!({"type": "PING"}));
        let loading_class = { if self.ws.is_pending() { "is-loading" } else { "" } };
        html! {
            <>
                <input
                    placeholder="WebSocket server address"
                    value=&self.ws_server_addr
                    oninput=self.link.callback(|e: InputData| Msg::WsServerAddrUpdated(e.value))
                    />
                <br/>
                { &self.ws_server_addr }
                <br/>
                <div class="buttons">
                    <button class=("button", "is-primary", loading_class)
                            disabled=!self.ws.is_none()
                            onclick=self.link.callback(|_| WsMsg::Connect)>
                        { "Connect to WebSocket" }
                    </button>
                    <button class="button"
                            disabled=!self.ws.is_connected()
                            onclick=self.link.callback(move |_| WsMsg::Send(mk_ping()))>
                        { "Send To WebSocket" }
                    </button>
                    <button class=("button", "is-danger is-outlined")
                            disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsMsg::Close)>
                        { "Close WebSocket connection" }
                    </button>
                </div>
                <br/>
                <h2 class="title is-size-4">{ "Message history" }</h2>
                <pre>
                    { self.msg_history.join("\n") }
                </pre>
            </>
        }
    }
}
