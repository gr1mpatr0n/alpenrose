# Alpenrose

A Rust port of the [Alpine](https://alpineapp.email/) email client.

Alpine is a venerable text-based email client descended from Pine, originally
developed at the University of Washington and currently maintained by Eduardo
Chappa. Alpenrose aims to faithfully reproduce Alpine's keyboard-driven UI
and workflow while leveraging Rust's safety, performance, and modern ecosystem.

## Status

**Proof of Concept** — the TUI shell is functional with:

- Main menu screen with Alpine-style layout and key accelerators
- Message index screen (placeholder, pending IMAP backend)
- Help screen with context-sensitive documentation
- Screen navigation matching Alpine's `next_screen`/`prev_screen` pattern
- TOML-based configuration (with planned legacy `pinerc` import)
- Placeholder screens for Compose, Folder List, Address Book, Setup

## Architecture

Alpenrose's module structure mirrors Alpine's source tree:

| Alpenrose       | Alpine equivalent         | Purpose                              |
|-----------------|---------------------------|--------------------------------------|
| `src/main.rs`   | `alpine/alpine.c::main()` | Entry point, arg parsing, init       |
| `src/tui/`      | `alpine/alpine.c` loop    | Terminal management & event loop     |
| `src/screen/`   | `alpine/*.c` screens      | Screen rendering (main menu, etc.)   |
| `src/config/`   | `pith/conf.c`             | Configuration loading                |
| `src/pith/`     | `pith/*.c`                | Core logic, state management         |
| `src/mail/`     | `imap/` (c-client)        | Mail protocol backends               |

## Building

```sh
cargo build
cargo run
```

## Configuration

Copy `alpenrose.toml.example` to `~/.alpenrose.toml` and edit to
match your mail server settings.

## Key Bindings (Main Menu)

| Key   | Action              |
|-------|---------------------|
| `?`   | Help                |
| `C`   | Compose message     |
| `I`   | Message index       |
| `L`   | Folder list         |
| `A`   | Address book        |
| `S`   | Setup               |
| `Q`   | Quit                |
| Up/Dn | Navigate menu       |
| Enter | Select item         |

## Roadmap

1. **IMAP INBOX listing** — connect to a real server, fetch envelopes
2. **Message viewing** — render message bodies with MIME handling
3. **Compose & send** — built-in editor + SMTP via `lettre`
4. **Address book** — local + LDAP support
5. **Folder management** — create, rename, delete
6. **Legacy pinerc import** — migrate existing Alpine configurations
7. **Local mailbox formats** — mbox, Maildir, MIX
8. **S/MIME & PGP** — encrypted/signed message support

## License

Apache-2.0, matching the original Alpine license.
