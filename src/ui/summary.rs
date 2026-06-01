use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use super::{helpers::centered_rect, theme::*};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);

    let pct = if app.session_total > 0 {
        app.session_correct * 100 / app.session_total
    } else { 0 };

    let (colour, msg) = match pct {
        100     => (GREEN, "¡Perfecto! Flawless run. 🎉"),
        80..=99 => (GREEN, "Muy bien — solid session!"),
        50..=79 => (GOLD,  "Keep going — repetition is the key."),
        _       => (RED,   "Rough one. Check the heatmap for weak spots."),
    };

    let score_line = format!("{}/{}", app.session_correct, app.session_total);

    let content = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(&score_line, Style::default().fg(colour)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)),
            Span::styled(format!("  ({pct}%)"), Style::default().fg(DIM_WHITE)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(msg, Style::default().fg(colour))]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("[Enter/r] drill again   ", Style::default().fg(DIM_WHITE)),
            Span::styled("[h] heatmap   ",           Style::default().fg(DIM_WHITE)),
            Span::styled("[s] streak   ",            Style::default().fg(DIM_WHITE)),
            Span::styled("[Esc] menu",               Style::default().fg(DIM_WHITE)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(content)
            .block(Block::default()
                .title(Span::styled(" Session Complete ",
                    Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(PURPLE))
                .style(Style::default().bg(PANEL_BG)))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false }),
        popup,
    );
}
