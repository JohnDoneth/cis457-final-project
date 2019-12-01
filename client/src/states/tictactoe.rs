use crate::state::Backend;
use crate::state::{Action, State};

use tui::terminal::Terminal;

use crate::state::StateManager;
use crate::util::event::{Event, Events};

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::Line;
use tui::widgets::canvas::Map;
use tui::widgets::canvas::MapResolution;
use tui::widgets::canvas::Rectangle;
use tui::widgets::{canvas::Canvas, Block, Borders, List, Paragraph, Row, Table, Text, Widget};

use unicode_width::UnicodeWidthStr;

use std::io::{self, Write};

use termion::cursor::Goto;

use uuid::Uuid;

use common::tictactoe::Board;
use common::tictactoe::BoardCell;
use common::tictactoe::GameState;
use common::Game;

pub struct TicTacToe {
    board: Board,
    player_token: BoardCell,
    selection: (i16, i16),
    player: Uuid,
    lobby: String,
    state: GameState,
    status: String,
}

impl TicTacToe {
    pub fn new(player: Uuid, lobby: &str) -> Self {
        Self {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player_token: BoardCell::X,
            selection: (0, 0),
            player,
            lobby: lobby.to_owned(),
            state: GameState::default(),
            status: String::from("waiting"),
        }
    }

    pub async fn fetch_state(&mut self) {
        // fetch the game state
        let url = format!("http://localhost:8000/lobbies/{}/state", self.lobby);

        let game: Game = surf::get(url).await.unwrap().body_json().await.unwrap();

        match game {
            Game::TicTacToe(s) => {
                self.state = s;
            }
            _ => panic!("wrong game type"),
        }

        self.update()
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::WaitingForPlayers { .. } => {
                self.status = format!("Waiting for another player");
            }
            GameState::WaitingForInput {
                active_player,
                board,
                ref tokens,
                ..
            } => {
                self.board = board;

                self.player_token = *tokens.clone().get_by_left(&self.player).unwrap();

                if self.player == active_player {
                    self.status = format!("It's your turn");
                } else {
                    self.status = format!("Waiting for the other play to make their move.")
                }
            }
            GameState::GameOver { winner, board } => {
                self.board = board;

                match winner {
                    Some(winner) => {
                        if self.player == winner {
                            self.status = format!("The game is over, you've won!");
                        } else {
                            self.status = format!("The game is over, you've lost.");
                        }
                    },
                    None => {
                        self.status = format!("The game is over, it was a tie.");
                    }
                }

                
            }
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl State for TicTacToe {
    async fn on_enter(&mut self) {
        self.fetch_state().await;
    }

    async fn on_update(&mut self) {
        self.fetch_state().await;
    }

    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
                    .margin(1)
                    .split(f.size());

                Paragraph::new([Text::raw(self.status.clone())].iter()).render(&mut f, chunks[0]);

                Canvas::default()
                    .block(Block::default().title("Tic-Tac-Toe").borders(Borders::ALL))
                    .x_bounds([0.0, 77.0])
                    .y_bounds([0.0, 77.0])
                    .paint(|ctx| {
                        // draw board
                        for x in 0..3 {
                            for y in 0..3 {
                                ctx.draw(&Rectangle {
                                    rect: Rect {
                                        x: (x * 25) + 1,
                                        y: (y * 25) + 1,
                                        width: 25,
                                        height: 25,
                                    },
                                    color: Color::White,
                                });
                            }
                        }

                        ctx.layer();

                        // draw board tokens
                        let margin = 8;
                        let half_margin = margin / 2;

                        for x in 0..3u16 {
                            for y in 0..3u16 {
                                match self.board[x as usize][y as usize] {
                                    Some(BoardCell::Circle) => {
                                        let rect = Rect {
                                            x: (x * 25) + 1,
                                            y: (y * 25) + 1,
                                            width: 25,
                                            height: 25,
                                        };

                                        ctx.draw(&Rectangle {
                                            rect: Rect {
                                                x: rect.x + half_margin,
                                                y: rect.y + half_margin,
                                                width: 25 - margin,
                                                height: 25 - margin,
                                            },
                                            color: Color::White,
                                        });
                                    }
                                    Some(BoardCell::X) => {
                                        let rect = Rect {
                                            x: (x * 25) + 1,
                                            y: (y * 25) + 1,
                                            width: 25,
                                            height: 25,
                                        };

                                        let rect = Rect {
                                            x: rect.x + half_margin,
                                            y: rect.y + half_margin,
                                            width: 25 - margin,
                                            height: 25 - margin,
                                        };

                                        ctx.draw(&Line {
                                            x1: rect.x as f64,
                                            y1: rect.y as f64,
                                            x2: (rect.x + rect.width) as f64,
                                            y2: (rect.y + rect.height) as f64,
                                            color: Color::White,
                                        });

                                        ctx.draw(&Line {
                                            x1: rect.x as f64,
                                            y1: (rect.y + rect.height) as f64,
                                            x2: (rect.x + rect.width) as f64,
                                            y2: rect.y as f64,
                                            color: Color::White,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // draw selection

                        let (x, y) = self.selection;
                        let x = x as u16;
                        let y = y as u16;

                        let cell = self.board[x as usize][y as usize];

                        let color = {
                            if cell == None {
                                Color::Green
                            } else if cell.is_some() && cell.unwrap() == self.player_token {
                                Color::Blue
                            } else {
                                Color::Red
                            }
                        };

                        ctx.draw(&Rectangle {
                            rect: Rect {
                                x: (x * 25) + 1,
                                y: (y * 25) + 1,
                                width: 25,
                                height: 25,
                            },
                            color: color,
                        });
                    })
                    .render(&mut f, chunks[1]);
            })
            .unwrap();
    }

    async fn on_event(&mut self, event: Event) -> Action {
        match event {
            Event::Input(input) => match input {
                Key::Up => {
                    self.selection.1 = {
                        if self.selection.1 + 1 >= 3 {
                            0
                        } else {
                            self.selection.1 + 1
                        }
                    }
                }
                Key::Down => {
                    self.selection.1 = {
                        if self.selection.1 - 1 < 0 {
                            2
                        } else {
                            self.selection.1 - 1
                        }
                    }
                }
                Key::Right => {
                    self.selection.0 = {
                        if self.selection.0 + 1 >= 3 {
                            0
                        } else {
                            self.selection.0 + 1
                        }
                    }
                }
                Key::Left => {
                    self.selection.0 = {
                        if self.selection.0 - 1 < 0 {
                            2
                        } else {
                            self.selection.0 - 1
                        }
                    }
                }
                Key::Char('\n') => {
                    let (x, y) = self.selection;
                    let x = x as usize;
                    let y = y as usize;

                    use common::tictactoe::GameState;

                    //self.board[x][y] = Some(self.player_token);
                    let url = format!("http://localhost:8000/lobbies/{}/action", self.lobby);

                    let res: serde_json::Value = surf::post(url)
                        .body_json(&common::tictactoe::PlayerAction::PlaceToken {
                            player: self.player,
                            position: (x, y),
                        })
                        .unwrap()
                        .await
                        .unwrap()
                        .body_json()
                        .await
                        .unwrap();

                    if let Ok(new_state) = serde_json::from_value::<Game>(res) {
                        match new_state {
                            Game::TicTacToe(new_state) => {
                                self.state = new_state;
                                self.update();
                            }
                            _ => panic!("wrong game type"),
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}
