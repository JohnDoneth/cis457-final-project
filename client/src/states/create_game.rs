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
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, Table, Tabs, Text, Widget};

use common::Lobby;

const SELECTION_MAX: usize = 3;

pub struct CreateGame {
    lobby_name: String,
    game_type: usize,
    selected: usize,
}

impl CreateGame {
    pub fn new() -> Self {
        Self {
            lobby_name: String::new(),
            game_type: 0,
            selected: 0,
        }
    }
}

use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
impl State for CreateGame {
    async fn on_enter(&mut self) {}

    async fn on_update(&mut self) {}

    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let chunks = Layout::default()
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(3),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(3),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .margin(1)
                    .split(f.size());

                Paragraph::new([Text::raw(String::from("Please enter a lobby name."))].iter())
                    .style(Style::default().fg(Color::Blue))
                    .render(&mut f, chunks[0]);

                let selected_border_style = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan));

                let p1_border = if self.selected == 0 {
                    selected_border_style
                } else {
                    Block::default().borders(Borders::ALL)
                };

                let p2_border = if self.selected == 1 {
                    selected_border_style
                } else {
                    Block::default().borders(Borders::ALL)
                };

                Paragraph::new([Text::raw(self.lobby_name.clone())].iter())
                    .block(p1_border)
                    .render(&mut f, chunks[1]);

                Paragraph::new([Text::raw(String::from("Please select a game type."))].iter())
                    .style(Style::default().fg(Color::Blue))
                    .render(&mut f, chunks[3]);

                Tabs::default()
                    .block(p2_border)
                    .titles(&["Tic-Tac-Toe", "Rock Paper Scissors"])
                    .style(Style::default().fg(Color::White))
                    .select(self.game_type)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Green)
                            .modifier(Modifier::UNDERLINED),
                    )
                    .render(&mut f, chunks[4])
            })
            .unwrap();
    }

    async fn on_event(&mut self, event: Event) -> Action {
        match event {
            Event::Input(key) => match key {
                Key::Down => {
                    self.selected += 1;
                    if self.selected > SELECTION_MAX - 1 {
                        self.selected = 0;
                    }
                }
                Key::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    } else {
                        self.selected = SELECTION_MAX - 1;
                    }
                }
                Key::Right => {
                    if self.selected == 1 {
                        self.game_type += 1;
                        if self.game_type > 2 - 1 {
                            self.game_type = 0;
                        }
                    }
                }
                Key::Left => {
                    if self.selected == 1 {
                        if self.game_type > 0 {
                            self.game_type -= 1;
                        } else {
                            self.game_type = 2 - 1;
                        }
                    }
                }
                Key::Char('\n') => {
                    if self.selected == 1 {
                        // try and create it.
                        let url = format!("http://localhost:8000/lobbies");

                        let res = surf::post(url)
                            .body_json(&common::CreateLobbyRequest {
                                name: self.lobby_name.clone(),
                                game: common::GameType::TicTacToe,
                            })
                            .unwrap()
                            .await
                            .unwrap();

                        let url = format!("http://localhost:8000/lobbies/{}/join", self.lobby_name);

                        let res: Result<common::JoinResponse, _> =
                            surf::post(url).await.unwrap().body_json().await;

                        if let Ok(res) = res {
                            use crate::states::TicTacToe;

                            return Action::PushState(Box::new(TicTacToe::new(
                                res.player,
                                &self.lobby_name,
                            )));
                        }
                    }
                }
                Key::Char(c) => {
                    if self.selected == 0 {
                        self.lobby_name.push(c);
                    }
                }
                Key::Backspace => {
                    if self.selected == 0 {
                        self.lobby_name.pop();
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}
