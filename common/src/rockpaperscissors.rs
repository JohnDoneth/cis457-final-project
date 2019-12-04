type PlayerID = Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerAction {
    Join { player: PlayerID },
    Move(Move),
}

struct HistoryEntry {
    moves: BiMap<PlayerID, Move>,
    winner: PlayerID,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameState {
    WaitingForPlayers {
        /// Players
        players: Vec<PlayerID>,
    },
    WaitingForInput {
        /// Players
        players: Vec<PlayerID>,

        /// What round we're on.
        round: usize,

        /// Either `None` or what the other play has moved.
        input: Option<Move>,

        /// Which player has which token
        history: Vec<HistoryEntry>,
    },
    GameOver {
        winner: PlayerID,
        history: Vec<HistoryEntry>,
    },
}
