use anyhow::{anyhow, Result};
use serde::Deserialize;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, Request, Response};

// Re-exported for convenience.
pub use yew::services::fetch::FetchTask;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoundResponse {
    pub id: String,
    pub player_id: String,
    pub game_id: String,
    pub status: String,
    pub created_on: String,
    pub created_by: String,
    pub min_players: u32,
    pub max_players: u32,
    pub public: bool,
    pub players: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinRoundResponse {
    pub id: String,
    pub player_id: String,
    pub game_id: String,
    pub status: String,
    pub created_on: String,
    pub created_by: String,
    pub min_players: u32,
    pub max_players: u32,
    pub public: bool,
    pub players: Vec<String>,
}

#[derive(Default)]
pub struct GameServerService {}

impl GameServerService {
    pub fn new() -> Self {
        GameServerService {}
    }

    // pub fn create_round(
    //     &mut self,
    //     game_name: impl AsRef<str>,
    //     username: impl AsRef<str>,
    //     callback: Callback<Result<CreateRoundResponse>>,
    // ) -> Result<FetchTask> {
    //     let url = format!("/api/round/create/{}", game_name.as_ref());
    //     let request_body = json!({"username": username.as_ref()});
    //     let request = Request::post(&url)
    //         .header("Content-Type", "application/json")
    //         .body(Json(&request_body))
    //         .context("Failed to build create_round request.")?;
    //
    //     let handler = move |response: Response<Json<Result<CreateRoundResponse>>>| {
    //         let (meta, Json(data)) = response.into_parts();
    //         if meta.status.is_success() {
    //             callback.emit(data)
    //         } else {
    //             callback.emit(Err(anyhow!("error creating the round: {}", meta.status)))
    //         }
    //     };
    //     self.web
    //         .fetch(request, handler.into())
    //         .context("Fetch failed in create_round")
    // }

    pub fn join_round(
        &mut self,
        game_id: impl AsRef<str>,
        username: impl AsRef<str>,
        callback: Callback<Result<JoinRoundResponse>>,
    ) -> FetchTask {
        let url = format!(
            "/api/round/{}/join?username={}",
            game_id.as_ref(),
            username.as_ref()
        );
        let request = Request::get(url.as_str()).body(Nothing).unwrap();

        let handler = move |response: Response<Json<Result<JoinRoundResponse>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!("{}", meta.status)))
            }
        };
        FetchService::fetch(request, handler.into()).unwrap()
    }
}
