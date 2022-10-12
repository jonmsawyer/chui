//! Chui: Chess UI

//
// For Bevy app
//

use chui::Ui;

fn main() {
    Ui::run();
}

//
// For console app
//

// When Chui is run as a command line application:
//  * On Windows, the `DejaVu Sans Mono` font should be used
//    in your terminal. Size 18 font looks decent.

// use chui::{
//     Engine, Player, Color,
//     parser::ParserEngine,
// };
//
// fn main() {
//     let white = Player::new(
//         Color::White,
//         Some("Camina Drummer"),
//         Some(37),
//         None,
//     );
//
//     let black = Player::new(
//         Color::Black,
//         Some("Klaes Ashford"),
//         Some(72),
//         Some(1500),
//     );
//
//     Engine::new(white, black, ParserEngine::Algebraic)
//         .expect("Failed to initialize engine")
//         .run()
//         .expect("Failed to run engine");
// }
