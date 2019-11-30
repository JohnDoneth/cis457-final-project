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

pub struct GameBrowser {
    items: Vec<Vec<String>>,
    selected: usize,
}

impl GameBrowser {
    pub fn new() -> Self {
        Self {
            items: vec![
                vec![
                    String::from("Cool Kids Only"),
                    String::from("127.0.0.1"),
                    String::from("Rock Paper Scissors"),
                    String::from("(0/2)"),
                ],
                vec![
                    String::from("Cool Kids Only"),
                    String::from("127.0.0.1"),
                    String::from("Rock Paper Scissors"),
                    String::from("(0/2)"),
                ],
                vec![
                    String::from("Cool Kids Only"),
                    String::from("127.0.0.1"),
                    String::from("Rock Paper Scissors"),
                    String::from("(1/2)"),
                ],
                vec![
                    String::from("Cool Kids Only"),
                    String::from("127.0.0.1"),
                    String::from("Tic Tac Toe"),
                    String::from("(0/2)"),
                ],
                vec![
                    String::from("Cool Kids Only"),
                    String::from("127.0.0.1"),
                    String::from("Tic Tac Toe"),
                    String::from("(0/2)"),
                ],
                vec![
                    String::from("Cool Kids Only"),
                    String::from("127.0.0.1"),
                    String::from("Tic Tac Toe"),
                    String::from("(0/2)"),
                ],
            ],
            selected: 0,
        }
    }
}

impl State for GameBrowser {
    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let selected_style = Style::default().fg(Color::Green).modifier(Modifier::BOLD);
                let normal_style = Style::default().fg(Color::White);
                let header = ["Lobby Name", "Server IP", "Game Type", "Players"];
                let rows = self.items.iter().enumerate().map(|(i, item)| {
                    if i == self.selected {
                        Row::StyledData(item.into_iter(), selected_style)
                    } else {
                        Row::StyledData(item.into_iter(), normal_style)
                    }
                });

                let rects = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(1)
                    .split(f.size());
                Table::new(header.into_iter(), rows)
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

    fn on_event(&mut self, event: Event) -> Action {
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
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}
