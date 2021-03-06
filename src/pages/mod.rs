pub mod create_game;
pub mod index;
pub mod joining_game;
pub mod list_games;
pub mod not_found;
pub mod play_game;

pub use create_game::{CreateGame, Msg as CreateGameMsg};
pub use index::{Index, Msg as IndexMsg};
pub use joining_game::{JoiningGame, Msg as JoiningGameMsg};
pub use list_games::{ListGames, Msg as ListGamesMsg};
pub use not_found::{Msg as NotFoundMsg, NotFound};
pub use play_game::{Msg as PlayGameMsg, PlayGame};
