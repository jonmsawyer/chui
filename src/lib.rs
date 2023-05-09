//! The main library for Chui.
//!
//! The application is defined in a library, rather than directly in the binary source,
//! in part so that tests can be more easily run against it. Writing it as a library
//! also gives us the ability to easily create other binaries that run parts of it
//! here.

pub use chui_core::{Color, Engine, ParserEngine, Player};
pub use chui_ui::Ui;
