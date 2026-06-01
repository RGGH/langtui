use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crate::{app::App, stats};
use super::{helpers::centered_rect, theme::*};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" STREAK CALENDAR ", Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)),
            Span::styled("  [Esc] menu   [h] heatmap", Style::default().fg(DIM_WHITE)),
        ])).block(Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(PANEL_BG))),
        outer[0],
    );

    let inner = centered_rect(60, 80, outer[1]);
    let days  = stats::last_n_days(28);

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "   Mo  Tu  We  Th  Fr  Sa  Su",
            Style::default().fg(DIM_WHITE).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let today          = stats::today_str();
    let first_dow      = stats::dow(&days[0]);
    let mut week_spans = vec![Span::raw("   ")];

    for _ in 0..first_dow {
        week_spans.push(Span::raw("    "));
    }

    for date in &days {
        let count = app.stats.streak.get(date).copied().unwrap_or(0);
        let (glyph, style) = if count == 0 {
            ("  ·  ", Style::default().fg(Color::Rgb(60, 55, 80)))
        } else if count < 10 {
            (" ▪▪  ", Style::default().fg(GOLD))
        } else {
            (" ██  ", Style::default().fg(GREEN)
                .add_modifier(if count >= 25 { Modifier::BOLD } else { Modifier::empty() }))
        };

        let final_style = if date == &today {
            style.add_modifier(Modifier::UNDERLINED)
        } else { style };

        week_spans.push(Span::styled(glyph, final_style));

        if stats::dow(date) == 6 {
            lines.push(Line::from(week_spans.clone()));
            week_spans = vec![Span::raw("   ")];
        }
    }
    if week_spans.len() > 1 {
        lines.push(Line::from(week_spans));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled("  ·  ", Style::default().fg(Color::Rgb(60,55,80))),
        Span::styled(" none   ", Style::default().fg(DIM_WHITE)),
        Span::styled(" ▪▪  ", Style::default().fg(GOLD)),
        Span::styled(" <10    ", Style::default().fg(DIM_WHITE)),
        Span::styled(" ██  ", Style::default().fg(GREEN)),
        Span::styled(" 10+", Style::default().fg(DIM_WHITE)),
    ]));

    let streak_days = count_streak(app, &today);
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled(
            format!("Current streak: {} day{}", streak_days, if streak_days == 1 { "" } else { "s" }),
            Style::default()
                .fg(if streak_days > 0 { GREEN } else { DIM_WHITE })
                .add_modifier(if streak_days > 0 { Modifier::BOLD } else { Modifier::empty() }),
        ),
    ]));

    f.render_widget(
        Paragraph::new(lines)
            .block(Block::default()
                .title(Span::styled(" Last 28 Days ", Style::default().fg(PURPLE)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(PANEL_BG))),
        inner,
    );
}

fn count_streak(app: &App, today: &str) -> u32 {
    let mut n = 0u32;
    for d in stats::last_n_days(365).iter().rev() {
        if app.stats.streak.get(d).copied().unwrap_or(0) > 0 {
            n += 1;
        } else if d != today {
            break;
        }
    }
    n
}
