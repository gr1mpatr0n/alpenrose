// SPDX-License-Identifier: Apache-2.0
//
// Alpenrose - A Rust port of the Alpine email client
// Error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlpenroseError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("Mail error: {0}")]
    Mail(String),

    #[error(transparent)]
    Other(#[from] color_eyre::eyre::Error),
}

pub type Result<T> = std::result::Result<T, AlpenroseError>;
