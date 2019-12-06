use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod rockpaperscissors;
pub mod tictactoe;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    TicTacToe(tictactoe::PlayerAction),
    RockPaperScissors(rockpaperscissors::PlayerAction),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Game {
    TicTacToe(tictactoe::GameState),
    RockPaperScissors(rockpaperscissors::GameState),
}

impl From<GameType> for Game {
    fn from(_type: GameType) -> Game {
        match _type {
            GameType::TicTacToe => Game::TicTacToe(tictactoe::GameState::default()),
            GameType::RockPaperScissors => {
                Game::RockPaperScissors(rockpaperscissors::GameState::default())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameType {
    TicTacToe,
    RockPaperScissors,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lobby {
    pub name: String,
    pub players: usize,
    pub max_players: usize,
    pub game_type: GameType,
    pub game: Game,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinResponse {
    pub player: Uuid,
    pub game_type: GameType,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateLobbyRequest {
    pub name: String,
    pub game: GameType,
}
