use log::*;
use serde::{Deserialize, Serialize};
use yew::worker::*;

// Re-export this for convenience
pub use yew::agent::{Dispatched, Dispatcher};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct State {
    pub ws_server_addr: String,
}

pub struct StateMgr {
    link: AgentLink<Self>,
    subscribers: Vec<HandlerId>,
    #[allow(dead_code)]
    state: State,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StateRequest {
    Get,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StateResponse {
    Nope,
}

impl Agent for StateMgr {
    type Reach = Context;
    type Message = ();
    type Input = StateRequest;
    type Output = StateResponse;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: Vec::with_capacity(10),
            state: State::default(),
        }
    }

    fn update(&mut self, _: ()) {}

    fn handle_input(&mut self, msg: Self::Input, sender: HandlerId) {
        trace!("Notification received from '{:?}': {:?}", sender, msg);
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, StateResponse::Nope);
        }
    }

    fn connected(&mut self, id: HandlerId) {
        trace!("New connection to state manager: {:?}", id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        trace!("Notification listener disconnected: {:?}", id);
        if let Some(pos) = self.subscribers.iter().position(|x| *x == id) {
            self.subscribers.swap_remove(pos);
        }
    }
}
