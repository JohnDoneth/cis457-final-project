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
use tui::widgets::{Block, Borders, Row, Table, Widget};

use common::Lobby;

pub struct GameBrowser {
    items: Vec<Lobby>,
    selected: usize,
}

impl GameBrowser {
    pub fn new() -> Self {
        Self {
            items: vec![],
            selected: 0,
        }
    }
}

use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
impl State for GameBrowser {
    async fn on_update(&mut self) {}

    async fn on_enter(&mut self) {

        /*
        let lobbies: Result<HashMap<String, Lobby>, _> = surf::get("http://localhost:8000/lobbies")
            .await
            .unwrap()
            .body_json()
            .await;
        */

        // fetch games
        let lobbies: Result<HashMap<String, Lobby>, _> = surf::get("http://localhost:8000/lobbies")
            .await
            .unwrap()
            .body_json().await;

        println!("{:?}", lobbies);

        self.items.clear();

        //for (_, lobby) in lobbies {
            //self.items.push(lobby);
        //}

        /*
        vec![
                format!("{}", lobby.name),
                format!("{}", "127.0.0.1"),
                format!("{}", "TicTacToe"), // #TODO!
                format!("({}/{})", lobby.players, lobby.max_players),
            ]
            */
    }

    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let selected_style = Style::default()
                    .fg(Color::Green)
                    .modifier(Modifier::UNDERLINED);
                let normal_style = Style::default().fg(Color::White);
                let header = ["Lobby Name", "Server IP", "Game Type", "Players"];
                let rows = self.items.iter().enumerate().map(|(i, item)| {

                    let data = vec![item.name.clone()];

                    if i == self.selected {
                        Row::StyledData(data.into_iter(), selected_style)
                    } else {
                        Row::StyledData(data.into_iter(), normal_style)
                    }
                });

                let rects = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(1)
                    .split(f.size());
                Table::new(header.iter(), rows)
                    .header_style(Style::default().fg(Color::Blue))
                    .block(Block::default().borders(Borders::ALL).title("Game List"))
                    .widths(&[
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ])
                    .column_spacing(1)
                    .render(&mut f, rects[0]);
            })
            .unwrap();
    }

    async fn on_event(&mut self, event: Event) -> Action {
        match event {
            Event::Input(key) => match key {
                Key::Down => {
                    self.selected += 1;
                    if self.selected > self.items.len() - 1 {
                        self.selected = 0;
                    }
                }
                Key::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    } else {
                        self.selected = self.items.len() - 1;
                    }
                }
                Key::Char('\n') => {
                    if !self.items.is_empty() {
                        let lobby = &self.items[self.selected];

                        let url = format!("http://localhost:8000/lobbies/{}/join", lobby.name);

                        let res: Result<common::JoinResponse, _> =
                            surf::post(url).await.unwrap().body_json().await;

                        if let Ok(res) = res {
                            use crate::states::TicTacToe;

                            return Action::PushState(Box::new(TicTacToe::new(res.player, &lobby.name)));
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
