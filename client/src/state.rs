use tui::backend::TermionBackend;
use tui::terminal::Frame;
use tui::Terminal;

use std::io::Stdout;
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use crate::util::event::{Event, Events};
use std::io::Write;

pub type Backend = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>;

pub enum Action {
    None,
    PushState(Box<dyn State>),
}

pub struct StateManager {
    states: Vec<Box<State>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self { states: vec![] }
    }

    pub fn push(&mut self, state: Box<dyn State>) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub fn current(&mut self) -> Option<&mut Box<dyn State>> {
        self.states.last_mut()
    }

    pub fn render(&mut self, terminal: &mut Terminal<Backend>) {
        if let Some(state) = self.current() {
            state.render(terminal);
        }
    }

    pub fn on_event(&mut self, event: Event) {
        if let Some(state) = self.current() {
            state.on_event(event);
        }
    }
}

pub trait State {
    fn render(&mut self, terminal: &mut Terminal<Backend>);

    fn on_event(&mut self, event: Event) -> Action;
}
