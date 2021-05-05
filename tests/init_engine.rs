use chui::{Engine, Player, Color};

#[test]
fn init_engine() {
    let white = Player::new(
        Color::White,
        Some("Nathan Vander Martin"),
        Some(36),
        None,
    );

    let black = Player::new(
        Color::Black,
        Some("Shop Guy. Bob Vila III"),
        Some(57),
        Some(987),
    );

    if let Ok(engine) = Engine::new(white, black) {

        // Note the whitespace around the output.
        assert_eq!(
            &format!("{}", engine),
            "White: Nathan Vander Martin (Age 36) (no Elo rating)
Black: Shop Guy. Bob Vila III (Age 57) (987 Elo)
Position:
8 | r  n  b  q  k  b  n  r
7 | p  p  p  p  p  p  p  p
6 | ·  ·  ·  ·  ·  ·  ·  ·
5 | ·  ·  ·  ·  ·  ·  ·  ·
4 | ·  ·  ·  ·  ·  ·  ·  ·
3 | ·  ·  ·  ·  ·  ·  ·  ·
2 | P  P  P  P  P  P  P  P
1 | R  N  B  Q  K  B  N  R
  +-----------------------
    a  b  c  d  e  f  g  h
White to move."
        );
    }
}
