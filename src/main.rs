//use std::convert::TryFrom;

use std::io;

use chui::{
    Player, Color, Engine,
    parser::{self, ParserEngine},
    //ChuiError,
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
    
    let engine = Engine::new(
        white,
        black,
        ParserEngine::Algebraic,
    ).unwrap();

    println!("{}", engine.white_to_string());
    println!();

    let mut parser = parser::new(ParserEngine::Algebraic);

    loop {
        println!("Please input your move. (q to quit, h for help)");
        
        let mut input_move = String::new();
        
        io::stdin()
            .read_line(&mut input_move)
            .expect("Failed to read input move or command.");
        

        let the_move = input_move.trim().to_string();
        
        if the_move.eq("q") || the_move.eq("quit") {
            break;
        }

        if the_move.eq("h") || the_move.eq("help") {
            display_help(&engine);
            continue
        }

        println!("Your move: {}", the_move);
        
        match parser.parse(&the_move, &engine) {
            Ok(the_move) => println!("Ok! The move: {}", the_move),
            Err(error) => println!("{}", error),
        }
    }
}

fn display_help(engine: &Engine) {
    println!("Parser Engine: {:?}", engine.parser);
}
