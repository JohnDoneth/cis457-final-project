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
use tui::widgets::{Block, Borders, List, Paragraph, Row, SelectableList, Table, Text, Widget};
use unicode_width::UnicodeWidthStr;

use std::io::{self, Write};

use termion::cursor::Goto;

use crate::states::CreateGame;
use crate::states::GameBrowser;
pub struct MainMenu {
    selected: usize,
    items: Vec<String>,
    address: String,
}

impl MainMenu {
    pub fn new(server_address: &str) -> Self {
        Self {
            selected: 0,
            items: vec![String::from("Create a game"), String::from("Join a game")],
            address: server_address.to_owned(),
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl State for MainMenu {
    async fn on_update(&mut self) {}

    async fn on_enter(&mut self) {}

    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(1)
                    .split(f.size());

                SelectableList::default()
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Select an Action"),
                    )
                    .items(&self.items)
                    .select(Some(self.selected))
                    .highlight_style(
                        Style::default()
                            .fg(Color::LightGreen)
                            .modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">")
                    .render(&mut f, chunks[0]);
            })
            .unwrap();
    }

    async fn on_event(&mut self, event: Event) -> Action {
        match event {
            Event::Input(input) => match input {
                Key::Down => {
                    self.selected = {
                        if self.selected >= self.items.len() - 1 {
                            0
                        } else {
                            self.selected + 1
                        }
                    };
                }
                Key::Up => {
                    self.selected = {
                        if self.selected > 0 {
                            self.selected - 1
                        } else {
                            self.items.len() - 1
                        }
                    };
                }
                Key::Char('\n') => {
                    if self.selected == 0 {
                        return Action::PushState(Box::new(CreateGame::new()));
                    }
                    if self.selected == 1 {
                        return Action::PushState(Box::new(GameBrowser::new()));
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}
