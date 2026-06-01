pub mod theme;
mod helpers;
mod menu;
mod drill;
mod result;
mod summary;
mod heatmap;
mod streak;

use ratatui::{style::Style, widgets::Block, Frame};
use crate::app::{App, Screen};
use theme::BG;

pub fn render(f: &mut Frame, app: &App) {
    let full = f.area();
    f.render_widget(Block::default().style(Style::default().bg(BG)), full);

    match &app.screen {
        Screen::Menu    => menu::draw(f, app, full),
        Screen::Drill   => drill::draw(f, app, full),
        Screen::Result { correct, expected, before, after } =>
            result::draw(f, app, full, *correct, expected, before, after),
        Screen::Summary => summary::draw(f, app, full),
        Screen::Heatmap => heatmap::draw(f, app, full),
        Screen::Streak  => streak::draw(f, app, full),
    }
}
