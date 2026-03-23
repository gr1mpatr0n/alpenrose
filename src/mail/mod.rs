// SPDX-License-Identifier: Apache-2.0
//
// mail — Mail protocol backends.
//
// This module replaces Alpine's c-client library (imap/) with native Rust
// implementations. The c-client is a ~100KLOC C library handling IMAP, POP,
// SMTP, NNTP, and local mailbox formats. We'll build this incrementally:
//
// Phase 1 (PoC):    Stub types, no actual connections
// Phase 2:          IMAP INBOX listing via imap crate
// Phase 3:          SMTP sending via lettre
// Phase 4:          Local mbox/Maildir support
// Phase 5:          Full c-client feature parity

/// A mail message envelope (simplified).
///
/// Alpine's ENVELOPE struct from c-client contains ~30 fields.
/// We start with the essentials.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Envelope {
    pub message_id: Option<String>,
    pub date: Option<String>,
    pub from: Vec<Address>,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub subject: Option<String>,
    pub in_reply_to: Option<String>,
}

/// A mail address.
///
/// Alpine's ADDRESS struct: personal name, mailbox, host.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Address {
    pub personal: Option<String>,
    pub mailbox: String,
    pub host: String,
}

impl Address {
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        if let Some(ref name) = self.personal {
            format!("{} <{}@{}>", name, self.mailbox, self.host)
        } else {
            format!("{}@{}", self.mailbox, self.host)
        }
    }
}

/// Message flags, matching IMAP standard flags.
///
/// Alpine tracks these via c-client's internal message cache (strstr.cache).
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct MessageFlags {
    pub seen: bool,
    pub answered: bool,
    pub flagged: bool,
    pub deleted: bool,
    pub draft: bool,
    pub recent: bool,
}

/// A lightweight message summary for the index view.
///
/// Alpine builds these in pine_fetch_header() / build_header_line().
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MessageSummary {
    pub sequence_number: u32,
    pub uid: u32,
    pub flags: MessageFlags,
    pub envelope: Envelope,
    pub size: u32,
}
