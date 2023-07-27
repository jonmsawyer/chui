//! Provides the `Engine` struct. `Engine` drives the game itself.

use std::collections::HashMap;
use std::fmt;
use std::io;

use crate::{
    parser::{self, Parser, ParserEngine},
    Board, ChessVariant, ChuiError, ChuiResult, Color, Command, Coord, Move, Piece, Player,
};
//use super::Fen;

mod commands;
mod fen;

/// The win condition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinCondition {
    /// Checkmate.
    #[default]
    Checkmate,

    /// White resigns.
    WhiteResigns,

    /// Black resigns.
    BlackResigns,
}

/// The draw condition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DrawCondition {
    /// Both players agree to a draw.
    #[default]
    AgreeToDraw,

    /// The one to move cannot make any valid moves.
    Stalemate,

    /// The board's position has repeated itself three times.
    ///
    /// According to FIDE rules, just because a position has repeated itself three times,
    /// doesn't mean the game is automatically a draw. A player must flag this condition after
    /// the position has repeated itself for the third time.
    ///
    /// Also note this has nothing to do with move order. This is a repetition of position, not
    /// moves.
    ThirdRepitition,

    /// The board's position has repeated itself five times.
    ///
    /// According to FIDE rules, a game is automatically drawn after the position has repeated
    /// itself five times. A player is not needed to flag this condition.
    FifthRepetition,

    /// 50 moves have been made with no piece capture or pawn move.
    FiftyMoveRule,

    /// Both players have insufficient material to check mate. Do note that it is still possible
    /// to checkmate an opponent King with just a bishop or a knight provided that the opponent
    /// has a blocking piece to make this possible.
    InsufficientMaterial,

    /// Both players agree to that there will be perpetual check. This draw condition is
    /// technically not needed because a perpectual check will often result in a position
    /// repetition or vai the Fifty Move Rule.
    PerpetualCheck,
}

/// Represents the engine of the chess game. Moves will be input
/// and output from this object. `Engine` captures and changes
/// the state of the current initialized chess game.
///
/// Example:
///
/// ```
/// use chui::{Player, Color, Game, ParserEngine};
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
/// if let Ok(game) = Game::new(white, black, ParserEngine::Algebraic) {
///     println!("{}", game.white_to_string());
/// };
/// ```
#[derive(Debug)]
pub struct Game {
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

    /// A hashmap of FEN positions with their position count. Used to count position repetitions
    /// via the Third Repetition and Fifth Repetition draw condition.
    pub position_record: HashMap<String, u8>,

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
    current_move: Option<Move>,

    /// The win condition.
    pub win_condition: Option<WinCondition>,

    /// The draw condition.
    pub draw_condition: Option<DrawCondition>,

    /// Display the chessboard for a particular `Color`.
    pub display_for: Option<Color>,
}

impl Default for Game {
    fn default() -> Self {
        let white = Player::new(Color::White, Some("Camina Drummer"), Some(37), None);

        let black = Player::new(Color::Black, Some("Klaes Ashford"), Some(72), Some(1500));

        Game::new(white, black, ParserEngine::Algebraic).expect("Failed to initialize engine")
    }
}

/// Formats the position for white.
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}

impl Game {
    /// Return a new instance of [`ChuiResult<Game>`] given a white
    /// [`Player`] and a black [`Player`].
    ///
    /// # Errors
    ///
    /// * Returns a [`ChuiError`] when the engine was initialized with incompatible
    ///   sides.
    pub fn new(
        player_1: Player,
        player_2: Player,
        parser_engine: ParserEngine,
    ) -> ChuiResult<Game> {
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

        Ok(Game {
            white,
            black,
            board: Board::new(ChessVariant::StandardChess),
            captured_pieces: Vec::<Piece>::new(),
            to_move: Color::White,
            position_record: HashMap::new(),
            white_wins: false,
            black_wins: false,
            is_draw: false,
            half_move_counter: 0,
            half_move_clock: 0,
            move_counter: 1,
            //move_generator: MoveGenerator::generate_move_list(),
            parser: parser::new(parser_engine),
            move_list: Vec::<Move>::new(),
            current_move: None,
            win_condition: None,
            draw_condition: None,
            display_for: None,
        })
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
    ///
    /// # Panics
    ///
    /// Panice if a new [`Coord`] could not be constructed.
    ///
    /// TODO: Mitigate panics.
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
                output = self
                    .board
                    .get_piece(Coord::try_from((*j, *i)).unwrap())
                    .map_or_else(
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
    /// TODO: Set more game state per the rules, such as en passant.
    ///
    /// # Errors
    ///
    /// * Errors if the piece we're moving is `None`.
    pub fn apply_move(&mut self) -> ChuiResult<()> {
        let current_move = self.process_move()?;
        if let Some(current_move) = current_move {
            println!("Apply Move: {:?}", current_move);
            println!("{}", current_move.get_move_text());
            println!();
            let apply = self.board.apply_move(&current_move);

            if let Ok(captured_piece) = apply {
                if let Some(captured_piece) = captured_piece {
                    self.captured_pieces.push(captured_piece);
                }
                self.toggle_to_move();
            }

            Ok(())
        } else {
            Err(ChuiError::InvalidMove("No move to apply".to_string()))
        }
    }

    /// Process the chess move.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if the chess move could not be processed.
    pub fn process_move(&mut self) -> ChuiResult<Option<Move>> {
        if let Some(mut chess_move) = self.current_move.clone() {
            chess_move.process_move(self)?;
            return Ok(Some(chess_move));
        }

        Err(ChuiError::InvalidMove("No move to apply".to_string()))
    }

    /// Switch `Player` to move.
    pub fn toggle_to_move(&mut self) {
        if let Color::White = self.to_move {
            self.to_move = Color::Black;
        } else {
            self.to_move = Color::White;
        }
    }

    /// Set the current move.
    pub fn set_current_move(&mut self, current_move: Option<Move>) {
        self.current_move = current_move;
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

        if let Err(error) = Game::new(white, white_2, ParserEngine::Algebraic) {
            panic!("{}", error);
        }
    }

    #[test]
    fn engine_init_correctly() {
        let white = Player::new(Color::White, Some("Camina Drummer"), Some(37), None);

        let black = Player::new(Color::Black, Some("Fred Johnson"), None, Some(2483));

        if let Err(error) = Game::new(black, white, ParserEngine::Algebraic) {
            panic!("{}", error);
        }
    }
}
