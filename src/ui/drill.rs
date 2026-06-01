use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use super::{helpers::centered_rect, theme::*};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let q = match app.current_q() { Some(q) => q, None => return };
    let total = app.questions.len();
    let idx   = app.q_index;

    let vert = centered_rect(80, 85, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(vert);

    // Progress bar
    let pct       = idx * 100 / total.max(1);
    let bar_width = (chunks[0].width as usize).saturating_sub(10);
    let filled    = bar_width * pct / 100;
    let bar_str   = format!("{}{}  {}/{}", "█".repeat(filled), "░".repeat(bar_width - filled), idx, total);
    f.render_widget(
        Paragraph::new(bar_str).style(Style::default().fg(PURPLE)),
        chunks[0],
    );

    // Verb / tense / person hint
    let hint = Line::from(vec![
        Span::styled(q.verb,   Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
        Span::styled("  ·  ",  Style::default().fg(BORDER)),
        Span::styled(q.tense,  Style::default().fg(LAVENDER)),
        Span::styled("  ·  ",  Style::default().fg(BORDER)),
        Span::styled(q.person, Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(
        Paragraph::new(hint)
            .block(Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(BORDER)))
            .alignment(Alignment::Left),
        chunks[1],
    );

    // Sentence with blank
    let sentence = build_sentence_line(q.before, q.after);
    f.render_widget(
        Paragraph::new(Text::from(vec![Line::from(""), sentence]))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(PANEL_BG)))
            .wrap(Wrap { trim: false }),
        chunks[2],
    );

    // Input box
    let cursor      = if (app.tick / 8) % 2 == 0 { "█" } else { " " };
    let input_text  = format!(" {}{}", app.input, cursor);
    f.render_widget(
        Paragraph::new(input_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(PURPLE))
                .style(Style::default().bg(Color::Rgb(30, 24, 54)))),
        chunks[3],
    );

    // Help bar
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("[Enter] submit  ",   Style::default().fg(DIM_WHITE)),
            Span::styled("[Esc] back to menu", Style::default().fg(DIM_WHITE)),
        ])).alignment(Alignment::Center),
        chunks[4],
    );
}

fn build_sentence_line<'a>(before: &'a str, after: &'a str) -> Line<'a> {
    const BLANK: &str = "___";
    let placeholder = Span::styled("▓▓▓▓▓▓▓▓▓", Style::default().fg(GOLD).add_modifier(Modifier::BOLD));
    let after_span  = Span::styled(after, Style::default().fg(Color::Rgb(100, 95, 130)));

    if let Some(pos) = before.find(BLANK) {
        Line::from(vec![
            Span::styled(&before[..pos],              Style::default().fg(DIM_WHITE)),
            placeholder,
            Span::styled(&before[pos + BLANK.len()..], Style::default().fg(DIM_WHITE)),
            after_span,
        ])
    } else {
        Line::from(vec![
            Span::styled(before, Style::default().fg(DIM_WHITE)),
            placeholder,
            after_span,
        ])
    }
}
