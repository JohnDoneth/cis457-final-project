#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerAction {
    Join { player: Uuid },
    Move(Move),
}

struct HistoryEntry {
    moves: BiMap<Uuid, Move>,
    winner: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameState {
    WaitingForPlayers {
        players: Vec<Uuid>,
    },
    WaitingForInput {
        // What round we're on.
        round: usize,

        // Either `None` or what the other play has moved.
        input: Option<Move>,

        // Which player has which token
        history: Vec<HistoryEntry>,
    },
    GameOver {
        winner: Uuid,
        history: Vec<HistoryEntry>,
    },
}
