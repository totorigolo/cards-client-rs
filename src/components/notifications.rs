use crate::agents::{Notification, NotificationBus, NotificationLevel, NotificationRequest};
use log::*;
use yew::prelude::*;

pub struct Notifications {
    link: ComponentLink<Self>,
    notifications: Vec<Notification>,

    // Holds a reference to the bus, in order for it to not be dropped
    _notification_bus: Box<dyn Bridge<NotificationBus>>,
}

#[derive(Debug)]
pub enum Msg {
    NewNotificationRequest(NotificationRequest),
    DeleteNotification(usize),
}

impl Component for Notifications {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::NewNotificationRequest);
        Self {
            //props,
            link,
            _notification_bus: NotificationBus::bridge(callback),
            notifications: Vec::with_capacity(10),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewNotificationRequest(request) => match request {
                NotificationRequest::New(notification) => {
                    self.notifications.push(notification);
                    true
                }
            },
            Msg::DeleteNotification(id) => {
                if id < self.notifications.len() {
                    self.notifications.remove(id);
                    true
                } else {
                    error!("Msg::DeleteNotification with invalid id.");
                    false
                }
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="notifications">
                { for self
                    .notifications
                    .iter()
                    .enumerate()
                    .map(|(id, notif)| render_notification(id, notif, &self.link))
                }
            </div>
        }
    }
}

fn render_notification(
    id: usize,
    notification: &Notification,
    link: &ComponentLink<Notifications>,
) -> Html {
    let color_class = match notification.level {
        NotificationLevel::Success => "is-success",
        NotificationLevel::Info => "is-info",
        NotificationLevel::Warning => "is-warning",
        NotificationLevel::Error => "is-danger",
    };

    let on_delete = link.callback(move |_: MouseEvent| Msg::DeleteNotification(id));

    html! {
        <div class=("notification", color_class)>
            <button class="delete" onclick=&on_delete></button>
            { for notification.text.lines().map(|l| html! { <p>{ l }</p> }) }
        </div>
    }
}
