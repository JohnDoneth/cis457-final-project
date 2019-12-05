use bimap::BiMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum BoardCell {
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
        winner: Option<Uuid>,
        board: Board,
    },
}

impl Default for GameState {
    fn default() -> Self {
        GameState::WaitingForPlayers { players: vec![] }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerAction {
    Join {
        player: Uuid,
    },
    PlaceToken {
        player: Uuid,
        position: (usize, usize),
    },
}

/// An input action would result in an invalid or inconsistent game state.
#[derive(Debug, Deserialize, Serialize)]
pub enum InvalidAction {
    CantJoinTwice,
    StillWaitingForPlayers,
    GameAlreadyInPlay,
    PositionOutOfBounds,
    AlreadyPlacedThere,
    NotYourTurn,
}

fn is_board_full(board: &Board) -> bool {
    let mut found_empty = false;

    for row in board {
        for col in row {
            if col.is_none() {
                found_empty = true;
            }
        }
    }

    !found_empty
}

fn check_rows(
    board: &Board,
    token: BoardCell,
    tokens: &BiMap<Uuid, BoardCell>,
) -> Option<Option<Option<Uuid>>> {
    for col in 0..2 {
        let mut all_match = true;

        for row in 0..2 {
            let ref cell = board[row][col];

            if cell.is_none() || cell.unwrap() == token {
                all_match = false;
                break;
            }
        }

        if all_match {
            return Some(Some(Some(*tokens.get_by_right(&token).unwrap())));
        }
    }

    None
}

fn check_columns(
    board: &Board,
    token: BoardCell,
    tokens: &BiMap<Uuid, BoardCell>,
) -> Option<Option<Option<Uuid>>> {
    for row in 0..2 {
        let mut all_match = true;

        for col in 0..2 {
            let ref cell = board[row][col];

            if cell.is_none() || cell.unwrap() == token {
                all_match = false;
                break;
            }
        }

        if all_match {
            return Some(Some(Some(*tokens.get_by_right(&token).unwrap())));
        }
    }

    None
}

fn check_match(cells: &[Option<BoardCell>; 3], cell: &BoardCell) -> bool {
    for i in cells {
        if *i != Some(*cell) {
            return false;
        }
    }

    true
}

fn check_diagonal(
    board: &Board,
    token: BoardCell,
    tokens: &BiMap<Uuid, BoardCell>,
) -> Option<Option<Option<Uuid>>> {
    let left_down = [board[0][0], board[1][1], board[1][1]];

    if check_match(&left_down, &token) {
        return Some(Some(Some(*tokens.get_by_right(&token).unwrap())));
    }

    let right_down = [board[2][0], board[1][1], board[0][2]];

    if check_match(&right_down, &token) {
        return Some(Some(Some(*tokens.get_by_right(&token).unwrap())));
    }

    None
}

// Some(None) = Tie
// Some(UUID) = Winner is UUID
// None = No Winner or Tie yet
fn check_win_condition(board: Board, tokens: &BiMap<Uuid, BoardCell>) -> Option<Option<Uuid>> {
    // check row victory
    if let Some(res) = check_rows(&board, BoardCell::X, tokens) {
        return res;
    }

    if let Some(res) = check_rows(&board, BoardCell::Circle, tokens) {
        return res;
    }

    if let Some(res) = check_columns(&board, BoardCell::X, tokens) {
        return res;
    }

    if let Some(res) = check_columns(&board, BoardCell::Circle, tokens) {
        return res;
    }

    if let Some(res) = check_diagonal(&board, BoardCell::X, tokens) {
        return res;
    }

    if let Some(res) = check_diagonal(&board, BoardCell::Circle, tokens) {
        return res;
    }

    if is_board_full(&board) {
        return Some(None); // tie
    }

    None
}

pub fn process_input(input: PlayerAction, state: GameState) -> Result<GameState, InvalidAction> {
    match state {
        GameState::WaitingForPlayers { ref players } => {
            println!("can't move yet, waiting for player 2");
            match input {
                PlayerAction::Join { player } => {
                    let mut players = players.clone();

                    if players.contains(&player) {
                        return Err(InvalidAction::CantJoinTwice);
                    }

                    players.push(player);

                    if players.len() == 2 {
                        let p1 = players[0];
                        let p2 = players[1];

                        let mut tokens = BiMap::new();

                        let active_player: Uuid;
                        let waiting: Uuid;

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

                        Ok(GameState::WaitingForInput {
                            active_player,
                            waiting,
                            tokens,

                            board: [[None, None, None], [None, None, None], [None, None, None]],
                        })
                    } else {
                        Ok(GameState::WaitingForPlayers { players: players })
                    }
                }
                _ => Err(InvalidAction::StillWaitingForPlayers),
            }
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
                        return Err(InvalidAction::NotYourTurn);
                    }

                    if position.0 > 3 {
                        return Err(InvalidAction::PositionOutOfBounds);
                    }
                    if position.1 > 3 {
                        return Err(InvalidAction::PositionOutOfBounds);
                    }

                    if board[position.0][position.1] != None {
                        return Err(InvalidAction::AlreadyPlacedThere);
                    }

                    let player_token = &tokens.get_by_left(&player).unwrap();

                    board[position.0][position.1] = Some(*player_token.clone());

                    // check for win condition
                    if let Some(winner) = check_win_condition(board, tokens) {
                        // Someone has won
                        Ok(GameState::GameOver { winner, board })
                    } else {
                        // Game is not over yet.
                        Ok(GameState::WaitingForInput {
                            waiting: player,        // swap
                            active_player: waiting, // swap
                            tokens: tokens.clone(),
                            board: board,
                        })
                    }
                }
                _ => Err(InvalidAction::GameAlreadyInPlay),
            }
        }
        GameState::GameOver { .. } => Ok(state),
    }
}

/*#[test]
fn test_gameplay() {
    let p1 = Uuid::new_v4();
    let p2 = Uuid::new_v4();

    let mut s = GameState::WaitingForPlayers { players: vec![] };

    s = process_input(PlayerAction::Join { player: p1 }, s).unwrap();
    s = process_input(PlayerAction::Join { player: p2 }, s).unwrap();

    println!("{:?}", s);

    s = process_input(
        PlayerAction::PlaceToken {
            player: p1,
            position: (0, 0),
        },
        s,
    )
    .unwrap();
    s = process_input(
        PlayerAction::PlaceToken {
            player: p2,
            position: (1, 0),
        },
        s,
    )
    .unwrap();

    s = process_input(
        PlayerAction::PlaceToken {
            player: p1,
            position: (0, 1),
        },
        s,
    )
    .unwrap();
    s = process_input(
        PlayerAction::PlaceToken {
            player: p2,
            position: (1, 1),
        },
        s,
    )
    .unwrap();

    s = process_input(
        PlayerAction::PlaceToken {
            player: p1,
            position: (0, 2),
        },
        s,
    )
    .unwrap();
    s = process_input(
        PlayerAction::PlaceToken {
            player: p2,
            position: (1, 2),
        },
        s,
    )
    .unwrap();

    println!("{:?}", s);
}*/

fn assert_winning_move(board: Board, new_pos: (usize, usize)) {
    let p1 = Uuid::new_v4();
    let p2 = Uuid::new_v4();

    let mut tokens = BiMap::new();
    tokens.insert(p1, BoardCell::X);
    tokens.insert(p2, BoardCell::Circle);

    let s = GameState::WaitingForInput {
        active_player: p1,
        waiting: p2,
        tokens,
        board,
    };

    let action = PlayerAction::PlaceToken {
        player: p1,
        position: new_pos,
    };

    match process_input(action, s) {
        Ok(GameState::GameOver { .. }) => { /* ok */ }
        _ => assert!(false, "game should be over"),
    }
}

#[test]
fn test_win_condition_row2() {
    assert_winning_move(
        [
            [Some(BoardCell::X), Some(BoardCell::X), None],
            [None, None, None],
            [None, None, None],
        ],
        (0, 2),
    )
}

#[test]
fn test_win_condition_column() {
    assert_winning_move(
        [
            [Some(BoardCell::X), None, None],
            [Some(BoardCell::X), None, None],
            [None, None, None],
        ],
        (2, 0),
    )
}

#[test]
fn test_win_condition_diagonal1() {
    assert_winning_move(
        [
            [Some(BoardCell::X), None, None],
            [None, Some(BoardCell::X), None],
            [None, None, None],
        ],
        (2, 2),
    )
}

#[test]
fn test_win_condition_diagonal2() {
    assert_winning_move(
        [
            [None, None, Some(BoardCell::X)],
            [None, Some(BoardCell::X), None],
            [None, None, None],
        ],
        (2, 0),
    )
}

#[test]
fn test_win_condition_row() {
    let p1 = Uuid::new_v4();
    let p2 = Uuid::new_v4();

    let mut tokens = BiMap::new();
    tokens.insert(p1, BoardCell::X);
    tokens.insert(p2, BoardCell::Circle);

    let mut s = GameState::WaitingForInput {
        active_player: p1,
        waiting: p2,
        tokens,
        board: [
            [Some(BoardCell::X), Some(BoardCell::X), None],
            [None, None, None],
            [None, None, None],
        ],
    };

    let action = PlayerAction::PlaceToken {
        player: p1,
        position: (0, 2),
    };

    match process_input(action, s) {
        Ok(GameState::GameOver { .. }) => { /* ok */ }
        _ => assert!(false),
    }
}
