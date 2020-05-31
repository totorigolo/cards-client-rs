use derive_more::From;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::agents::game_ws_mgr::*;
use crate::agents::notifications::*;

pub struct PlayGame {
    link: ComponentLink<Self>,
    notification_bus: Dispatcher<NotificationBus>,

    ws_agent: Box<dyn Bridge<GameWsMgr>>,
    ws_status: WebSocketStatus,

    props: Props,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub game_id: String,
    pub player_id: String,
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

#[derive(Debug)]
pub enum Event {
    WebSocketMessage(GameWsResponse),
}

impl NotificationSender for PlayGame {
    fn notification_bus(&mut self) -> &mut Dispatcher<NotificationBus> {
        &mut self.notification_bus
    }
}

impl Component for PlayGame {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Command::Update);
        let ws_msg_callback = link.callback(Event::WebSocketMessage);
        PlayGame {
            link,
            notification_bus: NotificationBus::dispatcher(),

            ws_agent: GameWsMgr::bridge(ws_msg_callback),
            ws_status: WebSocketStatus::NotConnected,

            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props.neq_assign(props);
        if changed {
            self.link.send_message(Command::Update);
        }
        changed
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Command(command) => match command {
                Command::Update => {
                    // Ensure we are connected.
                    self.ws_agent.send(GameWsRequest::JoinRound {
                        game_id: self.props.game_id.clone(),
                        player_id: self.props.player_id.clone(),
                    });
                    false
                }
            },
            Msg::Event(event) => match event {
                Event::WebSocketMessage(ws_msg) => match ws_msg {
                    GameWsResponse::Connecting(info) => {
                        self.ws_status = WebSocketStatus::Pending(info);
                        true
                    }
                    GameWsResponse::Connected(info) => {
                        self.ws_status = WebSocketStatus::Connected(info);
                        true
                    }
                    GameWsResponse::Closed => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        true
                    }
                    GameWsResponse::FailedToConnect(reason) => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        self.notify_error(format!("Failed to join game: {}", reason));
                        true
                    }
                    GameWsResponse::ErrorOccurred => {
                        self.ws_status = WebSocketStatus::NotConnected;
                        // TODO: Who should be responsible for this notification?
                        self.notify_error("Unknown error in WebSocket connection.");
                        true
                    }
                    GameWsResponse::Received(_data) => true,
                    GameWsResponse::ReceivedError(_error) => true,
                    GameWsResponse::WebSocketStatus(status) => self.ws_status.neq_assign(status),
                },
            },
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { "Play game" }
            </>
        }
    }
}
