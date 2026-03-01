use std::time::Duration;

use crate::{
    event::{AppEvent, Event, EventHandler},
    player::Player,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::{Rng, RngExt};
use ratatui::{DefaultTerminal, widgets::ListState};
use tokio::{sync::mpsc::Sender, time::sleep};
use tui_input::{Input, backend::crossterm::EventHandler as crosstermEventHandler};

#[derive(Clone, Default, Debug)]
pub enum Menu {
    #[default]
    Home,
    Inventory,
    Collection,
    Options,
}

pub const MENU_SIZE: i32 = 4;

impl Menu {
    fn next(&self) -> Self {
        match self {
            Menu::Home => Menu::Inventory,
            Menu::Inventory => Menu::Collection,
            Menu::Collection => Menu::Options,
            Menu::Options => Menu::Home,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Menu::Home => Menu::Options,
            Menu::Inventory => Menu::Home,
            Menu::Collection => Menu::Inventory,
            Menu::Options => Menu::Collection,
        }
    }
}

pub enum Anim {
    DEFAULT,
    BITING,
    CATCHING,
}
/// Application.
pub struct App {
    pub tx: Sender<String>,
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Currently open menu
    pub menu: Menu,
    /// Player data struct
    pub player: Player,
    /// Backpack state for ui
    pub backpack_state: ListState,

    pub input: Input,
    // Whether the chatbox is open or not
    pub input_mode: InputMode,
    // most recent n messages
    pub messages: Vec<String>,

    pub cursor_position: Option<(u16, u16)>,

    pub anim: Anim,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(tx: Sender<String>) -> Self {
        Self {
            tx: tx,
            running: true,
            events: EventHandler::new(),
            menu: Menu::default(),
            player: Player::default(),
            backpack_state: ListState::default(),
            input: Input::new(std::string::String::from("")),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            cursor_position: Option::Some((0, 0)),
            anim: Anim::DEFAULT,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                frame.render_widget(&mut self, frame.area());
                if let Some(pos) = self.cursor_position {
                    frame.set_cursor_position(pos);
                }
            })?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::ChangeMenu(menu) => self.menu = menu,
                    AppEvent::ScrollLeft => self.menu = self.menu.prev(),
                    AppEvent::ScrollRight => self.menu = self.menu.next(),
                    AppEvent::FishBiting => {
                        self.anim = Anim::BITING;
                        self.events.send(AppEvent::SendChat("biting...".to_owned()));
                        // let tx = self.events.sender();
                        // tokio::spawn(async move {
                        //     sleep(Duration::from_millis(1000)).await;
                        //     tx.send(Event::App(AppEvent::FishCatching));
                        // });
                    }
                    AppEvent::FishCatching => {
                        self.anim = Anim::CATCHING;
                        let tx = self.events.sender();
                        tokio::spawn(async move {
                            sleep(Duration::from_millis(2000)).await;
                            tx.send(Event::App(AppEvent::FishCaught));
                        });
                    }
                    AppEvent::FishCaught => {
                        self.player.catch_fish();
                        self.anim = Anim::DEFAULT;
                    }
                    AppEvent::ChangeInputMode(im) => match im {
                        InputMode::Normal => self.input_mode = im,
                        InputMode::Editing => self.input_mode = im,
                    },
                    AppEvent::SendChat(msg) => self.tx.send(msg).await?,
                    AppEvent::MessageReceived(msg) => self.messages.push(msg),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if self.input_mode == InputMode::Editing {
            match key_event.code {
                KeyCode::Enter => {
                    let msg = self.input.value().to_owned();
                    self.input.reset();
                    self.messages.push(msg.clone());
                    self.events.send(AppEvent::SendChat(msg));
                }
                KeyCode::Esc => self
                    .events
                    .send(AppEvent::ChangeInputMode(InputMode::Normal)),
                _ => {
                    self.input
                        .handle_event(&crossterm::event::Event::Key(key_event));
                }
            }
            return Ok(());
        }
        if matches!(self.anim, Anim::BITING) && matches!(key_event.code, KeyCode::Char('f')) {
            self.events.send(AppEvent::FishCatching);
        }
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Left => self.events.send(AppEvent::ScrollLeft),
            KeyCode::Right => self.events.send(AppEvent::ScrollRight),
            KeyCode::Char(' ') => self.events.send(AppEvent::FishBiting),
            KeyCode::Char('t') => self
                .events
                .send(AppEvent::ChangeInputMode(InputMode::Editing)),
            KeyCode::Char(c) => self.events.send(AppEvent::ChangeMenu(match c {
                'h' => Menu::Home,
                'c' => Menu::Collection,
                'i' => Menu::Inventory,
                'o' => Menu::Options,
                _ => Menu::Home,
            })),

            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        if matches!(self.anim, Anim::DEFAULT) {
            let mut rng = rand::rng();
            if rng.random_range(0..300) == 1 {
                self.events.send(AppEvent::FishBiting);
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
