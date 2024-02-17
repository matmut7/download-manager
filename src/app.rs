use std::{error, str::FromStr};

use url::Url;

use crate::{download::controller::Controller, ui::textarea::AddDownload};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub add_download: AddDownload,
    pub controller: Controller,
    pub selected: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            add_download: AddDownload::default(),
            controller: Controller::default(),
            selected: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_download(&mut self) {
        let url = Url::from_str(&self.add_download.textarea.lines()[0]).unwrap();
        self.controller.download(url);
        self.add_download.close();
        self.add_download.textarea.select_all();
        self.add_download.textarea.cut();
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn select_down(&mut self) {
        if self.selected < self.controller.workers.len() - 1 {
            self.selected += 1
        }
    }

    pub fn select_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1
        }
    }
}
