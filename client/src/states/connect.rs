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
use tui::widgets::{Block, Borders, List, Paragraph, Row, Table, Text, Widget};
use unicode_width::UnicodeWidthStr;

use std::io::{self, Write};

use termion::cursor::Goto;

pub struct Connect {
    address: String,
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Connect {
    pub fn new() -> Self {
        Self {
            address: String::new(),
            input: String::new(),
            messages: Vec::new(),
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl State for Connect {
    async fn on_update(&mut self) {}

    async fn on_enter(&mut self) {}

    fn render(&mut self, terminal: &mut Terminal<Backend>) {
        terminal
            .draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                    .split(f.size());

                Paragraph::new(
                    [Text::raw(String::from(
                        "Please enter the matchmaking servers IP Address.",
                    ))]
                    .iter(),
                )
                .style(Style::default().fg(Color::Blue))
                .render(&mut f, chunks[1]);

                Paragraph::new([Text::raw(&self.input)].iter())
                    .style(Style::default().fg(Color::Green))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Server IP Address"),
                    )
                    .render(&mut f, chunks[0]);
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
                Key::Char('\n') => {
                    self.messages.push(self.input.drain(..).collect());
                }
                Key::Char(c) => {
                    self.input.push(c);
                }
                Key::Backspace => {
                    self.input.pop();
                }
                _ => {}
            },
            _ => {}
        }

        Action::None
    }
}
