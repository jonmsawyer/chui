//! Provides the `Engine` struct. `Engine` drives the game itself.

use std::fmt;
use std::io;

use bevy::prelude::*;

use super::board::{Board, ChessVariant};
use super::chess_move::Move;
use super::piece::{Color, Piece};
use super::player::Player;
use crate::{ChuiError, ChuiResult};
//use super::MoveGenerator;
use super::parser::{self, Parser, ParserEngine};
use super::{Command, CommandContext, CommandKind};
//use super::Fen;

mod commands;
mod fen;

/// The win condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinCondition {
    /// Checkmate.
    Checkmate,

    /// White resigns.
    WhiteResigns,

    /// Black resigns.
    BlackResigns,
}

/// The draw condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawCondition {
    /// Both players agree to a draw.
    AgreeToDraw,

    /// The one to move cannot make any valid moves.
    Stalemate,

    /// The board's position has repeated itself three times.
    ThriceRepitition,

    /// 50 moves have been made with no piece capture or pawn move.
    FiftyMoveRule,

    /// Both players have insufficient material to check mate.
    InsufficientMaterial,

    /// Both players agree to that there will be perpetual check.
    PerpetualCheck,
}

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
#[derive(Debug, Resource)]
pub struct Engine {
    /// Represents the `White` player.
    pub white: Player,

    /// Represents the `Black` player.
    pub black: Player,

    /// Represents the board as an array of arrays each containing
    /// an `Option<Piece>`.
    pub board: Board,

    /// A vec containing the pieces that have been captured.
    pub captured_pieces: Vec<Piece>,

    /// Represents the current move parser.
    pub parser: Box<dyn Parser + Send + Sync>,

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

    /// Does White win?
    pub white_wins: bool,

    /// Does Black win?
    pub black_wins: bool,

    /// Is the game a draw?
    pub is_draw: bool,

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
    /*
    pub move_generator: MoveGenerator<'a>,
    */
    pub move_list: Vec<Move>,

    /// The current move.
    pub current_move: Option<Move>,

    /// The win condition.
    pub win_condition: Option<WinCondition>,

    /// The draw condition.
    pub draw_condition: Option<DrawCondition>,

    /// Display the chessboard for a particular `Color`.
    pub display_for: Option<Color>,
}

impl Default for Engine {
    fn default() -> Self {
        let white = Player::new(Color::White, Some("Camina Drummer"), Some(37), None);

        let black = Player::new(Color::Black, Some("Klaes Ashford"), Some(72), Some(1500));

        Engine::new(white, black, ParserEngine::Algebraic).expect("Failed to initialize engine")
    }
}

/// Formats the position for white.
impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}

impl Engine {
    /// Return a new instance of `Ok<Engine>` given a white
    /// `Player` and a black `Player`.
    ///
    /// # Errors
    ///
    /// * Returns a `ChuiError` when the engine was initialized with incompatible
    ///   sides.
    pub fn new(
        player_1: Player,
        player_2: Player,
        parser_engine: ParserEngine,
    ) -> ChuiResult<Engine> {
        if player_1.color == player_2.color {
            return Err(ChuiError::IncompatibleSides(
                "both players cannot be the same color".to_string(),
            ));
        }

        let (white, black) = if player_1.color == Color::White {
            (player_1, player_2)
        } else {
            (player_2, player_1)
        };

        Ok(Engine {
            white,
            black,
            board: Board::new(ChessVariant::StandardChess),
            captured_pieces: Vec::<Piece>::new(),
            to_move: Color::White,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            white_wins: false,
            black_wins: false,
            is_draw: false,
            half_move_counter: 0,
            half_move_clock: 0,
            move_counter: 1,
            enpassant_target_square: ('-', 9),
            true_enpassant_target_square: ('-', 9),
            //move_generator: MoveGenerator::generate_move_list(),
            parser: parser::new(parser_engine),
            move_list: Vec::<Move>::new(),
            current_move: None,
            win_condition: None,
            draw_condition: None,
            display_for: None,
        })
    }

    /// Run the engine.
    ///
    /// # Errors
    ///
    /// * Errors when...
    pub fn run(&mut self) -> ChuiResult<()> {
        let mut command = Command::new(self);
        let context = CommandContext::Main;
        let mut break_loop = false;
        let mut display_board = true;

        loop {
            if display_board {
                println!("{}", self.to_move_to_string());
            } else {
                display_board = true;
            }
            println!();
            println!("Please input move(s) or command. (q to quit, h for help)");

            let move_input = Engine::get_input();

            for move_str in move_input.split_whitespace() {
                let the_move = String::from(move_str);
                let command_move = the_move.clone();
                match command.process_command(context, command_move) {
                    Some(CommandKind::Quit) => {
                        break_loop = true;
                    }

                    Some(CommandKind::Help) => {
                        command.display_help(context);
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::SwitchParser) => {
                        self.switch_parser(&command);
                        command.rebuild_commands(self);
                        continue;
                    }

                    Some(CommandKind::DisplayToMove) => {
                        println!();
                        println!("{}", self.to_move_to_string());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::DisplayForWhite) => {
                        println!();
                        println!("{}", self.white_to_string());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::DisplayForBlack) => {
                        println!();
                        println!("{}", self.black_to_string());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::DisplayFEN) => {
                        println!();
                        println!("{}", self.get_fen());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::WhiteResigns) => {
                        println!();
                        println!("White resigns.");
                        self.win_condition = Some(WinCondition::WhiteResigns);
                        self.draw_condition = None;
                        continue;
                    }

                    Some(CommandKind::BlackResigns) => {
                        println!();
                        println!("Black resigns.");
                        self.win_condition = Some(WinCondition::BlackResigns);
                        self.draw_condition = None;
                        continue;
                    }

                    Some(CommandKind::DisplayForWhiteEachMove) => {
                        println!();
                        println!("Display for White after each move.");
                        self.display_for = Some(Color::White);
                        continue;
                    }

                    Some(CommandKind::DisplayForBlackEachMove) => {
                        println!();
                        println!("Display for Black after each move.");
                        self.display_for = Some(Color::Black);
                        continue;
                    }

                    Some(CommandKind::DisplayMoveList) => {
                        let mut output = String::new();

                        println!();

                        for (move_idx, move_obj) in self.move_list.iter().enumerate() {
                            let numeral = if move_idx % 2 == 0 {
                                format!("\n{}. ", (move_idx + 2) / 2)
                            } else {
                                String::new()
                            };

                            output = format!("{}{}{} ", output, numeral, move_obj);
                        }

                        if self.move_list.is_empty() {
                            output = "No moves have been made.".to_string();
                        }

                        display_board = false;

                        println!("Move List Notation:\n{}", output.trim());
                    }

                    _ => {
                        println!();
                        println!("Input move or command: {}", the_move);

                        // Ignore any moves or commands with a '.' in it.
                        // Eg., "1."
                        if the_move.contains('.') {
                            continue;
                        }

                        if the_move.eq("1-0") {
                            self.win_condition = Some(WinCondition::BlackResigns);
                            self.draw_condition = None;
                        } else if the_move.eq("0-1") {
                            self.win_condition = Some(WinCondition::WhiteResigns);
                            self.draw_condition = None;
                        } else if the_move.eq("1/2-1/2") || the_move.eq("½-½") {
                            self.win_condition = None;
                            self.draw_condition = None; // ?
                        }

                        match self.parse(the_move, self.to_move).as_ref() {
                            Ok(move_obj) => {
                                println!("Ok! The move: {:?}", move_obj);
                                self.current_move = Some(move_obj.clone());
                                if self.apply_move().is_ok() {
                                    println!("{}", move_obj.get_move_text());
                                    println!();

                                    self.move_list.push(move_obj.clone());

                                    self.half_move_counter += 1;
                                    if self.half_move_counter % 2 == 0 {
                                        self.move_counter += 1;
                                    }
                                } else {
                                    println!("Move not applied.");
                                    break;
                                }
                            }

                            Err(error) => println!("{}", error),
                        }
                    }
                }
            }

            if break_loop {
                break;
            }
        }

        Ok(())
    }

    /// Switch the current move parser based on a `CommandKind`.
    pub fn switch_parser(&mut self, command: &Command) {
        commands::switch_parser(self, command);
    }

    /// Parse the move. Returns an Ok(Move) if the parsing of the
    /// move is successful, otherwise a `ChuiError` will result.
    ///
    /// # Errors
    ///
    /// * Errors when the parser cannot parse the move.
    pub fn parse(&mut self, the_move: String, to_move: Color) -> ChuiResult<Move> {
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
        fen::get_fen_to_move(self)
    }

    /// Get the FEN for castle characters.
    pub fn get_fen_castle(&self) -> String {
        fen::get_fen_castle(self)
    }

    /// Get the FEN en passant square.
    pub fn get_fen_en_passant(&self) -> String {
        fen::get_fen_en_passant(self)
    }

    /// Get the X-FEN en passant square.
    pub fn get_x_fen_en_passant(&self) -> String {
        fen::get_x_fen_en_passant(self)
    }

    /// Get the FEN half-move clock.
    pub fn get_fen_half_move_clock(&self) -> String {
        fen::get_fen_half_move_clock(self)
    }

    /// Get the FEN full-move counter.
    pub fn get_fen_full_move_counter(&self) -> String {
        fen::get_fen_full_move_counter(self)
    }

    /// Display the FEN layout of the board.
    pub fn get_fen(&self) -> String {
        fen::get_fen(self)
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

        let mut output = "╔═════════════════════════╗\n║    ".to_string();

        for coord in alpha_coords.iter() {
            output = format!("{} {}", output, *coord);
        }

        output = format!("{}     ║\n║   ┌─────────────────┐   ║\n", output);

        for i in row_vec.iter() {
            output = format!("{}║ {} │", output, numeric_coords[*i as usize]);
            for j in col_vec.iter() {
                output = self.board.get_piece(*j as usize, *i as usize).map_or_else(
                    || format!("{} ·", output),
                    |piece| format!("{} {}", output, piece),
                );
            }
            output = format!("{} │ {} ║\n", output.trim(), numeric_coords[*i as usize]);
        }

        output = format!("{}║   └─────────────────┘   ║\n║    ", output);

        for coord in alpha_coords.iter() {
            output = format!("{} {}", output, *coord);
        }

        output = format!(
            "{} {}",
            output,
            format_args!("  {} ║\n╚═════════════════════════╝", self.to_move)
        );

        let output = output.trim();

        format!(
            "{}\n\
            Position:\n\
            {}\n\
            {}",
            display_headers, output, to_move,
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
        self.display_for.map_or_else(
            || {
                if self.to_move == Color::White {
                    self.to_string(Color::White)
                } else {
                    self.to_string(Color::Black)
                }
            },
            |display_for| self.to_string(display_for),
        )
    }

    /// Apply the move.
    ///
    /// # Errors
    ///
    /// * Errors if the piece we're moving is `None`.
    pub fn apply_move(&mut self) -> ChuiResult<()> {
        let apply = self.board.apply_move(&self.current_move);

        if apply.is_ok() {
            self.toggle_to_move();
        }

        apply
    }

    /// Switch `Player` to move.
    pub fn toggle_to_move(&mut self) {
        if let Color::White = self.to_move {
            self.to_move = Color::Black;
        } else {
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
        let white = Player::new(Color::White, Some("Camina Drummer"), Some(37), None);

        let white_2 = Player::new(Color::White, Some("Fred Johnson"), None, Some(2483));

        if let Err(error) = Engine::new(white, white_2, ParserEngine::Algebraic) {
            panic!("{}", error);
        }
    }

    #[test]
    fn engine_init_correctly() {
        let white = Player::new(Color::White, Some("Camina Drummer"), Some(37), None);

        let black = Player::new(Color::Black, Some("Fred Johnson"), None, Some(2483));

        if let Err(error) = Engine::new(black, white, ParserEngine::Algebraic) {
            panic!("{}", error);
        }
    }
}
