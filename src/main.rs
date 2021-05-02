use chui::{
    Player, PieceColor, Engine,
    parser::{self, ParserEngine}
};

fn main() {
    let white = Player::new(
        PieceColor::White,
        "Drummer",
        Some("Camina"),
        None,
        None,
        Some(37),
        None,
    );

    let black = Player::new(
        PieceColor::Black,
        "Ashford",
        Some("Klaes"),
        None,
        None,
        Some(72),
        Some(1500),
    );
    
    let engine = Engine::new(white, black);

    println!("{}", engine.display_for_white());
    println!();

    println!("{}", engine.display_for_black());
    println!();

    // Erroneous and possible moves for
    // `{col}{row}-{col}{row}{{=,/}RNBQ{+,++,#}}` format.

    // let the_move = engine.make_move("e4   21  1-e5-64");
    // println!("The move: {} [{:?}]", the_move, the_move);
    // println!("The move: {}", engine.make_move("e4-f5-g4"));
    // println!("The move: {}", engine.make_move("e41-e5"));
    // println!("The move: {}", engine.make_move("e4-a41"));
    // println!("The move: {}", engine.make_move("y3-e5"));
    // println!("The move: {}", engine.make_move("aa-y3"));
    // println!("The move: {}", engine.make_move("a8-z3"));
    // println!("The move: {}", engine.make_move("a8-a9"));
    // println!("The move: {}", engine.make_move("a7-a8=Q"));
    // println!("The move: {}", engine.make_move("a7-a8=R"));
    // println!("The move: {}", engine.make_move("a7-a8=N"));
    // println!("The move: {}", engine.make_move("a7-a8=B"));
    // println!("The move: {}", engine.make_move("a7-a8Q"));
    // println!("The move: {}", engine.make_move("a7-a8R"));
    // println!("The move: {}", engine.make_move("a7-a8N"));
    // println!("The move: {}", engine.make_move("a7-a8B"));
    // println!("The move: {}", engine.make_move("a7-a8=Q+"));
    // println!("The move: {}", engine.make_move("a7-a8=R++"));
    // println!("The move: {}", engine.make_move("a7-a8=N#"));
    // println!("The move: {}", engine.make_move("a7-a8=B+"));
    // println!("The move: {}", engine.make_move("a7-a8Q++"));
    // println!("The move: {}", engine.make_move("a7-a8R#"));
    // println!("The move: {}", engine.make_move("a7-a8N+"));
    // println!("The move: {}", engine.make_move("a7-a8+"));
    // println!("The move: {}", engine.make_move("a7-a8++"));
    // println!("The move: {}", engine.make_move("a7-a8#"));
    // let the_move = engine.make_move("a1-a3");
    // println!("The move: {} [{:?}]", the_move, the_move);
    // let the_move = engine.make_move("a1-d4");
    // println!("The move: {} [{:?}]", the_move, the_move);
    // let the_move = engine.make_move("h5-b5");
    // println!("The move: {} [{:?}]", the_move, the_move);
    // let the_move = engine.make_move("a1-b3");
    // println!("The move: {} [{:?}]", the_move, the_move);
    // let the_move = engine.make_move("h7-a7");
    // println!("The move: {} [{:?}]", the_move, the_move);
    // let the_move = engine.make_move("g3-f4");
    // println!("The move: {} [{:?}]", the_move, the_move);

    let parser = parser::new(ParserEngine::Algebraic);
    println!("the move: {:?}", parser.parse("abc 123", &engine.board));

    let parser = parser::new(ParserEngine::Coordinate);
    println!("the move: {:?}", parser.parse("def 456", &engine.board));
}
