// SPDX-License-Identifier: Apache-2.0
//
// TUI management — terminal init/teardown and the main render+event loop.
//
// Alpine's equivalent is spread across alpine/alpine.c (the main while(1) loop),
// osdep/termout.*, and the raw terminal handling in pico/osdep/raw.c.
// We consolidate this into a clean ratatui-based loop.

use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::error::Result;
use crate::pith::state::AppState;
use crate::screen::Screen;
use crate::screen::help::render_help;
use crate::screen::main_menu::{MainMenuState, render_main_menu};
use crate::screen::message_index::render_message_index;

/// Initialize the terminal for TUI rendering.
///
/// Equivalent to Alpine's raw_mode setup in pico/osdep/raw.c
pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to its normal state.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// The main application loop.
///
/// This mirrors the `while(1)` loop in Alpine's main_menu_screen():
///   1. Check for new mail
///   2. Render the current screen
///   3. Read a command from the keyboard
///   4. Dispatch the command
///   5. Possibly switch screens
pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> Result<()> {
    let mut menu_state = MainMenuState::default();

    state.status_message("Welcome to Alpenrose. Press ? for help.");

    loop {
        // ── Render current screen ──────────────────────────────────
        terminal.draw(|f| {
            // Update terminal dimensions (Alpine does this via SIGWINCH)
            let size = f.area();
            state.term.rows = size.height;
            state.term.cols = size.width;

            match state.current_screen {
                Screen::Main => render_main_menu(f, state, &menu_state),
                Screen::MessageIndex => render_message_index(f, state),
                Screen::Help => render_help(f, state),
                _ => render_placeholder(f, state),
            }
        })?;

        state.repaint.clear();

        // ── Read and dispatch keyboard input ───────────────────────
        // Alpine's READ_COMMAND macro, which calls ttyin() / read_char()
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not release/repeat)
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Ctrl-C always quits (Alpine's emergency exit)
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('c')
                {
                    break;
                }

                match state.current_screen {
                    Screen::Main => {
                        handle_main_menu_input(state, &mut menu_state, key.code);
                    }
                    Screen::MessageIndex => {
                        handle_message_index_input(state, key.code);
                    }
                    Screen::Help => {
                        handle_help_input(state, key.code);
                    }
                    Screen::Quit => break,
                    _ => {
                        // Placeholder screens: any key returns to main
                        handle_placeholder_input(state, key.code);
                    }
                }

                if state.quit {
                    break;
                }
            }
        }
    }

    Ok(())
}

/// Handle keyboard input on the main menu.
///
/// Mirrors Alpine's massive switch/case in main_menu_screen() that dispatches
/// on the command enum (MC_HELP, MC_COMPOSE, MC_INDEX, etc.)
fn handle_main_menu_input(state: &mut AppState, menu: &mut MainMenuState, key: KeyCode) {
    match key {
        // Arrow key navigation
        KeyCode::Up | KeyCode::Char('p') | KeyCode::Char('P') => menu.up(),
        KeyCode::Down | KeyCode::Char('n') | KeyCode::Char('N') => menu.down(),

        // Enter selects the current item
        KeyCode::Enter => {
            let item = menu.selected_item();
            if let Some(screen) = item.target_screen() {
                if screen == Screen::Quit {
                    state.quit = true;
                } else {
                    state.goto_screen(screen);
                }
            }
        }

        // Direct key accelerators (Alpine's primary input method)
        KeyCode::Char('?') => state.goto_screen(Screen::Help),
        KeyCode::Char('c') | KeyCode::Char('C') => state.goto_screen(Screen::Compose),
        KeyCode::Char('i') | KeyCode::Char('I') => state.goto_screen(Screen::MessageIndex),
        KeyCode::Char('l') | KeyCode::Char('L') => state.goto_screen(Screen::FolderList),
        KeyCode::Char('a') | KeyCode::Char('A') => state.goto_screen(Screen::AddressBook),
        KeyCode::Char('s') | KeyCode::Char('S') => state.goto_screen(Screen::Setup),
        KeyCode::Char('q') | KeyCode::Char('Q') => state.quit = true,

        _ => {}
    }
}

fn handle_message_index_input(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Char('<') | KeyCode::Char('m') | KeyCode::Char('M') => {
            state.goto_screen(Screen::Main);
        }
        KeyCode::Char('?') => state.goto_screen(Screen::Help),
        KeyCode::Char('q') | KeyCode::Char('Q') => state.quit = true,
        _ => {}
    }
}

fn handle_help_input(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char('<') | KeyCode::Esc => {
            state.goto_screen(state.prev_screen);
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => state.quit = true,
        _ => {}
    }
}

fn handle_placeholder_input(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Char('<') | KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Esc => {
            state.goto_screen(Screen::Main);
        }
        KeyCode::Char('?') => state.goto_screen(Screen::Help),
        KeyCode::Char('q') | KeyCode::Char('Q') => state.quit = true,
        _ => {}
    }
}

/// Render a placeholder screen for unimplemented features.
fn render_placeholder(f: &mut ratatui::Frame, state: &AppState) {
    use ratatui::{
        layout::{Constraint, Layout},
        style::{Color, Modifier, Style},
        text::Line,
        widgets::Paragraph,
    };

    let area = f.area();
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(area);

    let screen_name = format!("{:?}", state.current_screen);
    let title = format!(" ALPENROSE {} {}", state.version, screen_name.to_uppercase());
    let titlebar = Paragraph::new(title).style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(titlebar, chunks[0]);

    let body = Paragraph::new(vec![
        Line::from(""),
        Line::from(format!("   [{} screen - not yet implemented]", screen_name)),
        Line::from(""),
        Line::from("   Press '<' or 'M' to return to the main menu."),
    ])
    .style(Style::default().fg(Color::White));
    f.render_widget(body, chunks[1]);

    let keymenu = Paragraph::new(" < Back  M Main Menu  ? Help  Q Quit").style(
        Style::default()
            .fg(Color::DarkGray)
            .bg(Color::Black),
    );
    f.render_widget(keymenu, chunks[2]);
}
