mod tests {
    #[test]
    fn init_engine() {
        let white = chui::Player::new(
            chui::PieceColor::White,
            "Vander Martin",
            Some("Nathan"),
            None,
            None,
            Some(36),
            None,
        );

        let black = chui::Player::new(
            chui::PieceColor::Black,
            "Vila",
            Some("Bob"),
            Some("Shop Guy."),
            Some("III"),
            Some(57),
            Some(987),
        );

        let engine = chui::Engine::new(white, black);

        // Note the whitespace around the output.
        assert_eq!(
            &format!("{}", engine),
            "White: Vander Martin, Nathan (no rating)
Black: Shop Guy. Vila, Bob III (987)
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
