// SPDX-License-Identifier: Apache-2.0
//
// Main Menu Screen — Alpine's main_menu_screen().
//
// This is the first screen users see. Alpine renders it as a centered menu
// with single-key accelerators. We replicate that layout with ratatui.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::pith::state::AppState;
use crate::screen::Screen;

/// Menu items corresponding to Alpine's main menu.
/// Alpine displays these as single-character commands in a centered block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuItem {
    Help,
    Compose,
    MessageIndex,
    FolderList,
    AddressBook,
    Setup,
    Quit,
}

impl MainMenuItem {
    pub const ALL: &[MainMenuItem] = &[
        Self::Help,
        Self::Compose,
        Self::MessageIndex,
        Self::FolderList,
        Self::AddressBook,
        Self::Setup,
        Self::Quit,
    ];

    pub fn key(&self) -> char {
        match self {
            Self::Help => '?',
            Self::Compose => 'C',
            Self::MessageIndex => 'I',
            Self::FolderList => 'L',
            Self::AddressBook => 'A',
            Self::Setup => 'S',
            Self::Quit => 'Q',
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Help => "HELP",
            Self::Compose => "COMPOSE MESSAGE",
            Self::MessageIndex => "MESSAGE INDEX",
            Self::FolderList => "FOLDER LIST",
            Self::AddressBook => "ADDRESS BOOK",
            Self::Setup => "SETUP",
            Self::Quit => "QUIT",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Help => "Get help using Alpine",
            Self::Compose => "Compose and send a message",
            Self::MessageIndex => "View messages in current folder",
            Self::FolderList => "Select a folder to view",
            Self::AddressBook => "Update address book",
            Self::Setup => "Configure Alpine Options",
            Self::Quit => "Leave the Alpine program",
        }
    }

    /// Map to the screen this item navigates to.
    pub fn target_screen(&self) -> Option<Screen> {
        match self {
            Self::Help => Some(Screen::Help),
            Self::Compose => Some(Screen::Compose),
            Self::MessageIndex => Some(Screen::MessageIndex),
            Self::FolderList => Some(Screen::FolderList),
            Self::AddressBook => Some(Screen::AddressBook),
            Self::Setup => Some(Screen::Setup),
            Self::Quit => Some(Screen::Quit),
        }
    }
}

/// Index of the currently highlighted menu item.
pub struct MainMenuState {
    pub selected: usize,
}

impl Default for MainMenuState {
    fn default() -> Self {
        // Alpine defaults to MESSAGE INDEX selected
        Self { selected: 2 }
    }
}

impl MainMenuState {
    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.selected < MainMenuItem::ALL.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn selected_item(&self) -> MainMenuItem {
        MainMenuItem::ALL[self.selected]
    }
}

/// Render the main menu screen.
///
/// Layout mirrors Alpine's original:
/// ```text
///  +-- ALPINE 2.26 -- MAIN MENU ----------------------------+
///  │                                                                    │
///  │         ?     HELP               - Get help using Alpine           │
///  │         C     COMPOSE MESSAGE    - Compose and send a message      │
///  │       > I     MESSAGE INDEX      - View messages in current folder │
///  │         L     FOLDER LIST        - Select a folder to view         │
///  │         A     ADDRESS BOOK       - Update address book             │
///  │         S     SETUP              - Configure Alpine Options        │
///  │         Q     QUIT               - Leave the Alpine program        │
///  │                                                                    │
///  └────────────────────────────────────────────────────────────────────┘
/// ```
pub fn render_main_menu(f: &mut Frame, state: &AppState, menu_state: &MainMenuState) {
    let area = f.area();

    // Split into: titlebar (1) | blank (1) | body | footer (2)
    let chunks = Layout::vertical([
        Constraint::Length(1), // titlebar
        Constraint::Length(1), // blank line below titlebar
        Constraint::Min(0),   // body
        Constraint::Length(1), // status line
        Constraint::Length(1), // key menu
    ])
    .split(area);

    // ── Titlebar ───────────────────────────────────────────────────
    render_titlebar(f, chunks[0], state);

    // ── Menu body ──────────────────────────────────────────────────
    render_menu_body(f, chunks[2], menu_state);

    // ── Status line ────────────────────────────────────────────────
    let status_msgs = &state.status_messages;
    let status_text = if let Some(msg) = status_msgs.last() {
        msg.as_str()
    } else {
        ""
    };
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    f.render_widget(status, chunks[3]);

    // ── Key menu (footer) ──────────────────────────────────────────
    render_keymenu(f, chunks[4]);
}

fn render_titlebar(f: &mut Frame, area: Rect, state: &AppState) {
    // Alpine's titlebar: reverse video, shows version + screen name + folder
    let title = format!(
        " ALPENROSE {} MAIN MENU  Folder: INBOX  {} Messages",
        state.version,
        0, // placeholder message count
    );

    let titlebar = Paragraph::new(title).style(
        Style::default()
            .fg(Color::Black)
            .bg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(titlebar, area);
}

fn render_menu_body(f: &mut Frame, area: Rect, menu_state: &MainMenuState) {
    // Center the menu vertically
    let menu_height = MainMenuItem::ALL.len() as u16;
    let v_pad = area.height.saturating_sub(menu_height + 2) / 2;

    let body_chunks = Layout::vertical([
        Constraint::Length(v_pad),
        Constraint::Length(menu_height + 2),
        Constraint::Min(0),
    ])
    .split(area);

    let menu_area = body_chunks[1];

    // Center horizontally — Alpine's menu items are left-aligned within a
    // centered block. We target ~60 columns.
    let menu_width = 64u16.min(menu_area.width);
    let h_pad = menu_area.width.saturating_sub(menu_width) / 2;
    let centered = Rect {
        x: menu_area.x + h_pad,
        y: menu_area.y,
        width: menu_width,
        height: menu_area.height,
    };

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for (i, item) in MainMenuItem::ALL.iter().enumerate() {
        let is_selected = i == menu_state.selected;

        let marker = if is_selected { "  > " } else { "    " };
        let key_str = format!("{}     ", item.key());
        let label = format!("{:<20}", item.label());
        let desc = format!("- {}", item.description());

        let style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let key_style = if is_selected {
            style
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        };

        lines.push(Line::from(vec![
            Span::styled(marker, style),
            Span::styled(key_str, key_style),
            Span::styled(label, style),
            Span::styled(desc, style),
        ]));
    }

    lines.push(Line::from(""));

    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::Black));

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, centered);
}

fn render_keymenu(f: &mut Frame, area: Rect) {
    // Alpine's bottom key menu bar — reversed video, shows available commands
    let keys = vec![
        Span::styled(
            " ? ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Help "),
        Span::styled(
            "O ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("OTHER CMDS "),
        Span::styled(
            "Q ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Quit "),
        Span::styled(
            "C ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Compose "),
        Span::styled(
            "L ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("ListFldrs "),
        Span::styled(
            "I ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Index"),
    ];

    let keymenu = Paragraph::new(Line::from(keys)).style(
        Style::default()
            .fg(Color::DarkGray)
            .bg(Color::Black),
    );
    f.render_widget(keymenu, area);
}
