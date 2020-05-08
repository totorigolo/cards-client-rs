use log::*;
use serde::{Deserialize, Serialize};
use yew::worker::*;

// Re-export this for convenience
pub use yew::agent::{Dispatcher, Dispatched};

pub struct NotificationBus {
    link: AgentLink<Self>,
    subscribers: Vec<HandlerId>,
}

pub trait NotificationSender {
    fn notification_bus(&mut self) -> &mut Dispatcher<NotificationBus>;

    fn notify_success<T: ToString>(&mut self, text: T) {
        let s = text.to_string();
        debug!("Success notification: {:?}", &s);
        self.notification_bus()
            .send(NotificationRequest::New(Notification {
                text: s,
                level: NotificationLevel::Success,
            }));
    }

    fn notify_info<T: ToString>(&mut self, text: T) {
        let s = text.to_string();
        info!("Info notification: {:?}", &s);
        self.notification_bus()
            .send(NotificationRequest::New(Notification {
                text: s,
                level: NotificationLevel::Info,
            }));
    }

    fn notify_warn<T: ToString>(&mut self, text: T) {
        let s = text.to_string();
        warn!("Warning notification: {:?}", &s);
        self.notification_bus()
            .send(NotificationRequest::New(Notification {
                text: s,
                level: NotificationLevel::Warning,
            }));
    }

    fn notify_error<T: ToString>(&mut self, text: T) {
        let s = text.to_string();
        error!("Error notification: {:?}", &s);
        self.notification_bus()
            .send(NotificationRequest::New(Notification {
                text: s,
                level: NotificationLevel::Error,
            }));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NotificationRequest {
    New(Notification),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Notification {
    pub text: String,
    pub level: NotificationLevel,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum NotificationLevel {
    Success,
    Info,
    Warning,
    Error,
}

impl Agent for NotificationBus {
    type Reach = Context;
    type Message = ();
    type Input = NotificationRequest;
    type Output = NotificationRequest;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: Vec::with_capacity(10),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, sender: HandlerId) {
        trace!("Notification received from '{:?}': {:?}", sender, msg);
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        trace!("Notification listener connected: {:?}", id);
        if !self.subscribers.contains(&id) {
            self.subscribers.push(id);
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        trace!("Notification listener disconnected: {:?}", id);
        if let Some(pos) = self.subscribers.iter().position(|x| *x == id) {
            self.subscribers.swap_remove(pos);
        }
    }
}
