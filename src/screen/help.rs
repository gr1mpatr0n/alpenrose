// SPDX-License-Identifier: Apache-2.0
//
// Help Screen — Alpine's helper() / help_text().
//
// Alpine has extensive context-sensitive help built into every screen.
// For the PoC we show a general help overview.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::pith::state::AppState;

pub fn render_help(f: &mut Frame, state: &AppState) {
    let area = f.area();

    let chunks = Layout::vertical([
        Constraint::Length(1), // titlebar
        Constraint::Min(0),   // help body
        Constraint::Length(1), // keymenu
    ])
    .split(area);

    // ── Titlebar ───────────────────────────────────────────────────
    let title = format!(" ALPENROSE {} HELP", state.version);
    let titlebar = Paragraph::new(title).style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(titlebar, chunks[0]);

    // ── Help text ──────────────────────────────────────────────────
    let help_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Alpenrose — A Rust port of the Alpine email client",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Alpenrose is a modern reimplementation of the Alpine email client"),
        Line::from("  in Rust, preserving Alpine's intuitive keyboard-driven interface."),
        Line::from(""),
        Line::from(Span::styled(
            "  Main Menu Commands:",
            Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(""),
        Line::from("    ?         This help screen"),
        Line::from("    C         Compose a new message"),
        Line::from("    I         Go to Message Index for current folder"),
        Line::from("    L         List folders for selection"),
        Line::from("    A         Open the Address Book"),
        Line::from("    S         Setup / configuration"),
        Line::from("    Q         Quit Alpenrose"),
        Line::from(""),
        Line::from(Span::styled(
            "  Navigation:",
            Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(""),
        Line::from("    Up/Down   Move between menu items"),
        Line::from("    Enter     Select highlighted item"),
        Line::from("    <         Go back to previous screen"),
        Line::from("    M         Return to Main Menu from any screen"),
        Line::from(""),
        Line::from("  Press 'E' or '<' to exit help."),
    ];

    let body = Paragraph::new(help_lines)
        .style(Style::default().fg(Color::White));
    f.render_widget(body, chunks[1]);

    // ── Keymenu ────────────────────────────────────────────────────
    let keys = vec![
        Span::styled(
            " E ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Exit Help "),
        Span::styled(
            "< ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Back "),
    ];

    let keymenu = Paragraph::new(Line::from(keys)).style(
        Style::default()
            .fg(Color::DarkGray)
            .bg(Color::Black),
    );
    f.render_widget(keymenu, chunks[2]);
}
