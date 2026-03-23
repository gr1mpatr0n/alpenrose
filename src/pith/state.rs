// SPDX-License-Identifier: Apache-2.0
//
// Application state — the Rust equivalent of Alpine's `struct pine` / ps_global.
//
// In Alpine, `ps_global` is a massive god-struct with ~300 fields carrying all
// application state including screen pointers, mail streams, UI flags, etc.
// We decompose this into focused sub-structs behind a single `AppState`.

use crate::config::Config;
use crate::screen::Screen;

/// The central application state, threaded through the main loop.
///
/// Mirrors Alpine's `struct pine` but decomposed into logical groups.
pub struct AppState {
    /// Active screen (analogous to Alpine's next_screen/prev_screen fn ptrs)
    pub current_screen: Screen,
    pub prev_screen: Screen,

    /// Runtime configuration
    pub config: Config,

    /// Terminal dimensions
    pub term: TermInfo,

    /// UI repaint flags — Alpine's mangled_* bitfields
    pub repaint: RepaintFlags,

    /// Version info
    pub version: &'static str,

    /// Whether we should quit
    pub quit: bool,

    /// Status messages to display in the footer
    pub status_messages: Vec<String>,
}

/// Terminal geometry, analogous to Alpine's `struct ttyo`.
#[derive(Debug, Clone)]
pub struct TermInfo {
    pub rows: u16,
    pub cols: u16,
}

/// Repaint flags — Alpine's mangled_header, mangled_footer, etc.
#[derive(Debug, Default)]
pub struct RepaintFlags {
    pub header: bool,
    pub footer: bool,
    pub body: bool,
    pub screen: bool,
}

impl RepaintFlags {
    pub fn all(&mut self) {
        self.header = true;
        self.footer = true;
        self.body = true;
        self.screen = true;
    }

    pub fn clear(&mut self) {
        self.header = false;
        self.footer = false;
        self.body = false;
        self.screen = false;
    }
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));

        let mut state = Self {
            current_screen: Screen::Main,
            prev_screen: Screen::Main,
            config,
            term: TermInfo { rows, cols },
            repaint: RepaintFlags::default(),
            version: env!("CARGO_PKG_VERSION"),
            quit: false,
            status_messages: Vec::new(),
        };
        state.repaint.all();
        state
    }

    /// Push a status message (displayed in footer, like Alpine's q_status_message).
    pub fn status_message(&mut self, msg: impl Into<String>) {
        self.status_messages.push(msg.into());
    }

    /// Drain status messages for display.
    pub fn drain_status(&mut self) -> Vec<String> {
        std::mem::take(&mut self.status_messages)
    }

    /// Navigate to a new screen (mimics Alpine's next_screen pattern).
    pub fn goto_screen(&mut self, screen: Screen) {
        self.prev_screen = self.current_screen;
        self.current_screen = screen;
        self.repaint.all();
    }
}
