use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::prelude::*;

const KEY: &str = "cards-client-rs.state";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    value: String,
}

#[derive(Debug)]
pub enum Msg {
    Update(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();

        let state = match storage.restore(KEY) {
            Json(Ok(restored_state)) => restored_state,
            _ => State {
                value: String::from(""),
            },
        };

        App { link, storage, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
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

impl State {
}
