//! Provides the `Engine` struct. `Engine` drives the game itself.

use std::fmt;
use std::io;

use crate::{ChuiResult, ChuiError};
use super::piece::Color;
use super::player::Player;
use super::board::{ChessVariant, Board};
use super::chess_move::Move;
use super::MoveGenerator;
use super::parser::{self, Parser, ParserEngine};
use super::{Command, CommandContext, CommandKind};
use super::Fen;

/// Represents the engine of the chess game. Moves will be input
/// and output from this object. `Engine` captures and changes
/// the state of the current initialized chess game.
///
/// Example:
///
/// ```
/// use chui::{Player, Color, Engine, ParserEngine};
/// 
/// let white = Player::new(
///     Color::White,
///     Some("Camina Drummer"),
///     Some(37),
///     None,
/// );
/// 
/// let black = Player::new(
///     Color::Black,
///     Some("Klaes Ashford"),
///     Some(72),
///     Some(1500),
/// );
/// 
/// if let Ok(engine) = Engine::new(white, black, ParserEngine::Algebraic) {
///     println!("{}", engine.white_to_string());
/// };
/// ```
#[derive(Debug)]
pub struct Engine<'a> {
    /// Represents the `White` player.
    pub white: Player,

    /// Represents the `Black` player.
    pub black: Player,

    /// Represents the board as an array of arrays each containing
    /// an `Option<Piece>`.
    pub board: Board,

    /// Represents the current move parser.
    pub parser: Box<dyn Parser>,

    /// The `Color` to move.
    pub to_move: Color,

    /// Can white castle on the king side?
    pub white_can_castle_kingside: bool,

    /// Can white castle on the queen side?
    pub white_can_castle_queenside: bool,

    /// Can black castle on the king side?
    pub black_can_castle_kingside: bool,

    /// Can black castle on the queen side?
    pub black_can_castle_queenside: bool,

    /// The "ply", or number of half-moves, recorded in this game.
    pub half_move_counter: usize,

    /// The "ply", or number of half-moves, since last piece
    /// capture or pawn move.
    pub half_move_clock: usize,

    /// The number of full moves made in this game.
    pub move_counter: usize,

    /// When a pawn is moved, the en passant target square is
    /// noted, even if there's no en passant move possible. This
    /// comes from the FEN layout of the game.
    pub enpassant_target_square: (char, usize),

    /// When a pawn is moved, the en passant target square is
    /// noted, only if there's an en passant move possible. This
    /// comes from the X-FEN layout of the game.
    pub true_enpassant_target_square: (char, usize),

    /// The `MoveGenerator` object representing the move list
    /// of all possible supported chess notations. Useful for
    /// checking the parsing of a move against a known, calculated,
    /// database of possible moves. This will probably be deprecated
    /// later in favor of an actual move parser. For now, this
    /// will do. Access the move list via
    /// `self.move_generator.move_list` (which is a `Vec<String>`).
    pub move_generator: MoveGenerator<'a>,
}

/// Formats the position for white.
impl fmt::Display for Engine<'static> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}

impl Engine<'static> {
    /// Return a new instance of `Ok<Engine>` given a white
    /// `Player` and a black `Player`.
    pub fn new(player_1: Player, player_2: Player, parser_engine: ParserEngine)
    -> ChuiResult<Engine<'static>>
    {
        if player_1.color == player_2.color {
            return Err(
                ChuiError::IncompatibleSides(
                    "both players cannot be the same color".to_string()
                ),
            );
        }

        let white;
        let black;

        if player_1.color == Color::White {
            white = player_1;
            black = player_2;
        }
        else {
            white = player_2;
            black = player_1;
        }

        Ok(
            Engine {
                white,
                black,
                board: Board::new(ChessVariant::StandardChess),
                to_move: Color::White,
                white_can_castle_kingside: true,
                white_can_castle_queenside: true,
                black_can_castle_kingside: true,
                black_can_castle_queenside: true,
                half_move_counter: 0,
                half_move_clock: 0,
                move_counter: 1,
                enpassant_target_square: ('-', 9),
                true_enpassant_target_square: ('-', 9),
                move_generator: MoveGenerator::generate_move_list(),
                parser: parser::new(parser_engine),
            }
        )
    }

    /// Run the engine.
    pub fn run(&mut self) -> ChuiResult<()> {
        let mut command = Command::new(&self);
        let context = CommandContext::Main;

        loop {
            println!();
            println!("Please input move or command. (q to quit, h for help)");

            let the_move = Engine::get_input();
            
            match command.process_command(context, &the_move) {
                Some(CommandKind::Quit) => break,

                Some(CommandKind::Help) => {
                    command.display_help(context);
                    continue;
                },

                Some(CommandKind::SwitchParser) => {
                    self.switch_parser(&command);
                    command.set_commands(&self);
                    continue;
                },

                Some(CommandKind::DisplayToMove) => {
                    println!();
                    println!("{}", self.to_move_to_string());
                    continue;
                },

                Some(CommandKind::DisplayForWhite) => {
                    println!();
                    println!("{}", self.white_to_string());
                    continue;
                },

                Some(CommandKind::DisplayForBlack) => {
                    println!();
                    println!("{}", self.black_to_string());
                    continue;
                },

                Some(CommandKind::DisplayFEN) => {
                    println!();
                    println!("{}", self.get_fen());
                    continue;
                }

                _ => {
                    println!();
                    println!("Input move: {}", the_move);
            
                    match self.parse(&the_move, self.to_move) {
                        Ok(move_obj) => {
                            println!("Ok! The move: {:?}", move_obj);
                            if self.apply_move(&move_obj).is_ok() {
                                println!("{}", move_obj.get_move_text());
                                println!();
                                println!("{}", self.to_move_to_string())
                            }
                            else {
                                println!("Move not applied.")
                            }
                        },

                        Err(error) => println!("{}", error),
                    }
                },
            }
        }

        Ok(())
    }

    /// Switch the current move parser based on a `CommandKind`.
    pub fn switch_parser(&mut self, command: &Command) {
        let context = CommandContext::SwitchParser;

        loop {
            println!();
            println!("Current parser: {}", self.parser.name());
            command.display_help(context);
            println!();
            println!("Select option. (1-8, b to go back, h for help)");
    
            let input = Engine::get_input();

            match command.process_command(context, &input) {
                Some(CommandKind::SwitchToAlgebraicParser) =>{
                    let parser_engine = ParserEngine::Algebraic;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },

                Some(CommandKind::SwitchToConciseReversibleParser) => {
                    let parser_engine = ParserEngine::ConciseReversible;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },
    
                Some(CommandKind::SwitchToCoordinateParser) => {
                    let parser_engine = ParserEngine::Coordinate;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },
    
                Some(CommandKind::SwitchToDescriptiveParser) => {
                    let parser_engine = ParserEngine::Descriptive;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },
    
                Some(CommandKind::SwitchToICCFParser) => {
                    let parser_engine = ParserEngine::ICCF;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },
    
                Some(CommandKind::SwitchToLongAlgebraicParser) => {
                    let parser_engine = ParserEngine::LongAlgebraic;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },
    
                Some(CommandKind::SwitchToReversibleAlgebraicParser) => {
                    let parser_engine = ParserEngine::ReversibleAlgebraic;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },
    
                Some(CommandKind::SwitchToSmithParser) => {
                    let parser_engine = ParserEngine::Smith;
                    println!("Switching parser to {:?}.", parser_engine);
                    self.set_parser(parser_engine);
                    break;
                },

                Some(CommandKind::Help) => {
                    continue;
                },

                Some(CommandKind::Back) => {
                    println!("Not switching parser.");
                    break;
                }
    
                _ => println!("Invalid option.")
            }
        }
    }
    
    /// Parse the move. Returns am Ok(Move) is the parsing of the
    /// move is successful, otherwise a `ChuiError` will result.
    pub fn parse(&mut self, the_move: &str, to_move: Color)
    -> ChuiResult<Move>
    {
        self.parser.parse(the_move, to_move)
    }

    /// Set a new parser based on `ParserEngine`.
    pub fn set_parser(&mut self, parser_engine: ParserEngine) {
        self.parser = parser::new(parser_engine);
    }

    /// Get input string from `io::stdin()`.
    pub fn get_input() -> String {
        let mut input = String::new();
            
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input move or command.");

        input.trim().to_string()
    }

    /// Get the FEN to move character.
    pub fn get_fen_to_move(&self) -> String {
        match self.to_move {
            Color::White => "w".to_string(),
            Color::Black => "b".to_string(),
        }
    }

    /// Get the FEN for castle characters.
    pub fn get_fen_castle(&self) -> String {
        let mut castle = String::new();

        if self.white_can_castle_kingside {
            castle = format!("{}{}", castle, "K");
        }

        if self.white_can_castle_queenside {
            castle = format!("{}{}", castle, "Q");
        }

        if self.black_can_castle_kingside {
            castle = format!("{}{}", castle, "k");
        }

        if self.black_can_castle_queenside {
            castle = format!("{}{}", castle, "q");
        }

        castle
    }

    /// Get the FEN en passant square.
    pub fn get_fen_en_passant(&self) -> String {
        let (file, rank) = self.enpassant_target_square;

        if file == '-' || rank == 9 {
            "-".to_string()
        }
        else {
            format!("{}{}", file, rank)
        }
    }

    /// Get the X-FEN en passant square.
    pub fn get_x_fen_en_passant(&self) -> String {
        let (file, rank) = self.true_enpassant_target_square;

        if file == '-' || rank == 9 {
            "-".to_string()
        }
        else {
            format!("{}{}", file, rank)
        }
    }

    /// Get the FEN half-move clock.
    pub fn get_fen_half_move_clock(&self) -> String {
        self.half_move_clock.to_string()
    }

    /// Get the FEN full-move counter.
    pub fn get_fen_full_move_counter(&self) -> String {
        self.move_counter.to_string()
    }

    /// Test function to display the board colors by a straight
    /// index from `0..64` range.
    /// 
    /// Thanks to Kromey (https://github.com/Kromey).
    pub fn display_board_colors_by_index() {
        for idx in 0..64 {
            let color_id = ((idx / 8) % 2 + idx % 2) % 2;
            print!("{}  ", color_id);

            if (idx + 1) % 8 == 0 {
                println!();
            }
        }
    }

    /// Display the FEN layout of the board.
    pub fn get_fen(&self) -> String {
        format!(
            "FEN: {}\nX-FEN: {}\nShredder-FEN: {}",
            Fen::get_fen(&self),
            Fen::get_x_fen(&self),
            Fen::get_shredder_fen(&self),
        )
    }

    /// Return the display headers for white as a `String`.
    pub fn headers_for_white(&self) -> String {
        format!("{}\n{}", self.white, self.black)
    }

    /// Return the display headers for black as a `String`.
    pub fn headers_for_black(&self) -> String {
        format!("{}\n{}", self.black, self.white)
    }

    /// Return the formatted board for a given `Color` as a `String`.
    pub fn to_string(&self, color: Color) -> String {
        let alpha_coords: Vec<char> = match color {
            Color::White => ('a'..='h').collect(),
            Color::Black => ('a'..='h').rev().collect(),
        };

        let numeric_coords: Vec<u8> = (1..=8).collect();

        let display_headers = match color {
            Color::White => self.headers_for_white(),
            Color::Black => self.headers_for_black(),
        };

        let row_vec: Vec<u8> = match color {
            Color::White => (0..8).rev().collect(),
            Color::Black => (0..8).collect(),
        };

        let col_vec: Vec<u8> = match color {
            Color::White => (0..8).collect(),
            Color::Black => (0..8).rev().collect(),
        };

        let to_move = format!("{:?} to move.", self.to_move);

        let mut output = String::new();

        for i in row_vec.iter() {
            output = format!("{}{} |", output, numeric_coords[*i as usize]);
            for j in col_vec.iter() {
                output = match &self.board.get_piece(*j as usize, *i as usize) {
                    Some(piece) => format!("{} {} ", output, piece),
                    None => format!("{} · ", output),
                };
            }
            output = format!("{}\n", output.trim());
        }

        output = format!("{}  +-----------------------\n   ", output);

        for coord in alpha_coords.iter() {
            output = format!("{} {} ", output, *coord);
        }

        let output = output.trim();

        format!(
            "{}\n\
            Position:\n\
            {}\n\
            {}",
            display_headers,
            output,
            to_move,
        )
    }

    /// Display the chessboard for `White`.
    pub fn white_to_string(&self) -> String {
        self.to_string(Color::White)
    }

    /// Display the chessboard for `Black`.
    pub fn black_to_string(&self) -> String {
        self.to_string(Color::Black)
    }

    /// Display the chessboard for whomever's turn it is.
    pub fn to_move_to_string(&self) -> String {
        if self.to_move == Color::White {
            self.to_string(Color::White)
        }
        else {
            self.to_string(Color::Black)
        }
    }

    /// Apply the move.
    pub fn apply_move(&mut self, move_obj: &Move) -> ChuiResult<()> {
        let apply = self.board.apply_move(move_obj);

        if apply.is_ok() {
            self.toggle_to_move();
        }

        apply
    }

    /// Switch `Player` to move.
    pub fn toggle_to_move(&mut self) {
        if let Color::White = self.to_move {
            self.to_move = Color::Black;
        }
        else {
            self.to_move = Color::White;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn engine_init_incompatible_sides() {
        let white = Player::new(
            Color::White,
            Some("Camina Drummer"),
            Some(37),
            None,
        );
    
        let white_2 = Player::new(
            Color::White,
            Some("Fred Johnson"),
            None,
            Some(2483),
        );
    
        if let Err(error) = Engine::new(
            white,
            white_2,
            ParserEngine::Algebraic
        )
        {
            panic!("{}", error);
        }
    }

    #[test]
    fn engine_init_correctly() {
        let white = Player::new(
            Color::White,
            Some("Camina Drummer"),
            Some(37),
            None,
        );
    
        let black = Player::new(
            Color::Black,
            Some("Fred Johnson"),
            None,
            Some(2483),
        );
    
        if let Err(error) = Engine::new(
            black,
            white,
            ParserEngine::Algebraic
        )
        {
            panic!("{}", error);
        }
    }
}
