use derive_more::From;
use serde_json::json;
use yew::prelude::*;
use yewtil::NeqAssign;
use std::collections::VecDeque;

use crate::agents::game_ws_mgr::*;
use crate::agents::notifications::*;
use crate::html;

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
}

#[derive(From)]
pub enum Msg {
    Command(Command),
    Event(Event),
}

#[derive(Debug)]
pub enum Command {
    Update,
    ConnectWebSocket,
    SendPing,
    CloseWebSocket,
}

#[derive(Debug, From)]
pub enum Event {
    GameIdChanged(String),
    PlayerIdChanged(String),
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
            ws_history_last_id: 0,//std::usize::MAX,

            game_id: String::from(""),
            player_id: String::from(""),
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
                    let json = json!({"type": "PING"});
                    self.ws_agent.send(GameWsRequest::Send(WsRequest(json)));
                    false
                }
                Command::CloseWebSocket => {
                    self.ws_agent.send(GameWsRequest::CloseSocket);
                    false
                }
            },
            Msg::Event(event) => match event {
                Event::GameIdChanged(game_id) => self.game_id.neq_assign(game_id),
                Event::PlayerIdChanged(player_id) => self.player_id.neq_assign(player_id),
                Event::WebSocketMessage(ws_msg) => match ws_msg {
                    GameWsResponse::Connecting => {
                        self.ws_status = WebSocketStatus::Pending;
                        self.push_in_history("Connecting...");
                        true
                    }
                    GameWsResponse::Connected => {
                        self.ws_status = WebSocketStatus::Connected;
                        self.push_in_history("Connected");
                        true
                    }
                    GameWsResponse::Closed => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        self.push_in_history("Disconnected");
                        true
                    }
                    GameWsResponse::FailedToConnect(reason) => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        self.push_in_history(format!("Failed to connect: {}", reason));
                        true
                    }
                    GameWsResponse::ErrorOccurred => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        self.push_in_history("An unknown error occurred.");
                        true
                    }
                    GameWsResponse::Received(data) => {
                        self.ws_status = WebSocketStatus::Connected;

                        // Try to format (serialize) the data as JSON
                        let as_json = serde_json::to_string(&data)
                            .unwrap_or_else(|_| format!("{:?}", &data));

                        self.push_in_history(format!("Received: {}", as_json));
                        true
                    }
                    GameWsResponse::ReceivedError(error) => {
                        self.ws_status = WebSocketStatus::Connected;
                        self.push_in_history(format!("Failed to decode received data: {:?}", error));
                        true
                    }
                    GameWsResponse::WebSocketStatus(status) => self.ws_status.neq_assign(status),
                },
            },
        }
    }

    fn view(&self) -> Html {
        let connected = self.ws_status == WebSocketStatus::Connected;
        let pending = self.ws_status == WebSocketStatus::Pending;
        let loading_class = if pending { Some("is-loading") } else { None };

        let on_game_id_change = self
            .link
            .callback(|e: InputData| Event::GameIdChanged(e.value));
        let on_player_id_change = self
            .link
            .callback(|e: InputData| Event::PlayerIdChanged(e.value));

        html! {
            <div class="columns">
                <div class="column is-two-thirds is-flex is-flex-column">
                    <h2 class="title is-size-4">{ "Message history - " }{ self.ws_history.len() }</h2>
                    <pre class="ws-console">
                        {
                            for self.ws_history.iter().map(|(id, line)| {
                                html! { <p>{ format!("{} - {}", id, line) }</p> }
                            })
                        }
                    </pre>
                </div>
                <div class="column">
                    {
                        html::input_field(
                            "Game ID",
                            "Enter the ID of the round to join.",
                            &self.game_id,
                            on_game_id_change)
                    }
                    {
                        html::input_field(
                            "Player ID",
                            "Enter the player ID here.",
                            &self.player_id,
                            on_player_id_change)
                    }
                    <div class="buttons">
                        <button class=("button is-fullwidth", "is-primary", loading_class)
                                disabled=connected
                                onclick=self.link.callback(|_| Command::ConnectWebSocket)>
                            { "Connect to WebSocket" }
                        </button>
                        <button class="button is-fullwidth"
                                disabled=!connected
                                onclick=self.link.callback(move |_| Command::SendPing)>
                            { "Send To WebSocket" }
                        </button>
                        <button class=("button is-fullwidth", "is-danger is-outlined")
                                disabled=!(pending || connected)
                                onclick=self.link.callback(|_| Command::CloseWebSocket)>
                            { "Close WebSocket connection" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

impl WebSocketDebugConsole {
    fn push_in_history(&mut self, line: impl ToString) {
        while self.ws_history.len() >= MAX_HISTORY_LEN {
            self.ws_history.pop_back();
        }

        self.ws_history_last_id = self.ws_history_last_id.wrapping_add(1);

        self.ws_history.push_front((self.ws_history_last_id, line.to_string()));
    }
}
