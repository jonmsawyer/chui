//! Provides the `Engine` struct. `Engine` drives the game itself.

use std::fmt;

use crate::ChuiError;
use super::chess_move::Move;
use super::piece::{Piece, Color};
use super::player::Player;
use super::MoveGenerator;

/// Represents the engine of the chess game. Moves will be input
/// and output from this object. `Engine` captures and changes
/// the state of the current initialized chess game.
///
/// Example:
///
/// ```
/// use chui::{Player, Color, Engine};
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
/// if let Ok(engine) = Engine::new(white, black) {
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
    /// a `Square`.
    pub board: [[Option<Piece>; 8]; 8],

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

    /// Represents the half-move counter for pawn moves and piece
    /// capture. Needed to declare the "50-move rule" draws in
    /// chess games.
    pub pawn_move_or_piece_capture_half_move_counter: u32,

    /// The "ply", or number of half-moves, recorded in this game.
    pub half_move_counter: u32,

    /// The number of full moves made in this game.
    pub move_counter: u32,

    /// When a pawn is moved, the en passant target square is
    /// noted, even if there's no en passant move possible. This
    /// comes from the FEN layout of the game.
    pub enpassant_target_square: (char, u32),

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

    /// Return the display headers for white as a `String`.
    pub fn headers_for_white(&self) -> String {
        format!("{}\n{}", self.white, self.black)
    }

    /// Return the display headers for black as a `String`.
    pub fn headers_for_black(&self) -> String {
        format!("{}\n{}", self.black, self.white)
    }

    /// Make the specified move. If the move is valid, the
    /// chessboard will be modified accordingly. If the move
    /// is invalid, an invalid `Move` will be returned.
    pub fn make_move(&self, the_move: &str) -> Move {
        Move::parse_move(the_move, self.to_move)
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
            Color::White => (0..8).collect(),
            Color::Black => (0..8).rev().collect(),
        };

        let col_vec = row_vec.clone();

        let to_move = format!("{:?} to move.", self.to_move);

        let mut output = String::new();

        for i in row_vec.iter().rev() {
            output = format!("{}{} |", output, numeric_coords[*i as usize]);
            for j in col_vec.iter() {
                output = match &self.board[*i as usize][*j as usize] {
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

    /// Produces a row (`[Option<Piece>; 8]`) of pieces according their color.
    pub fn row_of_pieces(color: Color) -> [Option<Piece>; 8] {
        [
            Some(Piece::Rook(color)),
            Some(Piece::Knight(color)),
            Some(Piece::Bishop(color)),
            Some(Piece::Queen(color)),
            Some(Piece::King(color)),
            Some(Piece::Bishop(color)),
            Some(Piece::Knight(color)),
            Some(Piece::Rook(color)),
        ]
    }

    /// Return a new instance of `Ok<Engine>` given a white
    /// `Player` and a black `Player`.
    pub fn new(player_1: Player, player_2: Player)
    -> crate::Result<Engine<'static>>
    {
        if player_1.color == player_2.color {
            return Err(
                ChuiError::IncompatibleSides(
                    "both players cannot be the same color"
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
                to_move: Color::White,
                white_can_castle_kingside: true,
                white_can_castle_queenside: true,
                black_can_castle_kingside: true,
                black_can_castle_queenside: true,
                pawn_move_or_piece_capture_half_move_counter: 0,
                half_move_counter: 0,
                move_counter: 0,
                enpassant_target_square: ('-', 0),
                move_generator: MoveGenerator::generate_move_list(),
                board: [
                    Engine::row_of_pieces(Color::White),  // rank 1
                    [Some(Piece::Pawn(Color::White)); 8], // rank 2
                    [None; 8],                            // rank 3
                    [None; 8],                            // rank 4
                    [None; 8],                            // rank 5
                    [None; 8],                            // rank 6
                    [Some(Piece::Pawn(Color::Black)); 8], // rank 2
                    Engine::row_of_pieces(Color::Black),  // rank 8
                ],
            }
        )
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
    
        if let Err(error) = Engine::new(white, white_2) {
            panic!("{}", error);
        }

        let black = Player::new(
            Color::Black,
            Some("Camina Drummer"),
            Some(37),
            None,
        );
    
        let black_2 = Player::new(
            Color::Black,
            Some("Fred Johnson"),
            None,
            Some(2483),
        );
    
        if let Err(error) = Engine::new(black, black_2) {
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
    
        if let Err(error) = Engine::new(black, white) {
            panic!("{}", error);
        }
    }
}
