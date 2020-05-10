use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::json;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, Request, Response};

// Re-exported for convenience.
pub use yew::services::fetch::FetchTask;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoundResponse {
    id: String,
    player_id: String,
    game_id: String,
    status: String,
    created_on: String,
    created_by: String,
    min_players: u32,
    max_players: u32,
    public: bool,
    players: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinRoundResponse {
    id: String,
    player_id: String,
    game_id: String,
    status: String,
    created_on: String,
    created_by: String,
    min_players: u32,
    max_players: u32,
    public: bool,
    players: Vec<String>,
}

#[derive(Default)]
pub struct GameServerService {
    web: FetchService,
}

impl GameServerService {
    pub fn new() -> Self {
        GameServerService {
            web: FetchService::new(),
        }
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
                callback.emit(Err(anyhow!("error joining the round: {}", meta.status)))
            }
        };
        self.web.fetch(request, handler.into()).unwrap()
    }
}
