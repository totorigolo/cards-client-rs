use derive_more::From;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::agents::game_mgr::*;
use crate::agents::game_ws_mgr::{GameWsConnectionInfo, WebSocketStatus};
use crate::agents::notifications::*;

pub struct PlayGame {
    link: ComponentLink<Self>,
    notification_bus: Dispatcher<NotificationBus>,

    ws_status: WebSocketStatus,

    game_mgr_agent: Box<dyn Bridge<GameMgr>>,

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
    GameMgrMessage(GameMgrResponse),
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
        let game_mgr_msg_callback = link.callback(Event::GameMgrMessage);
        PlayGame {
            link,
            notification_bus: NotificationBus::dispatcher(),

            ws_status: WebSocketStatus::NotConnected,

            game_mgr_agent: GameMgr::bridge(game_mgr_msg_callback),

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
                    self.game_mgr_agent.send(GameMgrRequest::EnsureConnected(
                        GameWsConnectionInfo {
                            game_id: self.props.game_id.clone(),
                            player_id: self.props.player_id.clone(),
                        },
                    ));
                    false
                }
            },
            Msg::Event(event) => match event {
                Event::GameMgrMessage(game_mgr_msg) => match game_mgr_msg {
                    GameMgrResponse::WebSocketStatusChanged(status) => {
                        self.ws_status.neq_assign(status)
                    }
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
