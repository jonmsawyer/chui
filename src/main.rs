//use std::convert::TryFrom;

use chui::{
    Engine, Player, Color,
    parser::ParserEngine,
};

fn main() {
    let white = Player::new(
        Color::White,
        Some("Camina Drummer"),
        Some(37),
        None,
    );

    let black = Player::new(
        Color::Black,
        Some("Klaes Ashford"),
        Some(72),
        Some(1500),
    );
    
    let mut engine = Engine::new(
        white,
        black,
        ParserEngine::Algebraic,
    ).unwrap();

    println!("{}", engine.to_move_to_string());

    engine.run().expect("Failed to run engine.");
}
