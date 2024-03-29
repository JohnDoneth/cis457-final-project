#![allow(dead_code)]
#![allow(unused_imports)]

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
use states::CreateGame;
use states::GameBrowser;
use states::MainMenu;
use states::TicTacToe;
use std::panic::{self, PanicInfo};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    let events = Events::new();

    let mut state_manager = StateManager::new();

    //state_manager.push(Box::new(GameBrowser::new())).await;
    state_manager
        .push(Box::new(MainMenu::new("localhost:8000")))
        .await;

    // Input
    loop {
        state_manager.render(&mut terminal);

        if let Ok(event) = events.next() {
            if let Event::Input(key) = event {
                if let Key::Char('q') = key {
                    break;
                }
            }

            state_manager.update().await;

            state_manager.on_event(event).await;
        }
    }

    Ok(())
}

fn panic_hook(info: &PanicInfo<'_>) {
    let location = info.location().unwrap(); // The current implementation always returns Some

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };
    println!(
        "{}thread '<unnamed>' panicked at '{}', {}\r",
        termion::screen::ToMainScreen,
        msg,
        location
    );
}
