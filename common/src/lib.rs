use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lobby {
    pub name: String,
    pub players: usize,
    pub max_players: usize,
    pub game: String,
}