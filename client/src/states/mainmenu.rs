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

use crate::states::GameBrowser;

pub struct MainMenu {
    selected: usize,
    items: Vec<String>,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            selected: 0,
            items: vec![String::from("Create a game"), String::from("Join a game")],
        }
    }
}

impl State for MainMenu {
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

    fn on_event(&mut self, event: Event) -> Action {
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
                    return Action::PushState(Box::new(GameBrowser::new()));
                }
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}