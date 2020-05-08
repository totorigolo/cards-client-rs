use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct State {
    pub ws_server_addr: String,
}

impl State {}
