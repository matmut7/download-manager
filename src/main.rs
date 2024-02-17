use download_manager::app::{App, AppResult};
use download_manager::download::controller::Message;
use download_manager::events::event::{Event, EventHandler};
use download_manager::events::handler::handle_key_events;
use download_manager::ui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{env, io, str::FromStr};
use url::Url;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    start_example_downloads(&mut app);

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        tokio::select! {
            Some(message) = app.controller.rx.recv() => {
                match message {
                    Message::Downloaded(id, downloaded) => {
                        app.controller.get_worker(id).downloaded = downloaded;
                    },
                    Message::Total(id, total) => {
                        app.controller.get_worker(id).total_size = total;
                    },
                    Message::Paused(id, paused) => {
                        app.controller.get_worker(id).paused = paused;
                    }
                    Message::Done(id) => {
                        app.controller.get_worker(id).done = true;
                    }
                    Message::Speed(id, speed) => {
                        app.controller.get_worker(id).speed = speed
                    }
                }
            }
            Ok(event) = tui.events.next() => {
                match event {
                    Event::Tick => app.tick(),
                    Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }

        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

fn start_example_downloads(app: &mut App) {
    dotenv::dotenv().ok();
    if let Ok(file) = env::var("SMALL_FILE_URL") {
        if let Ok(url) = Url::from_str(&file) {
            app.controller.download(url)
        }
    }
    if let Ok(file) = env::var("MEDIUM_FILE_URL") {
        if let Ok(url) = Url::from_str(&file) {
            app.controller.download(url)
        }
    }
    if let Ok(file) = env::var("LARGE_FILE_URL") {
        if let Ok(url) = Url::from_str(&file) {
            app.controller.download(url)
        }
    }
}
