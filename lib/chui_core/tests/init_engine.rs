//! Chui Core Tests

use chui_error as _;
use chui_macros as _;
use colored as _;
use nonmax as _;

use chui_core::prelude::*;

// #[test]
fn _init_engine() {
    let white = Player::new(Color::White, Some("Nathan Vander Martin"), Some(36), None);

    let black = Player::new(
        Color::Black,
        Some("Shop Guy. Bob Vila III"),
        Some(57),
        Some(987),
    );

    if let Ok(engine) = Game::new(white, black, ParserEngine::Algebraic) {
        assert_eq!(
            &format!("{}", engine),
            "White: Nathan Vander Martin (Age 36) (no Elo rating)
Black: Shop Guy. Bob Vila III (Age 57) (987 Elo)
Position:
╔═════════════════════════╗
║     a b c d e f g h     ║
║   ┌─────────────────┐   ║
║ 8 │ r n b q k b n r │ 8 ║
║ 7 │ p p p p p p p p │ 7 ║
║ 6 │ · · · · · · · · │ 6 ║
║ 5 │ · · · · · · · · │ 5 ║
║ 4 │ · · · · · · · · │ 4 ║
║ 3 │ · · · · · · · · │ 3 ║
║ 2 │ P P P P P P P P │ 2 ║
║ 1 │ R N B Q K B N R │ 1 ║
║   └─────────────────┘   ║
║     a b c d e f g h   \u{1b}[1;33m♔\u{1b}[0m ║
╚═════════════════════════╝
White to move."
        );
    }
}
