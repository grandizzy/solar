//! Source positions and related helper functions.
//!
//! Important concepts in this module include:
//!
//! - the *span*, represented by [`Span`] and related types;
//! - source code as represented by a [`SourceMap`]; and
//! - interned strings, represented by [`Symbol`]s, with some common symbols available statically in
//!   the [`sym`] module.
//!
//! ## Note
//!
//! This API is completely unstable and subject to change.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/danipopes/sulk/main/assets/logo.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/danipopes/sulk/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(min_specialization))]

pub mod diagnostics;
mod globals;
mod pos;
mod span;
mod symbol;

pub use globals::*;
pub use pos::{BytePos, CharPos, Pos};
pub use span::Span;
pub use symbol::{kw, sym, Ident, Symbol};

/// TODO
pub struct SourceMap {}
