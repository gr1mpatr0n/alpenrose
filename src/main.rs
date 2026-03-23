// SPDX-License-Identifier: Apache-2.0
//
// Alpenrose — A Rust port of the Alpine email client.
//
// Architecture overview:
//
//   ┌─────────────────────────────────────────────────────────┐
//   │ main.rs         Entry point, arg parsing, init          │
//   │                 (~ alpine/alpine.c::main)               │
//   ├─────────────────────────────────────────────────────────┤
//   │ tui/            Terminal UI management & event loop      │
//   │                 (~ alpine/alpine.c main loop +           │
//   │                    osdep/termout + pico/osdep/raw)       │
//   ├─────────────────────────────────────────────────────────┤
//   │ screen/         Screen rendering (main_menu, index, etc)│
//   │                 (~ alpine/*.c screen functions)          │
//   ├─────────────────────────────────────────────────────────┤
//   │ config/         Configuration loading & pinerc compat   │
//   │                 (~ pith/conf.c + pith/init.c)           │
//   ├─────────────────────────────────────────────────────────┤
//   │ pith/           Core logic: state, mail ops, non-UI     │
//   │                 (~ pith/*.c)                             │
//   ├─────────────────────────────────────────────────────────┤
//   │ mail/           IMAP/SMTP backends                      │
//   │                 (~ imap/ c-client library)              │
//   └─────────────────────────────────────────────────────────┘

// Allow dead code in the PoC — many types exist for the API surface but
// aren't exercised yet.
#![allow(dead_code)]

mod config;
mod error;
mod mail;
mod pith;
mod screen;
mod tui;

use std::path::PathBuf;

use crate::config::Config;
use crate::error::Result;
use crate::pith::state::AppState;

fn main() -> Result<()> {
    // Initialize logging (RUST_LOG=debug for verbose output)
    env_logger::init();

    // Parse command-line arguments
    // Alpine supports a rich set of CLI args (-f folder, -i start_in_index, etc.)
    // For the PoC we support just -c <config> and -h.
    let args = parse_args();

    // Load configuration
    let config_path = args.config_file.unwrap_or_else(Config::default_path);
    let config = Config::load(&config_path)?;

    log::info!("Starting Alpenrose v{}", env!("CARGO_PKG_VERSION"));

    // Build application state (~ new_pine_struct() in Alpine)
    let mut state = AppState::new(config);

    // Initialize terminal
    let mut terminal = tui::init_terminal()?;

    // Run the main event loop (~ the while(1) in alpine.c::main)
    let result = tui::run_app(&mut terminal, &mut state);

    // Always restore terminal, even on error
    tui::restore_terminal(&mut terminal)?;

    // Print farewell (Alpine does this in goodnight_gracey())
    if result.is_ok() {
        println!("\n  Alpenrose finished.");
    }

    result
}

struct Args {
    config_file: Option<PathBuf>,
}

fn parse_args() -> Args {
    let mut args = Args {
        config_file: None,
    };

    let mut argv = std::env::args().skip(1);
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "-c" | "--config" => {
                args.config_file = argv.next().map(PathBuf::from);
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "-v" | "--version" => {
                println!("Alpenrose {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown option: {other}");
                eprintln!("Try 'alpenrose --help' for more information.");
                std::process::exit(1);
            }
        }
    }

    args
}

fn print_usage() {
    println!(
        "Alpenrose {} -- A Rust port of the Alpine email client\n\
         \n\
         Usage: alpenrose [OPTIONS]\n\
         \n\
         Options:\n\
         \x20 -c, --config <FILE>  Use specified config file\n\
         \x20                      (default: ~/.alpenrose.toml)\n\
         \x20 -h, --help           Show this help message\n\
         \x20 -v, --version        Show version\n\
         \n\
         Alpine-compatible options (planned):\n\
         \x20 -f <folder>          Open named folder on startup\n\
         \x20 -i                   Start in message index (skip main menu)\n\
         \x20 -n <number>          Start on message number\n\
         \x20 -p <pinerc>          Import legacy Alpine/Pine config file\n",
        env!("CARGO_PKG_VERSION")
    );
}
