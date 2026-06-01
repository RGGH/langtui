use crossterm::event::KeyCode;
use crate::{app::{App, Screen}, stats};

pub fn handle_menu(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Char('1') | KeyCode::Char(' ') => {
            app.build_questions();
            app.screen = Screen::Drill;
        }
        KeyCode::Char('2') | KeyCode::Char('h') => app.screen = Screen::Heatmap,
        KeyCode::Char('3') | KeyCode::Char('s') => app.screen = Screen::Streak,
        _ => {}
    }
}

pub fn handle_drill(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char(c)   => app.input.push(c),
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

pub fn handle_result(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Char(' ') => app.advance(),
        KeyCode::Esc => {
            stats::save(&app.stats);
            app.screen = Screen::Menu;
        }
        _ => {}
    }
}

pub fn handle_summary(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Char('r') => {
            app.build_questions();
            app.screen = Screen::Drill;
        }
        KeyCode::Char('h')              => app.screen = Screen::Heatmap,
        KeyCode::Char('s')              => app.screen = Screen::Streak,
        KeyCode::Esc | KeyCode::Char('q') => app.screen = Screen::Menu,
        _ => {}
    }
}

pub fn handle_heatmap(app: &mut App, key: KeyCode) {
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

pub fn handle_streak(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Backspace => app.screen = Screen::Menu,
        KeyCode::Char('h') => app.screen = Screen::Heatmap,
        _ => {}
    }
}