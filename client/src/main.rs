#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::Terminal;

use crate::util::event::{Event, Events};

mod state;
use state::StateManager;

mod states;
use states::Connect;
use states::GameBrowser;
use states::MainMenu;
use states::TicTacToe;

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut state_manager = StateManager::new();

    state_manager.push(Box::new(TicTacToe::new()));

    // Input
    loop {
        state_manager.render(&mut terminal);

        let event = events.next()?;

        if let Event::Input(key) = event {
            if let Key::Char('q') = key {
                break;
            }
        }

        state_manager.on_event(event);
    }

    Ok(())
}
