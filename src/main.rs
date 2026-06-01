mod data;
mod stats;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};
use rand::seq::SliceRandom;

// ── Palette ───────────────────────────────────────────────────────────────────
const PURPLE:    Color = Color::Rgb(130, 80, 220);
const LAVENDER:  Color = Color::Rgb(180, 150, 255);
const CYAN:      Color = Color::Rgb(80, 210, 220);
const GOLD:      Color = Color::Rgb(230, 190, 60);
const GREEN:     Color = Color::Rgb(80, 210, 120);
const RED:       Color = Color::Rgb(230, 80, 80);
const DIM_WHITE: Color = Color::Rgb(170, 170, 180);
const BG:        Color = Color::Rgb(16, 14, 28);
const PANEL_BG:  Color = Color::Rgb(26, 22, 44);
const BORDER:    Color = Color::Rgb(70, 55, 110);

// Heatmap cell colours
const CELL_GREEN:  Color = Color::Rgb(60, 180, 100);   // no errors
const CELL_YELLOW: Color = Color::Rgb(200, 170, 40);   // ≤40% err
const CELL_RED:    Color = Color::Rgb(200, 70, 70);    // >40% err
const CELL_GREY:   Color = Color::Rgb(70, 65, 90);     // not attempted

// ── App state ─────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
enum Screen {
    Menu,
    Drill,
    Result { correct: bool, expected: String, before: String, after: String },
    Summary,
    Heatmap,
    Streak,
}

struct Question {
    verb:   &'static str,
    tense:  &'static str,
    person: &'static str,
    before: &'static str,
    after:  &'static str,
}

struct App {
    screen:    Screen,
    questions: Vec<Question>,
    q_index:   usize,
    input:     String,
    session_correct: u32,
    session_total:   u32,
    stats:     stats::Stats,
    tick:      u64,   // for animations
}

impl App {
    fn new() -> Self {
        let stats = stats::load();
        App {
            screen:    Screen::Menu,
            questions: vec![],
            q_index:   0,
            input:     String::new(),
            session_correct: 0,
            session_total:   0,
            stats,
            tick: 0,
        }
    }

    fn build_questions(&mut self) {
        let mut rng = rand::thread_rng();
        let mut qs: Vec<Question> = data::VERBS.iter().flat_map(|&v| {
            data::TENSES.iter().flat_map(move |&t| {
                data::PERSONS.iter().map(move |&p| {
                    let options = data::sentences(v, t, p);
                    let (before, after) = options[rand::random::<usize>() % options.len()];
                    Question { verb: v, tense: t, person: p, before, after }
                })
            })
        }).collect();
        qs.shuffle(&mut rng);
        self.questions = qs;
        self.q_index = 0;
        self.session_correct = 0;
        self.session_total = 0;
        self.input.clear();
    }

    fn current_q(&self) -> Option<&Question> {
        self.questions.get(self.q_index)
    }

    fn submit_answer(&mut self) {
        if let Some(q) = self.questions.get(self.q_index) {
            let expected = data::conjugation(q.verb, q.tense, q.person);
            let correct  = self.input.trim().to_lowercase() == expected.to_lowercase();
            self.stats.record(q.verb, q.tense, q.person, correct);
            if correct { self.session_correct += 1; }
            self.session_total += 1;
            self.screen = Screen::Result {
                correct,
                expected: expected.to_string(),
                before:   q.before.to_string(),
                after:    q.after.to_string(),
            };
            self.input.clear();
        }
    }

    fn advance(&mut self) {
        self.q_index += 1;
        if self.q_index >= self.questions.len() {
            stats::save(&self.stats);
            self.screen = Screen::Summary;
        } else {
            self.screen = Screen::Drill;
        }
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────
fn main() -> io::Result<()> {
    // Check args
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--reset") {
        let path = stats::stats_path();
        if path.exists() { std::fs::remove_file(&path)?; println!("Stats wiped."); }
        else { println!("No stats file found."); }
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend  = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut app = App::new();
    let start_screen = if args.iter().any(|a| a == "--stats") {
        Screen::Heatmap
    } else {
        Screen::Menu
    };
    app.screen = start_screen;

    loop {
        app.tick = app.tick.wrapping_add(1);
        term.draw(|f| ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }

                // Global quit
                if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }

                match &app.screen.clone() {
                    Screen::Menu => handle_menu(&mut app, key.code),
                    Screen::Drill => handle_drill(&mut app, key.code),
                    Screen::Result { .. } => handle_result(&mut app, key.code),
                    Screen::Summary => handle_summary(&mut app, key.code),
                    Screen::Heatmap => handle_heatmap(&mut app, key.code),
                    Screen::Streak => handle_streak(&mut app, key.code),
                }

                if matches!(app.screen, Screen::Menu) && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stats::save(&app.stats);
    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

// ── Key handlers ──────────────────────────────────────────────────────────────
fn handle_menu(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Char('1') | KeyCode::Char(' ') => {
            app.build_questions();
            app.screen = Screen::Drill;
        }
        KeyCode::Char('2') | KeyCode::Char('h') => app.screen = Screen::Heatmap,
        KeyCode::Char('3') | KeyCode::Char('s') => app.screen = Screen::Streak,
        KeyCode::Char('q') | KeyCode::Esc => {}
        _ => {}
    }
}

fn handle_drill(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c) => app.input.push(c),
        KeyCode::Backspace => { app.input.pop(); }
        KeyCode::Enter => {
            if !app.input.trim().is_empty() {
                app.submit_answer();
            }
        }
        KeyCode::Esc => {
            stats::save(&app.stats);
            app.screen = Screen::Menu;
        }
        _ => {}
    }
}

fn handle_result(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Char(' ') => app.advance(),
        KeyCode::Esc => {
            stats::save(&app.stats);
            app.screen = Screen::Menu;
        }
        _ => {}
    }
}

fn handle_summary(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Char('r') => {
            app.build_questions();
            app.screen = Screen::Drill;
        }
        KeyCode::Char('h') => app.screen = Screen::Heatmap,
        KeyCode::Char('s') => app.screen = Screen::Streak,
        KeyCode::Esc | KeyCode::Char('q') => app.screen = Screen::Menu,
        _ => {}
    }
}

fn handle_heatmap(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Backspace => app.screen = Screen::Menu,
        KeyCode::Char('s') => app.screen = Screen::Streak,
        KeyCode::Enter | KeyCode::Char('r') => {
            app.build_questions();
            app.screen = Screen::Drill;
        }
        _ => {}
    }
}

fn handle_streak(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Backspace => app.screen = Screen::Menu,
        KeyCode::Char('h') => app.screen = Screen::Heatmap,
        _ => {}
    }
}

// ── UI entry point ────────────────────────────────────────────────────────────
fn ui(f: &mut Frame, app: &App) {
    // Dark background
    let full = f.area();
    f.render_widget(
        Block::default().style(Style::default().bg(BG)),
        full,
    );

    match &app.screen {
        Screen::Menu          => draw_menu(f, app, full),
        Screen::Drill         => draw_drill(f, app, full),
        Screen::Result { correct, expected, before, after } =>
            draw_result(f, app, full, *correct, expected, before, after),
        Screen::Summary       => draw_summary(f, app, full),
        Screen::Heatmap       => draw_heatmap(f, app, full),
        Screen::Streak        => draw_streak(f, app, full),
    }
}

// ── Menu ──────────────────────────────────────────────────────────────────────
fn draw_menu(f: &mut Frame, app: &App, area: Rect) {
    let vert = centered_rect(60, 70, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // title
            Constraint::Length(2),
            Constraint::Length(8),  // menu items
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(vert);

    // Animated title
    let pulse = (app.tick / 12) % 3;
    let title_style = Style::default().fg(PURPLE).add_modifier(Modifier::BOLD);
    let title = vec![
        Line::from(vec![Span::styled("  🇪🇸  Reflexive Verb Drill  🇪🇸", title_style)]),
        Line::from(""),
        Line::from(vec![Span::styled(
            match pulse { 0 => "·  irse · quedarse · ponerse · llevarse · acordarse  ·",
                          1 => "—  irse · quedarse · ponerse · llevarse · acordarse  —",
                          _ => "›  irse · quedarse · ponerse · llevarse · acordarse  ‹" },
            Style::default().fg(LAVENDER),
        )]),
        Line::from(vec![Span::styled(
            "   yo · tú · él/ella  ·  4 tenses  ·  60 questions",
            Style::default().fg(DIM_WHITE),
        )]),
    ];
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(PANEL_BG));
    f.render_widget(Paragraph::new(title).block(title_block).alignment(Alignment::Center), chunks[0]);

    // Menu
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
    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(PANEL_BG));
    f.render_widget(Paragraph::new(menu_items).block(menu_block), chunks[2]);
}

// ── Drill ─────────────────────────────────────────────────────────────────────
fn draw_drill(f: &mut Frame, app: &App, area: Rect) {
    let q = match app.current_q() { Some(q) => q, None => return };
    let total = app.questions.len();
    let idx   = app.q_index;

    let vert = centered_rect(80, 85, area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // progress bar
            Constraint::Length(3),  // verb hint
            Constraint::Length(5),  // sentence
            Constraint::Length(3),  // input
            Constraint::Length(2),  // help
            Constraint::Min(0),
        ])
        .split(vert);

    // Progress bar
    let pct = idx * 100 / total.max(1);
    let bar_width = (chunks[0].width as usize).saturating_sub(10);
    let filled = bar_width * pct / 100;
    let bar_str = format!("{}{}  {}/{}", "█".repeat(filled), "░".repeat(bar_width - filled), idx, total);
    f.render_widget(
        Paragraph::new(bar_str)
            .style(Style::default().fg(PURPLE))
            .block(Block::default().borders(Borders::NONE)),
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
    let blank_marker = "___";
    let before = q.before;
    let after  = q.after;
    let sentence: Line = if let Some(pos) = before.find(blank_marker) {
        let pre  = &before[..pos];
        let post = &before[pos + blank_marker.len()..];
        Line::from(vec![
            Span::styled(pre,  Style::default().fg(DIM_WHITE)),
            Span::styled("▓▓▓▓▓▓▓▓▓", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
            Span::styled(post, Style::default().fg(DIM_WHITE)),
            Span::styled(after, Style::default().fg(Color::Rgb(100, 95, 130))),
        ])
    } else {
        Line::from(vec![
            Span::styled(before, Style::default().fg(DIM_WHITE)),
            Span::styled("▓▓▓▓▓▓▓▓▓", Style::default().fg(GOLD).add_modifier(Modifier::BOLD)),
            Span::styled(after, Style::default().fg(Color::Rgb(100, 95, 130))),
        ])
    };
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
    let cursor_blink = if (app.tick / 8) % 2 == 0 { "█" } else { " " };
    let input_display = format!(" {} {}", app.input, cursor_blink);
    f.render_widget(
        Paragraph::new(input_display)
            .style(Style::default().fg(Color::White))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(PURPLE))
                .style(Style::default().bg(Color::Rgb(30, 24, 54)))),
        chunks[3],
    );

    // Help
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("[Enter] submit  ", Style::default().fg(DIM_WHITE)),
            Span::styled("[Esc] back to menu", Style::default().fg(DIM_WHITE)),
        ])).alignment(Alignment::Center),
        chunks[4],
    );
}

// ── Result ────────────────────────────────────────────────────────────────────
fn draw_result(
    f: &mut Frame, app: &App, area: Rect,
    correct: bool, expected: &str, before: &str, after: &str,
) {
    let popup = centered_rect(70, 55, area);
    f.render_widget(Clear, popup);

    let (border_col, icon, verdict) = if correct {
        (GREEN,  "✓", "Correct!")
    } else {
        (RED, "✗", "Not quite")
    };

    // Full completed sentence
    let blank_marker = "___";
    let full_sentence: Line = if let Some(pos) = before.find(blank_marker) {
        let pre  = &before[..pos];
        let post = &before[pos + blank_marker.len()..];
        Line::from(vec![
            Span::styled(pre,      Style::default().fg(DIM_WHITE)),
            Span::styled(expected, Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(post,     Style::default().fg(DIM_WHITE)),
            Span::styled(after,    Style::default().fg(Color::Rgb(110, 105, 140))),
        ])
    } else {
        Line::from(vec![
            Span::styled(before,   Style::default().fg(DIM_WHITE)),
            Span::styled(expected, Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(after,    Style::default().fg(Color::Rgb(110, 105, 140))),
        ])
    };

    let score_pct = if app.session_total > 0 {
        app.session_correct * 100 / app.session_total
    } else { 0 };

    let content = vec![
        Line::from(vec![
            Span::styled(format!(" {icon}  "), Style::default().fg(border_col).add_modifier(Modifier::BOLD)),
            Span::styled(verdict, Style::default().fg(border_col).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("     {}/{} ({}%)", app.session_correct, app.session_total, score_pct),
                Style::default().fg(DIM_WHITE),
            ),
        ]),
        Line::from(""),
        if !correct {
            Line::from(vec![
                Span::styled(" Answer: ", Style::default().fg(DIM_WHITE)),
                Span::styled(expected, Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            ])
        } else {
            Line::from("")
        },
        Line::from(""),
        full_sentence,
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

// ── Summary ───────────────────────────────────────────────────────────────────
fn draw_summary(f: &mut Frame, app: &App, area: Rect) {
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);

    let pct = if app.session_total > 0 {
        app.session_correct * 100 / app.session_total
    } else { 0 };

    let (colour, msg) = match pct {
        100      => (GREEN,   "¡Perfecto! Flawless run. 🎉"),
        80..=99  => (GREEN,   "Muy bien — solid session!"),
        50..=79  => (GOLD,    "Keep going — repetition is the key."),
        _        => (RED,     "Rough one. Check the heatmap for weak spots."),
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
            Span::styled("[h] heatmap   ", Style::default().fg(DIM_WHITE)),
            Span::styled("[s] streak   ", Style::default().fg(DIM_WHITE)),
            Span::styled("[Esc] menu", Style::default().fg(DIM_WHITE)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(content)
            .block(Block::default()
                .title(Span::styled(" Session Complete ", Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(PURPLE))
                .style(Style::default().bg(PANEL_BG)))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false }),
        popup,
    );
}

// ── Heatmap ───────────────────────────────────────────────────────────────────
fn draw_heatmap(f: &mut Frame, app: &App, area: Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    // Title bar
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" ERROR HEATMAP ", Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)),
            Span::styled("  [Esc] menu   [r] drill   [s] streak",
                Style::default().fg(DIM_WHITE)),
        ])).block(Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(PANEL_BG))),
        outer[0],
    );

    let inner = centered_rect(90, 95, outer[1]);

    // Column widths: verb label + 4 tense columns
    let verb_w  = 16u16;
    let cell_w  = 14u16;
    let n_tense = data::TENSES.len() as u16;
    let total_w = verb_w + n_tense * cell_w;

    // Header row
    let mut header_spans = vec![Span::raw(format!("{:width$}", "", width = verb_w as usize))];
    for (i, &t) in data::TENSES.iter().enumerate() {
        let lbl = data::TENSE_LABELS[i];
        header_spans.push(Span::styled(
            format!("{:^width$}", lbl, width = cell_w as usize),
            Style::default().fg(DIM_WHITE).add_modifier(Modifier::BOLD),
        ));
    }

    let mut lines = vec![Line::from(""), Line::from(header_spans), Line::from("")];

    for &verb in data::VERBS {
        let mut row_spans = vec![
            Span::styled(
                format!("{:<width$}", verb, width = verb_w as usize),
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
            row_spans.push(Span::styled(label, Style::default().fg(fg).bg(bg)));
            row_spans.push(Span::raw(" ")); // gap
        }
        lines.push(Line::from(row_spans));
        lines.push(Line::from(""));
    }

    // Legend
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

// ── Streak calendar ───────────────────────────────────────────────────────────
fn draw_streak(f: &mut Frame, app: &App, area: Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" STREAK CALENDAR ", Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)),
            Span::styled("  [Esc] menu   [h] heatmap",
                Style::default().fg(DIM_WHITE)),
        ])).block(Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(PANEL_BG))),
        outer[0],
    );

    let inner = centered_rect(60, 80, outer[1]);

    let days = stats::last_n_days(28);
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("   Mo  Tu  We  Th  Fr  Sa  Su",
                Style::default().fg(DIM_WHITE).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
    ];

    // Build rows week by week
    // Find what day-of-week the first day is
    let first_dow = stats::dow(&days[0]); // 0=Mon
    let mut week_spans: Vec<Span> = vec![Span::raw("   ")];

    // Pad
    for _ in 0..first_dow {
        week_spans.push(Span::raw("    "));
    }

    for date in &days {
        let count = app.stats.streak.get(date).copied().unwrap_or(0);
        let (block, style) = if count == 0 {
            ("  ·  ", Style::default().fg(Color::Rgb(60, 55, 80)))
        } else if count < 10 {
            (" ▪▪  ", Style::default().fg(GOLD))
        } else if count < 25 {
            (" ██  ", Style::default().fg(GREEN))
        } else {
            (" ██  ", Style::default().fg(GREEN).add_modifier(Modifier::BOLD))
        };

        // Check if today
        let today = stats::today_str();
        let final_style = if date == &today {
            style.add_modifier(Modifier::UNDERLINED)
        } else { style };

        week_spans.push(Span::styled(block, final_style));

        // End of week (Sunday = dow 6)
        let cur_dow = stats::dow(date);
        if cur_dow == 6 {
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

    // Compute streak count
    let today = stats::today_str();
    let mut streak_days = 0u32;
    for d in stats::last_n_days(365).iter().rev() {
        if app.stats.streak.get(d).copied().unwrap_or(0) > 0 {
            streak_days += 1;
        } else if d != &today {
            break;
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled(
            format!("Current streak: {} day{}", streak_days, if streak_days == 1 { "" } else { "s" }),
            Style::default().fg(if streak_days > 0 { GREEN } else { DIM_WHITE })
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

// ── Layout helpers ────────────────────────────────────────────────────────────
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vert[1])[1]
}
