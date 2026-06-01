mod app;
mod data;
mod handlers;
mod stats;
mod ui;

use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use app::Screen;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--reset") {
        let path = stats::stats_path();
        if path.exists() { std::fs::remove_file(&path)?; println!("Stats wiped."); }
        else              { println!("No stats file found."); }
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = app::App::new();
    app.screen = if args.iter().any(|a| a == "--stats") {
        Screen::Heatmap
    } else {
        Screen::Menu
    };

    loop {
        app.tick = app.tick.wrapping_add(1);
        term.draw(|f| ui::render(f, &app))?;

        if !event::poll(std::time::Duration::from_millis(50))? { continue; }
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press { continue; }

        if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
            break;
        }

        match app.screen.clone() {
            Screen::Menu    => handlers::handle_menu(&mut app, key.code),
            Screen::Drill   => handlers::handle_drill(&mut app, key.code),
            Screen::Result { .. } => handlers::handle_result(&mut app, key.code),
            Screen::Summary => handlers::handle_summary(&mut app, key.code),
            Screen::Heatmap => handlers::handle_heatmap(&mut app, key.code),
            Screen::Streak  => handlers::handle_streak(&mut app, key.code),
        }

        if matches!(app.screen, Screen::Menu) && key.code == KeyCode::Char('q') {
            break;
        }
    }

    stats::save(&app.stats);
    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}