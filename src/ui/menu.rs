use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use ratatui::layout::Rect;
use crate::app::App;
use super::{helpers::centered_rect, theme::*};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let vert = centered_rect(60, 70, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(2),
            Constraint::Length(8),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(vert);

    let pulse = (app.tick / 12) % 3;
    let title = vec![
        Line::from(vec![Span::styled(
            "  🇪🇸  Reflexive Verb Drill  🇪🇸",
            Style::default().fg(PURPLE).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            match pulse {
                0 => "·  irse · quedarse · ponerse · llevarse · acordarse  ·",
                1 => "—  irse · quedarse · ponerse · llevarse · acordarse  —",
                _ => "›  irse · quedarse · ponerse · llevarse · acordarse  ‹",
            },
            Style::default().fg(LAVENDER),
        )]),
        Line::from(vec![Span::styled(
            "   yo · tú · él/ella  ·  4 tenses  ·  60 questions",
            Style::default().fg(DIM_WHITE),
        )]),
    ];
    f.render_widget(
        Paragraph::new(title)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(PANEL_BG)))
            .alignment(Alignment::Center),
        chunks[0],
    );

    let menu_items = vec![
        Line::from(vec![
            Span::styled("  [Enter]  ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
            Span::styled("Start drill", Style::default().fg(DIM_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("  [h]      ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
            Span::styled("Error heatmap", Style::default().fg(DIM_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("  [s]      ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
            Span::styled("Streak calendar", Style::default().fg(DIM_WHITE)),
        ]),
        Line::from(vec![
            Span::styled("  [q]      ", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
            Span::styled("Quit", Style::default().fg(DIM_WHITE)),
        ]),
    ];
    f.render_widget(
        Paragraph::new(menu_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(PANEL_BG))),
        chunks[2],
    );
}
