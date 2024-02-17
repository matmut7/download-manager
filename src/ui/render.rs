use byte_unit::{Byte, UnitType};
use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    symbols,
    widgets::{Block, LineGauge, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let split = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.size());
    render_header(frame, split[0]);
    render_table(app, frame, split[1]);
    render_prompt(app, frame);
}

fn render_table(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::bordered();
    frame.render_widget(&block, area);
    let vertical_split =
        Layout::vertical(vec![Constraint::Length(1); app.controller.workers.len()])
            .flex(Flex::Start)
            .split(block.inner(area));
    let horizontal_split = Layout::horizontal([
        Constraint::Length(3),
        Constraint::Percentage(30),
        Constraint::Length(20),
        Constraint::Fill(1),
        Constraint::Length(1),
    ]);
    for (index, worker) in app.controller.workers.iter().enumerate() {
        let name = Paragraph::new(worker.file_name.clone());
        let progress = LineGauge::default()
            .ratio(worker.ratio())
            .line_set(symbols::line::THICK)
            .gauge_style(Style::new().black().on_white());
        let areas = horizontal_split.split(vertical_split[index]);
        if index == app.selected {
            frame.render_widget(Paragraph::new(" > "), areas[0])
        }
        frame.render_widget(name, areas[1]);
        if worker.done {
            frame.render_widget(Paragraph::new("done"), areas[2])
        } else if worker.paused {
            frame.render_widget(Paragraph::new("paused"), areas[2])
        } else {
            frame.render_widget(
                Paragraph::new(format!(
                    "{:.2}/s",
                    Byte::from_u64(worker.speed).get_appropriate_unit(UnitType::Decimal)
                )),
                areas[2],
            );
        }
        frame.render_widget(progress, areas[3]);
    }
}

fn render_header(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new(
            "Esc, Ctrl-C, q: stop running\n\
            j, k: move focus\n\
            space: pause and resume focused download\n\
            a: add new download"
                .to_string(),
        )
        .block(
            Block::bordered()
                .title("TUI download manager")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default())
        .left_aligned(),
        area,
    );
}

fn render_prompt(app: &App, frame: &mut Frame) {
    if app.add_download.showing {
        let vertical = Layout::default()
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .flex(Flex::Center);
        let horizontal = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Percentage(70),
            Constraint::Min(1),
        ])
        .flex(Flex::Center);
        frame.render_widget(
            app.add_download.textarea.widget(),
            vertical.split(horizontal.split(frame.size())[1])[1],
        );
    }
}
