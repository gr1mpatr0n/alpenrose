// SPDX-License-Identifier: Apache-2.0
//
// Configuration management - the Rust equivalent of Alpine's pinerc.
//
// Alpine uses a flat key=value config file (~/.pinerc). We use TOML for the
// native format but can import legacy pinerc files for migration.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::{Result, AlpenroseError};

/// Top-level configuration, analogous to Alpine's `struct pine` config fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// User's personal name for outgoing mail
    pub personal_name: String,

    /// User's email address(es)
    pub user_domain: String,

    /// SMTP server for sending mail
    pub smtp_server: Option<SmtpConfig>,

    /// Incoming mail folders (INBOX, plus user-defined)
    pub inbox: InboxConfig,

    /// Additional incoming folder collections
    #[serde(default)]
    pub folder_collections: Vec<FolderCollection>,

    /// Editor to use for composing (default: built-in pico-equivalent)
    pub editor: Option<String>,

    /// Display preferences
    pub display: DisplayConfig,

    /// Signature file path
    pub signature_file: Option<PathBuf>,

    /// Feature flags, analogous to Alpine's feature-list
    #[serde(default)]
    pub features: Features,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub use_tls: bool,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InboxConfig {
    /// IMAP server for INBOX
    pub server: String,
    pub port: u16,
    pub use_tls: bool,
    pub username: Option<String>,
    /// IMAP mailbox name to open
    pub mailbox: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderCollection {
    pub nickname: String,
    pub server: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplayConfig {
    /// Sort order for message index (analogous to sort-key)
    pub sort_key: SortKey,
    pub sort_reverse: bool,
    /// Threading style
    pub threading: ThreadStyle,
    /// Number of lines to use for index
    pub index_format: String,
    /// Color configuration
    pub use_colors: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Features {
    /// Enable-full-header-cmd
    pub enable_full_header: bool,
    /// Enable-msg-view-urls
    pub enable_msg_view_urls: bool,
    /// Enable-msg-view-attachments
    pub enable_msg_view_attachments: bool,
    /// Quit-without-confirm
    pub quit_without_confirm: bool,
    /// Enable-bounce-cmd
    pub enable_bounce_cmd: bool,
    /// Enable-flag-cmd
    pub enable_flag_cmd: bool,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum SortKey {
    #[default]
    Arrival,
    Date,
    From,
    To,
    Subject,
    Size,
    Thread,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum ThreadStyle {
    #[default]
    None,
    MuttLike,
    IndentSubject1,
    IndentSubject2,
    IndentFromSubject,
}

// ── Defaults ────────────────────────────────────────────────────────

impl Default for Config {
    fn default() -> Self {
        Self {
            personal_name: String::new(),
            user_domain: String::new(),
            smtp_server: None,
            inbox: InboxConfig::default(),
            folder_collections: Vec::new(),
            editor: None,
            display: DisplayConfig::default(),
            signature_file: None,
            features: Features::default(),
        }
    }
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            server: String::new(),
            port: 587,
            use_tls: true,
            username: None,
        }
    }
}

impl Default for InboxConfig {
    fn default() -> Self {
        Self {
            server: String::new(),
            port: 993,
            use_tls: true,
            username: None,
            mailbox: "INBOX".to_string(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            sort_key: SortKey::Arrival,
            sort_reverse: false,
            threading: ThreadStyle::None,
            index_format: "STATUS MSGNO DATE FROM SIZE SUBJECT"
                .to_string(),
            use_colors: true,
        }
    }
}

// ── Config file discovery & loading ────────────────────────────────

impl Config {
    /// Returns the default config file path: ~/.alpenrose.toml
    pub fn default_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".alpenrose.toml")
    }

    /// Load config from a TOML file, falling back to defaults.
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&contents)?;
            log::info!("Loaded config from {}", path.display());
            Ok(config)
        } else {
            log::info!(
                "No config at {}, using defaults",
                path.display()
            );
            Ok(Config::default())
        }
    }

    /// Save config to a TOML file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| AlpenroseError::Config(e.to_string()))?;
        std::fs::write(path, contents)?;
        log::info!("Saved config to {}", path.display());
        Ok(())
    }
}
