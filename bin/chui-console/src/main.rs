//! Chui: Chess UI
//!
//! Console application.

use chui_core::ChuiResult;

mod console;
use console::Console;

// When Chui is run as a command line application:
//  * On Windows, the `DejaVu Sans Mono` font should be used
//    in your terminal. Size 18 font looks decent.
fn main() -> ChuiResult<()> {
    let mut console = Console::new();
    console.run()
}
