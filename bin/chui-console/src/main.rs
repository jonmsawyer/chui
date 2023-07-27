//! Chui: Chess UI
//!
//! Console application.

use chui_error::ChuiResult;

mod console;

// When Chui is run as a command line application:
//  * On Windows, the `DejaVu Sans Mono` font should be used
//    in your terminal. Size 18 font looks decent.
fn main() -> ChuiResult<()> {
    console::run()
}
