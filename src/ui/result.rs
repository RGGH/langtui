use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use super::{helpers::centered_rect, theme::*};

pub fn draw(
    f: &mut Frame, app: &App, area: Rect,
    correct: bool, expected: &str, before: &str, after: &str,
) {
    let popup = centered_rect(70, 55, area);
    f.render_widget(Clear, popup);

    let (border_col, icon, verdict) = if correct {
        (GREEN, "✓", "Correct!")
    } else {
        (RED, "✗", "Not quite")
    };

    let score_pct = if app.session_total > 0 {
        app.session_correct * 100 / app.session_total
    } else { 0 };

    let content = vec![
        Line::from(vec![
            Span::styled(format!(" {icon}  "), Style::default().fg(border_col).add_modifier(Modifier::BOLD)),
            Span::styled(verdict,             Style::default().fg(border_col).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("     {}/{} ({}%)", app.session_correct, app.session_total, score_pct),
                Style::default().fg(DIM_WHITE),
            ),
        ]),
        Line::from(""),
        if !correct {
            Line::from(vec![
                Span::styled(" Answer: ", Style::default().fg(DIM_WHITE)),
                Span::styled(expected,   Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            ])
        } else {
            Line::from("")
        },
        Line::from(""),
        build_completed_sentence(before, after, expected),
        Line::from(""),
        Line::from(vec![
            Span::styled("[Space/Enter] next   [Esc] menu",
                Style::default().fg(Color::Rgb(90, 85, 110))),
        ]),
    ];

    f.render_widget(
        Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(border_col))
                .style(Style::default().bg(Color::Rgb(20, 18, 36))))
            .wrap(Wrap { trim: false }),
        popup,
    );
}

fn build_completed_sentence<'a>(before: &'a str, after: &'a str, expected: &'a str) -> Line<'a> {
    const BLANK: &str = "___";
    let answer_span = Span::styled(expected, Style::default().fg(GREEN).add_modifier(Modifier::BOLD));
    let after_span  = Span::styled(after, Style::default().fg(Color::Rgb(110, 105, 140)));

    if let Some(pos) = before.find(BLANK) {
        Line::from(vec![
            Span::styled(&before[..pos],               Style::default().fg(DIM_WHITE)),
            answer_span,
            Span::styled(&before[pos + BLANK.len()..], Style::default().fg(DIM_WHITE)),
            after_span,
        ])
    } else {
        Line::from(vec![
            Span::styled(before, Style::default().fg(DIM_WHITE)),
            answer_span,
            after_span,
        ])
    }
}
