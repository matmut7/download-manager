use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.add_download.showing {
        match key_event.code {
            KeyCode::Esc => {
                app.add_download.close();
            }
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                app.add_download.close();
            }
            KeyCode::Char('m') if key_event.modifiers == KeyModifiers::CONTROL => {}
            KeyCode::Enter => {
                app.add_download();
            }
            _ => {
                app.add_download.textarea.input(key_event);
            }
        }
    } else {
        match key_event.code {
            KeyCode::Esc if app.add_download.showing => {
                app.add_download.close();
            }
            KeyCode::Esc | KeyCode::Char('q') if !app.add_download.showing => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                app.quit();
            }
            KeyCode::Char('a') => app.add_download.open(),
            KeyCode::Char('j') => app.select_down(),
            KeyCode::Char('k') => app.select_up(),
            KeyCode::Char(' ') => {
                let worker = app.controller.workers.get(app.selected).unwrap();
                if !worker.done {
                    worker.pause_tx.send(()).unwrap();
                }
            }
            _ => {}
        }
    }
    Ok(())
}
