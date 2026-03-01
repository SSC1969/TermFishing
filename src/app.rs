use crate::event::{AppEvent, Event, EventHandler};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

#[derive(Clone, Default, Debug)]
pub enum Menu {
    #[default]
    Home,
    Inventory,
    Collection,
    Options,
}

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

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Currently open menu
    pub menu: Menu,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            menu: Menu::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
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
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Left => self.events.send(AppEvent::ScrollLeft),
            KeyCode::Right => self.events.send(AppEvent::ScrollRight),
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
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
