use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, From, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "PING")]
    Ping,

    #[serde(rename = "PONG")]
    Pong,

    #[serde(rename = "CLOSE")]
    Close,

    #[serde(rename = "PLAYER_CONNECTED")]
    #[from]
    PlayerConnected(PlayerConnected),

    #[serde(rename = "START_GAME")]
    StartGame,

    #[serde(rename = "GAME_STARTED")]
    GameStarted,

    #[serde(rename = "GAME_FINISHED")]
    #[from]
    GameFinished(GameFinished),

    #[serde(rename = "ERROR")]
    #[from]
    Error(Error),

    #[serde(rename = "ACTION_AWAITED")]
    #[from]
    ActionAwaited(ActionAwaited),

    #[serde(rename = "INTERFACE_UPDATE")]
    #[from]
    InterfaceUpdate(InterfaceUpdate),

    #[serde(rename = "COMPONENTS_UPDATES")]
    #[from]
    ComponentsUpdates(ComponentsUpdates),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerConnected {
    message: String,
    username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameFinished {
    winners: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Error {
    messages: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ActionAwaited {
    all_of: Vec<AwaitedAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum AwaitedAction {
    #[serde(rename = "OnClick")]
    OnClick { target_component: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InterfaceUpdate {
    components: Vec<InterfaceComponent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InterfaceComponent {
    id: ComponentId,
    position: ComponentPosition,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ComponentId(String);

impl<S: ToString> From<S> for ComponentId {
    fn from(id: S) -> Self {
        ComponentId(id.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComponentPosition {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ComponentsUpdates {
    #[serde(rename = "components")]
    updates: Vec<ComponentUpdate>,
}

#[derive(Serialize, Deserialize, From, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ComponentUpdate {
    Create {
        id: ComponentId,
        component: Component,
    },
}

#[derive(Serialize, Deserialize, From, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Component {
    #[serde(rename = "Card")]
    #[from]
    Card(Card),

    #[serde(rename = "Hand")]
    #[from]
    Hand(Hand),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Card {
    name: String,
    description: String,
    front_image: Option<String>,
    back_image: Option<String>,
    state: CardState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CardState {
    suit: String,
    value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Hand {
    cards: Vec<ComponentId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_encoding_decoding(msg: impl Into<Message>, as_str: impl AsRef<str>) {
        let msg = msg.into();
        let encoded = serde_json::to_string(&msg).expect("encoding failed");
        assert_eq!(
            encoded,
            as_str.as_ref(),
            "encoding differs from expectation"
        );

        let decoded: Message = serde_json::from_str(as_str.as_ref()).expect("decoding failed");
        assert_eq!(decoded, msg, "decoding differs from expectation");
    }

    #[test]
    fn ping() {
        test_encoding_decoding(Message::Ping, r#"{"type":"PING"}"#);
    }

    #[test]
    fn pong() {
        test_encoding_decoding(Message::Pong, r#"{"type":"PONG"}"#);
    }

    #[test]
    fn close() {
        test_encoding_decoding(Message::Close, r#"{"type":"CLOSE"}"#);
    }

    #[test]
    fn player_connected() {
        test_encoding_decoding(
            PlayerConnected {
                username: "Toto".to_string(),
                message: "Say hello to Toto.".to_string(),
            },
            r#"{"type":"PLAYER_CONNECTED","message":"Say hello to Toto.","username":"Toto"}"#,
        );
    }

    #[test]
    fn start_game() {
        test_encoding_decoding(Message::StartGame, r#"{"type":"START_GAME"}"#);
    }

    #[test]
    fn game_finished() {
        test_encoding_decoding(
            GameFinished {
                winners: vec!["Toto".to_string(), "Tata".to_string()],
            },
            r#"{"type":"GAME_FINISHED","winners":["Toto","Tata"]}"#,
        );
    }

    #[test]
    fn error() {
        test_encoding_decoding(
            Error {
                messages: vec![
                    "You are dumb.".to_string(),
                    "The cake is a lie.".to_string(),
                ],
            },
            r#"{"type":"ERROR","messages":["You are dumb.","The cake is a lie."]}"#,
        );
    }

    #[test]
    fn action_awaited() {
        test_encoding_decoding(
            ActionAwaited {
                all_of: vec![AwaitedAction::OnClick {
                    target_component: "hand".to_string(),
                }],
            },
            r#"{
                "type": "ACTION_AWAITED",
                "all_of": [
                    {
                        "type": "OnClick",
                        "target_component": "hand"
                    }
                ]
            }"#
            .replace(|c: char| c.is_whitespace(), ""),
        );
    }

    #[test]
    fn interface_update() {
        test_encoding_decoding(
            InterfaceUpdate {
                components: vec![
                    InterfaceComponent {
                        id: "played_cards".into(),
                        position: ComponentPosition::Bottom,
                    },
                    InterfaceComponent {
                        id: "hand".into(),
                        position: ComponentPosition::Center,
                    },
                ],
            },
            r#"{
                "type": "INTERFACE_UPDATE",
                "components": [
                    {
                        "id": "played_cards",
                        "position": "bottom"
                    },
                    {
                        "id": "hand",
                        "position": "center"
                    }
                ]
            }"#
            .replace(|c: char| c.is_whitespace(), ""),
        );
    }

    #[test]
    fn components_updates() {
        test_encoding_decoding(
            ComponentsUpdates {
                updates: vec![
                    ComponentUpdate::Create {
                        id: "hand".into(),
                        component: Hand {
                            cards: vec![
                                "773b57de804b4067a27b9650d077d470".into(),
                                "3a618ae83d664b43b1096738f559978a".into(),
                                "bd5b40c1a4c342539dbc3165982ccf31".into(),
                            ],
                        }
                        .into(),
                    },
                    ComponentUpdate::Create {
                        id: "bd5b40c1a4c342539dbc3165982ccf31".into(),
                        component: Card {
                            name: "H2".to_string(),
                            description: "".to_string(),
                            front_image: None,
                            back_image: None,
                            state: CardState {
                                suit: "H".to_string(),
                                value: "2".to_string(),
                            },
                        }
                        .into(),
                    },
                ],
            },
            r#"{
                "type": "COMPONENTS_UPDATES",
                "components": [
                    {
                        "type":"Create",
                        "id":"hand",
                        "component": {
                            "type":"Hand",
                            "cards": [
                                "773b57de804b4067a27b9650d077d470",
                                "3a618ae83d664b43b1096738f559978a",
                                "bd5b40c1a4c342539dbc3165982ccf31"
                            ]
                        }
                    },
                    {
                        "type": "Create",
                        "id": "bd5b40c1a4c342539dbc3165982ccf31",
                        "component": {
                            "type": "Card",
                            "name": "H2",
                            "description": "",
                            "front_image": null,
                            "back_image": null,
                            "state": {
                                "suit": "H",
                                "value": "2"
                            }
                        }
                    }
                ]
            }"#
            .replace(|c: char| c.is_whitespace(), ""),
        );
    }
}
