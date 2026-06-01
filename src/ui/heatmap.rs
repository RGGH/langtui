use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crate::{app::App, data};
use super::{helpers::centered_rect, theme::*};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" ERROR HEATMAP ", Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)),
            Span::styled("  [Esc] menu   [r] drill   [s] streak", Style::default().fg(DIM_WHITE)),
        ])).block(Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(PANEL_BG))),
        outer[0],
    );

    let inner  = centered_rect(90, 95, outer[1]);
    let verb_w = 16usize;
    let cell_w = 14usize;

    let mut header_spans = vec![Span::raw(format!("{:width$}", "", width = verb_w))];
    for (i, _) in data::TENSES.iter().enumerate() {
        header_spans.push(Span::styled(
            format!("{:^width$}", data::TENSE_LABELS[i], width = cell_w),
            Style::default().fg(DIM_WHITE).add_modifier(Modifier::BOLD),
        ));
    }

    let mut lines = vec![Line::from(""), Line::from(header_spans), Line::from("")];

    for &verb in data::VERBS {
        let mut row = vec![
            Span::styled(
                format!("{:<width$}", verb, width = verb_w),
                Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD),
            )
        ];
        for &tense in data::TENSES {
            let cell = app.stats.cell(verb, tense);
            let (bg, fg, label) = if cell.total() == 0 {
                (CELL_GREY, Color::Rgb(140, 135, 160), format!("{:^12}", "·"))
            } else {
                let pct = cell.error_pct();
                if pct == 0.0 {
                    (CELL_GREEN,  Color::Rgb(20, 40, 25),
                     format!("{:^12}", format!("✓ {}", cell.total())))
                } else if pct <= 0.40 {
                    (CELL_YELLOW, Color::Rgb(40, 35, 10),
                     format!("{:^12}", format!("{}% err", (pct * 100.0).round() as u32)))
                } else {
                    (CELL_RED,    Color::Rgb(40, 10, 10),
                     format!("{:^12}", format!("{}% err", (pct * 100.0).round() as u32)))
                }
            };
            row.push(Span::styled(label, Style::default().fg(fg).bg(bg)));
            row.push(Span::raw(" "));
        }
        lines.push(Line::from(row));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("  no errors  ", Style::default().fg(Color::Rgb(20,40,25)).bg(CELL_GREEN)),
        Span::raw("  "),
        Span::styled("  ≤40% err   ", Style::default().fg(Color::Rgb(40,35,10)).bg(CELL_YELLOW)),
        Span::raw("  "),
        Span::styled("  >40% err   ", Style::default().fg(Color::Rgb(40,10,10)).bg(CELL_RED)),
        Span::raw("  "),
        Span::styled("  not tried  ", Style::default().fg(Color::Rgb(140,135,160)).bg(CELL_GREY)),
    ]));

    f.render_widget(
        Paragraph::new(lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(PANEL_BG))),
        inner,
    );
}
