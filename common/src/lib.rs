use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod tictactoe;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Game {
    TicTacToe(tictactoe::GameState),
    RockPaperScissors,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lobby {
    pub name: String,
    pub players: usize,
    pub max_players: usize,
    pub game: Game,
}

#[derive(Serialize, Deserialize)]
pub struct JoinResponse {
    pub player: Uuid,
}
