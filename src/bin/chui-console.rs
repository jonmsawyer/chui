//! Chui: Chess UI
//!
//! Console application.

use chui::{Color, Engine, ParserEngine, Player};

// When Chui is run as a command line application:
//  * On Windows, the `DejaVu Sans Mono` font should be used
//    in your terminal. Size 18 font looks decent.
//
//
fn main() {
    let white = Player::new(Color::White, Some("Camina Drummer"), Some(37), None);

    let black = Player::new(Color::Black, Some("Klaes Ashford"), Some(72), Some(1500));

    Engine::new(white, black, ParserEngine::Algebraic)
        .expect("Failed to initialize engine")
        .run()
        .expect("Failed to run engine");
}
//
// Or easy default:
//
//fn main() {
//    engine = Engine::default();
//    engine.run().expect("Failed to run engine");
//}
