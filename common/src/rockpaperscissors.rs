use bimap::BiMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type PlayerID = Uuid;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum PlayerAction {
    Join { player: PlayerID },
    Move { player: PlayerID, action: Move },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryEntry {
    pub moves: BiMap<PlayerID, Move>,
    pub winner: Option<PlayerID>,
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
        input: Option<(PlayerID, Move)>,

        /// Which player has which token
        history: Vec<HistoryEntry>,
    },
    GameOver {
        winner: PlayerID,
        history: Vec<HistoryEntry>,
    },
}

impl Default for GameState {
    fn default() -> GameState {
        GameState::WaitingForPlayers { players: vec![] }
    }
}

fn rock_paper_scissors(
    p1: &PlayerID,
    p1move: &Move,
    p2: &PlayerID,
    p2move: &Move,
) -> Option<PlayerID> {
    match p1move {
        Move::Rock => match p2move {
            Move::Rock => None,
            Move::Paper => Some(*p2),
            Move::Scissors => Some(*p1),
        },
        Move::Paper => match p2move {
            Move::Rock => Some(*p1),
            Move::Paper => None,
            Move::Scissors => Some(*p2),
        },
        Move::Scissors => match p2move {
            Move::Rock => Some(*p2),
            Move::Paper => Some(*p1),
            Move::Scissors => None,
        },
    }
}

impl GameState {
    fn score(&self, player: PlayerID, history: &Vec<HistoryEntry>) -> usize {
        let mut wins = 0;

        match self {
            GameState::WaitingForPlayers { .. } => 0,
            GameState::WaitingForInput { .. } => {
                for event in history {
                    if event.winner == Some(player) {
                        wins += 1;
                    }
                }

                wins
            }
            GameState::GameOver { .. } => {
                for event in history {
                    if event.winner == Some(player) {
                        wins += 1;
                    }
                }

                wins
            }
        }
    }

    pub fn apply(&self, action: PlayerAction) -> Result<GameState, String> {
        match self {
            GameState::WaitingForPlayers { players } => match action {
                PlayerAction::Join { player: new_player } => {
                    if players.len() == 0 {
                        Ok(GameState::WaitingForPlayers {
                            players: vec![new_player],
                        })
                    } else {
                        let mut players = players.clone();
                        players.push(new_player);

                        Ok(GameState::WaitingForInput {
                            players,
                            round: 0,
                            input: None,
                            history: vec![],
                        })
                    }
                }
                _ => Err(format!(
                    "invalid action {:?} for given state {:?}",
                    self, action
                )),
            },
            GameState::WaitingForInput {
                players,
                round,
                input,
                history,
            } => match action {
                PlayerAction::Move { player, action } => {
                    if !players.contains(&player) {
                        return Err(format!("Invalid player"));
                    }

                    match input {
                        None => Ok(GameState::WaitingForInput {
                            players: players.clone(),
                            round: *round,
                            input: Some((player, action)),
                            history: history.clone(),
                        }),
                        Some((p1, p1move)) => {
                            let p2 = player;
                            let p2move = action;

                            if *p1 == p2 {
                                return Ok(self.clone()); // #TODO Error handling here
                            }

                            let round_winner = rock_paper_scissors(p1, p1move, &p2, &p2move);

                            let mut moves: BiMap<Uuid, Move> = BiMap::new();
                            moves.insert(*p1, *p1move);
                            moves.insert(p2, p2move);

                            let mut history = history.clone();
                            history.push(HistoryEntry {
                                winner: round_winner,
                                moves,
                            });

                            if self.score(*p1, &history) == 2 || self.score(p2, &history) == 2 {
                                let winner = if self.score(*p1, &history) == 2 {
                                    *p1
                                } else {
                                    p2
                                };

                                Ok(GameState::GameOver { winner, history })
                            } else {
                                Ok(GameState::WaitingForInput {
                                    players: players.clone(),
                                    round: round + 1,
                                    input: None,
                                    history,
                                })
                            }
                        }
                    }
                }
                _ => Err(format!(
                    "invalid action {:?} for given state {:?}",
                    self, action
                )),
            },
            GameState::GameOver { .. } => Ok(self.clone()),
        }
    }
}

#[test]
fn test_history() {
    let p1 = Uuid::new_v4();
    let p2 = Uuid::new_v4();
    let mut state = GameState::default();

    state = state.apply(PlayerAction::Join { player: p1 }).unwrap();
    state = state.apply(PlayerAction::Join { player: p2 }).unwrap();

    state = state
        .apply(PlayerAction::Move {
            player: p1,
            action: Move::Paper,
        })
        .unwrap();

    state = state
        .apply(PlayerAction::Move {
            player: p2,
            action: Move::Scissors,
        })
        .unwrap();

    //println!("{:#?}", state);

    state = state
        .apply(PlayerAction::Move {
            player: p1,
            action: Move::Paper,
        })
        .unwrap();

    //println!("{:#?}", state);

    state = state
        .apply(PlayerAction::Move {
            player: p2,
            action: Move::Scissors,
        })
        .unwrap();

    println!("{:#?}", state);

    match state {
        GameState::GameOver { winner, history } => {
            assert_eq!(winner, p2);
        }
        _ => assert!(false, "game should be over"),
    }
}
