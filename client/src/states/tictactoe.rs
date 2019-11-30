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

#[derive(PartialEq, Copy, Clone)]
enum BoardCell {
    Circle,
    Square,
}

pub struct TicTacToe {
    board: [[Option<BoardCell>; 3]; 3],
    player_token: BoardCell,
    selection: (i16, i16),
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            player_token: BoardCell::Circle,
            selection: (0, 0),
        }
    }
}

impl State for TicTacToe {
    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(1)
                    .split(f.size());

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

                        // draw tokens
                        // #TODO DRAW PLAYER TOKENS

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
                    .render(&mut f, chunks[0]);
            })
            .unwrap();
    }

    fn on_event(&mut self, event: Event) -> Action {
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

                    self.board[x][y] = Some(self.player_token);
                }
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}
