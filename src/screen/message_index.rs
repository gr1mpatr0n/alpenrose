// SPDX-License-Identifier: Apache-2.0
//
// Message Index Screen — Alpine's mail_index_screen().
//
// Displays the list of messages in the current folder. For the PoC this
// shows a placeholder; the real implementation will use the mail backend.

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::pith::state::AppState;

/// Render the message index screen.
pub fn render_message_index(f: &mut Frame, state: &AppState) {
    let area = f.area();

    let chunks = Layout::vertical([
        Constraint::Length(1), // titlebar
        Constraint::Min(0),   // index body
        Constraint::Length(1), // status
        Constraint::Length(1), // keymenu
    ])
    .split(area);

    // ── Titlebar ───────────────────────────────────────────────────
    let title = format!(
        " ALPENROSE {} MESSAGE INDEX  Strstr Folder: INBOX  Strstr Strstr  No Messages",
        state.version,
    );
    let titlebar = Paragraph::new(title).style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(titlebar, chunks[0]);

    // ── Body ───────────────────────────────────────────────────────
    let body = Paragraph::new(vec![
        Line::from(""),
        Line::from("   [Message index will appear here once IMAP backend is connected]"),
        Line::from(""),
        Line::from("   This screen corresponds to Alpine's mail_index_screen() function."),
        Line::from("   It will display messages from the current folder in a columnar format:"),
        Line::from(""),
        Line::from("     STATUS  NUM   DATE         FROM                 SIZE  SUBJECT"),
        Line::from("     ──────  ───   ──────────   ──────────────────   ────  ───────────"),
        Line::from("       N  +    1   Mar 23       user@example.com     2KB   Welcome to Alpenrose"),
        Line::from(""),
        Line::from("   Press '<' or 'M' to return to the main menu."),
    ])
    .style(Style::default().fg(Color::White));
    f.render_widget(body, chunks[1]);

    // ── Keymenu ────────────────────────────────────────────────────
    let keys = vec![
        Span::styled(
            " ? ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Help "),
        Span::styled(
            "M ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Main Menu "),
        Span::styled(
            "< ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("FldrList "),
        Span::styled(
            "V ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("ViewMsg"),
    ];

    let keymenu = Paragraph::new(Line::from(keys)).style(
        Style::default()
            .fg(Color::DarkGray)
            .bg(Color::Black),
    );
    f.render_widget(keymenu, chunks[3]);
}
