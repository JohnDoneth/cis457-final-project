use serde::{Deserialize, Serialize};
use bimap::BiMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum BoardCell {
    Circle,
    X,
}

pub type Board = [[Option<BoardCell>; 3]; 3];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameState {
    WaitingForPlayers {
        players: Vec<Uuid>,
    },
    WaitingForInput {
        // Which player's turn it is.
        active_player: Uuid,

        // The waiting player.
        waiting: Uuid,

        // Which player has which token
        tokens: BiMap<Uuid, BoardCell>,

        // Board
        board: Board,
    },
    GameOver {
        winner: Uuid,
        loser: Uuid,
        board: Board,
    },
}

#[derive(Debug)]
enum PlayerAction {
    Join {
        player: Uuid,
    },
    PlaceToken {
        player: Uuid,
        position: (usize, usize),
    },
}

fn checkWinCondition(board: Board, tokens: &BiMap<Uuid, BoardCell>) -> Option<Uuid> {
    // check row victory
    for row in board {
        if row[0] == Some(BoardCell::X)
            && row[1] == Some(BoardCell::X)
            && row[2] == Some(BoardCell::X)
        {
            return Some(tokens.get_by_right(BoardCell::X))
        }
    }

    None
}

pub fn process_input(input: PlayerAction, state: GameState) -> GameState {
    match state {
        GameState::WaitingForPlayers { ref players } => {
            println!("can't move yet, waiting for player 2");
            match input {
                PlayerAction::Join { player } => {
                    let mut players = players.clone();

                    players.push(player);

                    if players.len() == 2 {
                        let p1 = players[0];
                        let p2 = players[1];

                        let mut tokens = BiMap::new();

                        let mut active_player: Uuid;
                        let mut waiting: Uuid;

                        // randomly assign tokens
                        if rand::random() {
                            // player 1 is X

                            tokens.insert(p1, BoardCell::X);
                            tokens.insert(p2, BoardCell::Circle);

                            active_player = p1;
                            waiting = p2;
                        } else {
                            // player 2 is X

                            tokens.insert(p1, BoardCell::Circle);
                            tokens.insert(p2, BoardCell::X);

                            active_player = p2;
                            waiting = p1;
                        }

                        return GameState::WaitingForInput {
                            active_player,
                            waiting,
                            tokens,

                            board: [[None, None, None], [None, None, None], [None, None, None]],
                        };
                    } else {
                        return GameState::WaitingForPlayers { players: players };
                    }
                }
                _ => println!("invalid action"),
            }

            state
        }
        GameState::WaitingForInput {
            active_player,
            waiting,
            ref tokens,
            mut board,
        } => {
            match input {
                PlayerAction::PlaceToken { player, position } => {
                    if active_player != player {
                        println!("invalid action: not your turn");
                        return state;
                    }

                    if position.0 > 3 || position.0 < 0 {
                        println!("invalid action: invalid pos");
                        return state;
                    }
                    if position.1 > 3 || position.1 < 0 {
                        println!("invalid action: invalid pos");
                        return state;
                    }

                    let player_token = &tokens.get_by_left(player);

                    board[position.0][position.1] = Some(player_token.clone());

                    // check for win condition

                    if let Some(winner) = checkWinCondition(board, tokens) {
                        println!("winner");
                        return state;
                    }

                    GameState::WaitingForInput {
                        waiting: player,        // swap
                        active_player: waiting, // swap
                        tokens: tokens.clone(),
                        board: board,
                    }
                }
                _ => {
                    println!("invalid action, game in play");
                    state
                }
            }
        }
        GameState::GameOver { .. } => state,
    }
}

#[test]
fn test_gameplay() {
    use super::*;

    let p1 = Uuid::new_v4();
    let p2 = Uuid::new_v4();

    let mut s = GameState::WaitingForPlayers { players: vec![] };

    s = process_input(PlayerAction::Join { player: p1 }, s);
    s = process_input(PlayerAction::Join { player: p2 }, s);

    println!("{:?}", s);

    s = process_input(
        PlayerAction::PlaceToken {
            player: p1,
            position: (0, 0),
        },
        s,
    );
    s = process_input(
        PlayerAction::PlaceToken {
            player: p2,
            position: (1, 0),
        },
        s,
    );

    s = process_input(
        PlayerAction::PlaceToken {
            player: p1,
            position: (0, 1),
        },
        s,
    );
    s = process_input(
        PlayerAction::PlaceToken {
            player: p2,
            position: (1, 1),
        },
        s,
    );

    s = process_input(
        PlayerAction::PlaceToken {
            player: p1,
            position: (0, 2),
        },
        s,
    );
    s = process_input(
        PlayerAction::PlaceToken {
            player: p2,
            position: (1, 2),
        },
        s,
    );

    println!("{:?}", s);

    // s = process_input(PlayerAction::JoinLobby(p1));
}
