//! Chui: UI

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
mod macros;

pub use chui_core::Coord;
pub use chui_error::{ChuiError, ChuiResult};

pub mod ui;
pub use ui::Ui;
