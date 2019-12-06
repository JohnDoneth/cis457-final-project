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
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Row, Table, Tabs, Text, Widget};
use unicode_width::UnicodeWidthStr;

use std::io::{self, Write};
use uuid::Uuid;

use termion::cursor::Goto;

use common::rockpaperscissors::*;
use common::Game;

pub struct RockPaperScissors {
    address: String,
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Vec<String>,

    player: Uuid,
    lobby: String,

    state: GameState,
    status: String,
    round: usize,
    move_selection: usize,
    history: Vec<HistoryEntry>,
}

impl RockPaperScissors {
    pub fn new(player: Uuid, lobby: &str) -> Self {
        Self {
            address: String::new(),
            input: String::new(),
            messages: Vec::new(),

            player,
            lobby: lobby.to_string(),

            state: GameState::WaitingForPlayers { players: vec![] },
            status: String::new(),
            round: 0,

            move_selection: 0,

            history: vec![],
        }
    }

    pub async fn fetch_state(&mut self) {
        // fetch the game state
        let url = format!("http://localhost:8000/lobbies/{}/state", self.lobby);

        let game: Game = surf::get(url).await.unwrap().body_json().await.unwrap();

        match game {
            Game::RockPaperScissors(s) => {
                self.state = s;
            }
            _ => panic!("wrong game type"),
        }

        self.update()
    }

    pub fn update(&mut self) {
        match &self.state {
            GameState::WaitingForPlayers { .. } => {
                self.status = format!("Waiting for another player before the game will begin.");
            }
            GameState::WaitingForInput {
                players,
                round,
                history,
                ..
            } => {
                self.round = *round;
                self.history = history.clone();

                self.status = format!("Waiting for input");
            }
            GameState::GameOver { winner, history } => {
                self.history = vec![];

                if self.player == *winner {
                    self.status = format!("The game is over, you've won!");
                } else {
                    self.status = format!("The game is over, you've lost.");
                }
            }
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl State for RockPaperScissors {
    async fn on_enter(&mut self) {}

    async fn on_update(&mut self) {
        self.fetch_state().await;
    }

    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
                    .split(f.size());

                Paragraph::new(
                    [Text::raw(
                        self.status.clone(),
                    )]
                    .iter(),
                )
                .style(Style::default().fg(Color::Blue))
                .render(&mut f, chunks[0]);

                let mut rows = vec![];

                for entry in &self.history {

                    for player_move in &entry.moves {
                        rows.push(format!("{:?}", player_move));
                    }
                    
                    match entry.winner {
                        Some(p) => {
                            if p == self.player {
                                rows.push(format!("You won that round!"));
                            } else {
                                rows.push(format!("You lost that round."));
                            }
                        }
                        None => {
                            rows.push(format!("It was a tie!"));
                        }
                    }
                    
                }

                let rows: Vec<Text> = rows.iter().map(|s| Text::raw(format!("{}\n", s))).collect();

                Paragraph::new(
                    rows.iter(),
                )
                .style(Style::default().fg(Color::Blue))
                .render(&mut f, chunks[1]);

                Tabs::default()
                    .titles(&["Rock", "Paper", "Scissors"])
                    .style(Style::default().fg(Color::White))
                    .select(self.move_selection)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Green)
                            .modifier(Modifier::UNDERLINED),
                    )
                    .render(&mut f, chunks[2])
            })
            .unwrap();

        // Put the cursor back inside the input box
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + self.input.width() as u16, 4)
        )
        .unwrap();
        // stdout is buffered, flush it to see the effect immediately when hitting backspace
        io::stdout().flush().ok();
    }

    async fn on_event(&mut self, event: Event) -> Action {
        match event {
            Event::Input(input) => match input {
                Key::Right => {
                    self.move_selection += 1;
                    if self.move_selection > 3 - 1 {
                        self.move_selection = 0;
                    }
                }
                Key::Left => {
                    if self.move_selection > 0 {
                        self.move_selection -= 1;
                    } else {
                        self.move_selection = 3 - 1;
                    }
                },
                Key::Char('\n') => {
                    let player_move = if self.move_selection == 0 {
                        Move::Rock
                    } else if self.move_selection == 1 {
                        Move::Paper
                    } else {
                        Move::Scissors
                    };

                    let url = format!("http://localhost:8000/lobbies/{}/action", self.lobby);

                    let res: serde_json::Value = surf::post(url)
                        .body_json(&common::Action::RockPaperScissors(PlayerAction::Move {
                            player: self.player,
                            action: player_move,
                        }))
                        .unwrap()
                        .await
                        .unwrap()
                        .body_json()
                        .await
                        .unwrap();

                    if let Ok(new_state) = serde_json::from_value::<Game>(res) {
                        match new_state {
                            Game::RockPaperScissors(new_state) => {
                                self.state = new_state;
                                self.update();
                            }
                            _ => panic!("wrong game type"),
                        }
                    }
                },
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}
