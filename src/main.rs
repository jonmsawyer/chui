use chui::{
    Player, Color, Engine,
    parser::{self, ParserEngine},
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
    
    if let Ok(engine) = Engine::new(white, black) {
        println!("{}", engine.white_to_string());
        println!();

        println!("{}", engine.black_to_string());
        println!();

        let parser = parser::new(ParserEngine::Algebraic);
        println!("the move: {:?}", parser.parse("abc 123", &engine.board));
    
        let parser = parser::new(ParserEngine::Coordinate);
        println!("the move: {:?}", parser.parse("def 456", &engine.board));
    
        let parser = parser::new(ParserEngine::ConciseReversible);
        println!("the move: {:?}", parser.parse("ghi 789", &engine.board));
    
        let parser = parser::new(ParserEngine::Descriptive);
        println!("the move: {:?}", parser.parse("jkl 10-11-12", &engine.board));
    
        let parser = parser::new(ParserEngine::ICCF);
        println!("the move: {:?}", parser.parse("mno 13-14-15", &engine.board));
    
        let parser = parser::new(ParserEngine::ReversibleAlgebraic);
        println!("the move: {:?}", parser.parse("pqr 16-17-18", &engine.board));
    
        let parser = parser::new(ParserEngine::Smith);
        println!("the move: {:?}", parser.parse("stu 19-20-21", &engine.board));
    
        let parser = parser::new(ParserEngine::LongAlgebraic);
        println!("the move: {:?}", parser.parse("vwx 22-23-24", &engine.board));
    }
}
