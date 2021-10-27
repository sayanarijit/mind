use crate::backend::information::InformationImpl;
use crate::backend::state::State;
use crate::backend::state::Step;
use crate::backend::storage::Storage;
use crate::backend::ui;
use anyhow::Result;
use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use std::io;
use std::path::PathBuf;
use tui::backend::CrosstermBackend;
use tui::terminal::Terminal;
use tui_input::backend::crossterm as input_backend;
use tui_input::{Input, InputResponse};

enum Signal {
    Continue,
    Quit,
}

pub struct Runner {
    storage: Storage,
    state: State,
}

impl Runner {
    pub fn new(storage_path: PathBuf) -> Result<Self> {
        let storage = Storage::new(storage_path)?;
        let state = State::default();
        let runner = Self { storage, state };
        Ok(runner)
    }

    fn read_storage(&mut self) -> Result<()> {
        let mut list = self.storage.list_information()?;
        list.sort_by_key(|l| l.captured_at);
        self.state.information_list = list;

        Ok(())
    }

    fn handle_key(&mut self, evt: KeyEvent) -> Result<Signal> {
        match self.state.current_step {
            Step::Capture => self.handle_key_for_capture(evt),
            _ => {
                todo!()
            }
        }
    }

    fn handle_key_for_capture(&mut self, evt: KeyEvent) -> Result<Signal> {
        if let Some(input) = self.state.input.as_mut() {
            match input_backend::to_input_request(Event::Key(evt))
                .and_then(|req| input.handle(req))
            {
                Some(InputResponse::StateChanged(_)) => Ok(Signal::Continue),
                Some(InputResponse::Escaped) => Ok(Signal::Quit),
                Some(InputResponse::Submitted) => {
                    if input.value().is_empty() {
                        Ok(Signal::Quit)
                    } else {
                        let info = InformationImpl::new(input.value());
                        self.storage.save_information(&info)?;
                        *input = Input::default();
                        Ok(Signal::Continue)
                    }
                }
                None => Ok(Signal::Continue),
            }
        } else {
            self.state.input = Some(Input::default());
            self.handle_key_for_capture(evt)
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;

        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        self.read_storage()?;
        terminal.draw(|f| ui::draw(f, &self.state))?;
        loop {
            match event::read()? {
                Event::Key(k) => match self.handle_key(k) {
                    Ok(Signal::Continue) => {}
                    Ok(Signal::Quit) => {
                        break;
                    }
                    Err(_) => {
                        break;
                    }
                },
                _ => {}
            };
            self.read_storage()?;
            terminal.draw(|f| ui::draw(f, &self.state))?;
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}
