// SPDX-License-Identifier: Apache-2.0
//
// Screen definitions — Alpine uses function pointers (next_screen, prev_screen)
// to implement a screen state machine. We use an enum instead.

pub mod help;
pub mod main_menu;
pub mod message_index;

/// All screens in the application.
///
/// Alpine implements this as function pointers stored in `struct pine`:
///   pine_state->next_screen = main_menu_screen;
///   pine_state->next_screen = mail_index_screen;
/// We use an enum + match dispatch instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// Main menu — the first thing you see (Alpine's main_menu_screen)
    Main,
    /// Message index for a folder (Alpine's mail_index_screen)
    MessageIndex,
    /// Help screen (Alpine's helper)
    Help,
    /// Compose a new message
    Compose,
    /// Folder list / selection
    FolderList,
    /// Address book
    AddressBook,
    /// Setup / configuration
    Setup,
    /// Quit confirmation
    Quit,
}
