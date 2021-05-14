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
    
    let engine = Engine::new(white, black).unwrap();

    println!("{}", engine.white_to_string());
    println!();

    let parser = parser::new(ParserEngine::Algebraic);

    loop {
        println!("Please input your move. (q to quit)");
        
        let mut input_move = String::new();
        
        io::stdin().read_line(&mut input_move)
                   .expect("Failed to read line.");
        

        let the_move = input_move.trim().to_string();
        
        if *"q".to_string() == the_move ||
           *"quit".to_string() == the_move
        {
            break;
        }

        println!("Your move: {}", the_move);
        
        match parser.parse(&the_move, &engine) {
            Ok(the_move) => println!("Ok! The move: {}", the_move),
            Err(error) => println!("{}", error),
        }
    }
}
