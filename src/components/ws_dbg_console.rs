use derive_more::From;
use serde_json::json;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::agents::game_ws_mgr::*;
use crate::agents::notifications::*;
use crate::html;

pub struct WebSocketDebugConsole {
    link: ComponentLink<Self>,
    notification_bus: Dispatcher<NotificationBus>,

    ws_agent: Box<dyn Bridge<GameWsMgr>>,
    ws_status: WebSocketStatus,

    ws_history: Vec<String>,

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
            ws_history: vec![],

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
                        self.ws_history.push(format!("Connecting..."));
                        true
                    }
                    GameWsResponse::Connected => {
                        self.ws_status = WebSocketStatus::Connected;
                        self.ws_history.push(format!("Connected"));
                        true
                    }
                    GameWsResponse::Closed => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        self.ws_history.push(format!("Disconnected"));
                        true
                    }
                    GameWsResponse::FailedToConnect(reason) => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        self.ws_history
                            .push(format!("Failed to connect: {}", reason));
                        true
                    }
                    GameWsResponse::ErrorOccurred => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        self.ws_history.push(format!("An unknown error occurred."));
                        true
                    }
                    GameWsResponse::Received(data) => {
                        self.ws_status = WebSocketStatus::Connected;
                        self.ws_history.push(format!("Received: {:?}", data));
                        true
                    }
                    GameWsResponse::ReceivedError(error) => {
                        self.ws_status = WebSocketStatus::Connected;
                        self.ws_history
                            .push(format!("Failed to decode received data: {:?}", error));
                        true
                    }
                    GameWsResponse::WebSocketStatus(status) => self.ws_status.neq_assign(status),
                },
            },
        }
    }

    fn view(&self) -> Html {
        let connected = self.ws_status == WebSocketStatus::Connected;
        let loading_class = if connected { Some("is-loading") } else { None };

        let on_game_id_change = self
            .link
            .callback(|e: InputData| Event::GameIdChanged(e.value));
        let on_player_id_change = self
            .link
            .callback(|e: InputData| Event::PlayerIdChanged(e.value));

        html! {
            <div class="columns">
                <div class="column is-two-thirds is-flex is-flex-column">
                    <h2 class="title is-size-4">{ "Message history" }</h2>
                    <pre style="flex-grow: 1">
                        {
                            // TODO: Add keys
                            self.ws_history.join("\n")
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
                                disabled=!connected
                                onclick=self.link.callback(|_| Command::CloseWebSocket)>
                            { "Close WebSocket connection" }
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
