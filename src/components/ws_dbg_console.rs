use derive_more::From;
use serde_json::json;
use std::collections::VecDeque;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::agents::game_ws_mgr::*;
use crate::agents::notifications::*;
use crate::html::*;

const MAX_HISTORY_LEN: usize = 500;

pub struct WebSocketDebugConsole {
    link: ComponentLink<Self>,
    notification_bus: Dispatcher<NotificationBus>,

    ws_agent: Box<dyn Bridge<GameWsMgr>>,
    ws_status: WebSocketStatus,

    ws_history: VecDeque<(usize, String)>,
    ws_history_last_id: usize,

    game_id: String,
    player_id: String,
    ws_message: String,
}

#[derive(From, Debug)]
pub enum Msg {
    Command(Command),
    Event(Event),
}

#[derive(Debug)]
pub enum Command {
    Update,
    ConnectWebSocket,
    SendPing,
    SendMessage,
    CloseWebSocket,
}

#[derive(Debug, From)]
pub enum Event {
    GameIdChanged(String),
    PlayerIdChanged(String),
    WsMessageChanged(String),
    #[from]
    WebSocketMessage(GameWsResponse),
}

impl NotificationSender for WebSocketDebugConsole {
    fn notification_bus(&mut self) -> &mut Dispatcher<NotificationBus> {
        &mut self.notification_bus
    }
}

impl Component for WebSocketDebugConsole {
    type Message = Msg;
    type Properties = ();

    fn create(_props: (), link: ComponentLink<Self>) -> Self {
        link.send_message(Command::Update);
        let ws_msg_callback = link.callback(Event::WebSocketMessage);
        WebSocketDebugConsole {
            link,
            notification_bus: NotificationBus::dispatcher(),

            ws_agent: GameWsMgr::bridge(ws_msg_callback),
            ws_status: WebSocketStatus::NotConnected,
            ws_history: VecDeque::with_capacity(MAX_HISTORY_LEN),
            ws_history_last_id: 0, //std::usize::MAX,

            game_id: String::from(""),
            player_id: String::from(""),
            ws_message: String::from(""),
        }
    }

    fn change(&mut self, _props: ()) -> ShouldRender {
        self.link.send_message(Command::Update);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Command(command) => match command {
                Command::Update => {
                    self.ws_agent.send(GameWsRequest::GetWebSocketStatus);
                    false
                }
                Command::ConnectWebSocket => {
                    self.ws_agent.send(GameWsRequest::JoinRound {
                        game_id: self.game_id.clone(),
                        player_id: self.player_id.clone(),
                    });
                    false
                }
                Command::SendPing => {
                    self.send_on_ws(json!({"type": "PING"}));
                    true
                }
                Command::SendMessage => match serde_json::from_str(&self.ws_message) {
                    Ok(json) => {
                        self.send_on_ws(json);
                        true
                    }
                    Err(e) => {
                        self.push_in_history(format!("ERROR: Message is not correct JSON: {}", e));
                        true
                    }
                },
                Command::CloseWebSocket => {
                    self.ws_agent.send(GameWsRequest::CloseSocket);
                    false
                }
            },
            Msg::Event(event) => match event {
                Event::GameIdChanged(game_id) => self.game_id.neq_assign(game_id),
                Event::PlayerIdChanged(player_id) => self.player_id.neq_assign(player_id),
                Event::WsMessageChanged(ws_message) => self.ws_message.neq_assign(ws_message),
                Event::WebSocketMessage(ws_msg) => match ws_msg {
                    GameWsResponse::Connecting(info) => {
                        self.change_status(WebSocketStatus::Pending(info));
                        self.push_in_history("Connecting...");
                        true
                    }
                    GameWsResponse::Connected(info) => {
                        self.change_status(WebSocketStatus::Connected(info));
                        self.push_in_history("Connected");
                        true
                    }
                    GameWsResponse::Closed => {
                        self.change_status(WebSocketStatus::NotConnected);
                        self.push_in_history("Disconnected");
                        true
                    }
                    GameWsResponse::FailedToConnect(reason) => {
                        self.change_status(WebSocketStatus::NotConnected);
                        self.push_in_history(format!("Failed to connect: {}", reason));
                        true
                    }
                    GameWsResponse::ErrorOccurred => {
                        self.push_in_history("An unknown error occurred.");
                        self.change_status(WebSocketStatus::NotConnected);
                        true
                    }
                    GameWsResponse::Received(data) => {
                        // Try to format (serialize) the data as JSON
                        let as_json =
                            serde_json::to_string(&data).unwrap_or_else(|_| format!("{:?}", &data));

                        self.push_in_history(format!("<- {}", as_json));
                        true
                    }
                    GameWsResponse::ReceivedError(error) => {
                        self.push_in_history(format!(
                            "Failed to decode received data: {:?}",
                            error
                        ));
                        true
                    }
                    GameWsResponse::WebSocketStatus(status) => self.change_status(status),
                },
            },
        }
    }

    fn view(&self) -> Html {
        let connected = self.ws_status.is_connected();
        let pending = self.ws_status.is_pending();
        let loading_class = if pending { Some("is-loading") } else { None };

        let textarea_color = if self.ws_message.is_empty() {
            Some("is-info".to_string())
        } else if serde_json::from_str::<serde_json::Value>(&self.ws_message).is_ok() {
            Some("is-success".to_string())
        } else {
            Some("is-danger".to_string())
        };

        let on_game_id_change = self
            .link
            .callback(|e: InputData| Event::GameIdChanged(e.value));
        let on_player_id_change = self
            .link
            .callback(|e: InputData| Event::PlayerIdChanged(e.value));
        let on_msg_change = self
            .link
            .callback(|e: InputData| Event::WsMessageChanged(e.value));

        html! {
            <div class="columns">
                <div class="column is-two-thirds is-flex is-flex-column">
                    <h2 class="title is-size-4">{ "Message history - " }{ self.ws_history.len() }</h2>
                    <pre class="ws-console">
                        {
                            for self.ws_history.iter().map(|(id, line)| {
                                html! { <p>{ format!("{}. {}", id, line) }</p> }
                            })
                        }
                    </pre>
                </div>
                <div class="column">
                    <TextInputField
                        label="Game ID"
                        placeholder="Enter the ID of the round to join."
                        value=&self.game_id
                        oninput=on_game_id_change
                        disabled=(connected || pending)
                        />
                    <TextInputField
                        label="Player ID"
                        placeholder="Enter the player ID here."
                        value=&self.player_id
                        oninput=on_player_id_change
                        disabled=(connected || pending)
                        />
                    <div class="buttons">
                        <button class=("button is-fullwidth", "is-primary", loading_class)
                                disabled=connected
                                onclick=self.link.callback(|_| Command::ConnectWebSocket)>
                            { "Connect to WebSocket" }
                        </button>
                        <button class=("button is-fullwidth", "is-danger is-outlined")
                                disabled=!(pending || connected)
                                onclick=self.link.callback(|_| Command::CloseWebSocket)>
                            { "Close WebSocket connection" }
                        </button>
                        <button class="button is-fullwidth"
                                disabled=!connected
                                onclick=self.link.callback(move |_| Command::SendPing)>
                            { "Send Ping on WebSocket" }
                        </button>
                    </div>
                    <TextAreaField
                        label="Message to send"
                        placeholder="Enter a message to send on the WebSocket."
                        class=textarea_color
                        value=&self.ws_message
                        oninput=on_msg_change
                        />
                    <button class="button is-fullwidth"
                            disabled=!connected
                            onclick=self.link.callback(move |_| Command::SendMessage)>
                        { "Send" }
                    </button>
                </div>
            </div>
        }
    }
}

impl WebSocketDebugConsole {
    fn change_status(&mut self, status: WebSocketStatus) -> ShouldRender {
        match status.clone() {
            WebSocketStatus::NotConnected => {}
            WebSocketStatus::Pending(info) => {
                self.player_id = info.player_id;
                self.game_id = info.game_id;
            }
            WebSocketStatus::Connected(info) => {
                self.player_id = info.player_id;
                self.game_id = info.game_id;
            }
        }
        self.ws_status.neq_assign(status)
    }

    fn push_in_history(&mut self, line: impl ToString) {
        while self.ws_history.len() >= MAX_HISTORY_LEN {
            self.ws_history.pop_back();
        }

        self.ws_history_last_id = self.ws_history_last_id.wrapping_add(1);

        self.ws_history
            .push_front((self.ws_history_last_id, line.to_string()));
    }

    fn send_on_ws(&mut self, message: serde_json::Value) {
        let json = message.into();
        self.push_in_history(format!("-> {}", &json));
        self.ws_agent.send(GameWsRequest::Send(WsRequest(json)));
    }
}
