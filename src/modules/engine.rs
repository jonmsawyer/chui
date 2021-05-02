//! File: `engine.rs`
//!
//! Module: `engine`
//!
//! Provides the `Engine` struct. `Engine` drives the game itself.

use std::fmt;

use super::chess_move::Move;
use super::color::{PieceColor, SquareColor};
use super::piece::Piece;
use super::player::Player;
use super::square::Square;
use super::MoveGenerator;

/// Represents the engine of the chess game. Moves will be input
/// and output from this object. `Engine` captures and changes
/// the state of the current initialized chess game.
///
/// Example:
///
/// ```
/// use chui::{Player, PieceColor, Engine};
/// 
/// let white = Player::new(
///     PieceColor::White,
///     "Drummer",
///     Some("Camina"),
///     None,
///     None,
///     Some(37),
///     None,
/// );
/// 
/// let black = Player::new(
///     PieceColor::Black,
///     "Ashford",
///     Some("Klaes"),
///     None,
///     None,
///     Some(72),
///     Some(1500),
/// );
/// 
/// let mut engine = Engine::new(white, black);
/// 
/// println!("{}", engine.display_for_white());
/// ```
#[derive(Debug)]
pub struct Engine<'a> {
    /// Represents the `White` player.
    pub white: Player,
    /// Represents the `Black` player.
    pub black: Player,
    /// Represents the board as an array of arrays each containing
    /// a `Square`.
    pub board: [[Square; 8]; 8],
    /// The `PieceColor` to move.
    pub to_move: PieceColor,
    /// Can white castle on the king side?
    pub can_white_castle_kingside: bool,
    /// Can white castle on the queen side?
    pub can_white_castle_queenside: bool,
    /// Can black castle on the king side?
    pub can_black_castle_kingside: bool,
    /// Can black castle on the queen side?
    pub can_black_castle_queenside: bool,
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
    /// `self.move_generator.move_list`.
    pub move_generator: MoveGenerator<'a>,
}

/// Implements `Display` for `Engine`. Displays the position for
/// white.
impl fmt::Display for Engine<'static> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display_for_white())
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
    pub fn display_headers_for_white(&self) -> String {
        format!("{}\n{}", self.white, self.black)
    }

    /// Return the display headers for black as a `String`.
    pub fn display_headers_for_black(&self) -> String {
        format!("{}\n{}", self.black, self.white)
    }

    /// Make the specified move. If the move is valid, the
    /// chessboard will be modified accordingly. If the move
    /// is invalid, an invalid `Move` will be returned.
    pub fn make_move(&self, the_move: &str) -> Move {
        Move::parse_move(the_move, self.to_move)
    }

    /// Match the `Piece` for the given `&Square` and return
    /// a `&str` instance of the representation of the piece
    /// on the chessboard. If the square is empty, a "·" str
    /// is returned.
    pub fn match_for_piece(&self, square: &Square) -> &str {
        match square.piece {
            Piece::None => "·",
            Piece::Pawn(PieceColor::White) => "P",
            Piece::Rook(PieceColor::White) => "R",
            Piece::Knight(PieceColor::White) => "N",
            Piece::Bishop(PieceColor::White) => "B",
            Piece::Queen(PieceColor::White) => "Q",
            Piece::King(PieceColor::White) => "K",
            Piece::Pawn(PieceColor::Black) => "p",
            Piece::Rook(PieceColor::Black) => "r",
            Piece::Knight(PieceColor::Black) => "n",
            Piece::Bishop(PieceColor::Black) => "b",
            Piece::Queen(PieceColor::Black) => "q",
            Piece::King(PieceColor::Black) => "k",
        }
    }

    /// Return the display of the board for a given `PieceColor` as
    /// a `String`.
    pub fn display(&self, color: PieceColor) -> String {
        let alpha_coords: Vec<char> = match color {
            PieceColor::White => ('a'..='h').collect(),
            PieceColor::Black => ('a'..='h').rev().collect(),
        };

        let numeric_coords: Vec<u32> = (1..=8).rev().collect();

        let display_headers = match color {
            PieceColor::White => self.display_headers_for_white(),
            PieceColor::Black => self.display_headers_for_black(),
        };

        let row_vec: Vec<u32> = match color {
            PieceColor::White => (0..8).collect(),
            PieceColor::Black => (0..8).rev().collect(),
        };

        let col_vec = row_vec.clone();

        let to_move = match self.to_move {
            PieceColor::White => "White to move.",
            PieceColor::Black => "Black to move.",
        };

        let mut output = String::new();

        for i in row_vec.iter() {
            output += &format!("{} |", numeric_coords[*i as usize]);
            for j in col_vec.iter() {
                output += &format!(
                    " {} ",
                    self.match_for_piece(&self.board[*i as usize][*j as usize])
                );
            }
            output += "\n";
        }

        output += "  +-----------------------\n   ";

        for coord in alpha_coords.iter() {
            output += &format!(" {} ", *coord);
        }

        format!(
            "{}\n\
            Position:\n\
            {}\n\
            {}",
            display_headers, output, to_move,
        )
    }

    /// Display the chessboard for `White`.
    pub fn display_for_white(&self) -> String {
        self.display(PieceColor::White)
    }

    /// Display the chessboard for `Black`.
    pub fn display_for_black(&self) -> String {
        self.display(PieceColor::Black)
    }

    /// Return a new instance of `Engine` given a white
    /// `Player` and a black `Player`.
    pub fn new(white: Player, black: Player) -> Engine<'static> {
        Engine {
            white,
            black,
            to_move: PieceColor::White,
            can_white_castle_kingside: true,
            can_white_castle_queenside: true,
            can_black_castle_kingside: true,
            can_black_castle_queenside: true,
            pawn_move_or_piece_capture_half_move_counter: 0,
            half_move_counter: 0,
            move_counter: 0,
            enpassant_target_square: ('-', 0),
            move_generator: MoveGenerator::generate_move_list(),
            board: [
                // rank 8
                [
                    Square {
                        coord: ('a', 8),
                        piece: Piece::Rook(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('b', 8),
                        piece: Piece::Knight(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('c', 8),
                        piece: Piece::Bishop(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('d', 8),
                        piece: Piece::Queen(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('e', 8),
                        piece: Piece::King(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('f', 8),
                        piece: Piece::Bishop(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('g', 8),
                        piece: Piece::Knight(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('h', 8),
                        piece: Piece::Rook(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                ],
                // rank 7
                [
                    Square {
                        coord: ('a', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('b', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('c', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('d', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('e', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('f', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('g', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('h', 7),
                        piece: Piece::Pawn(PieceColor::Black),
                        color: SquareColor::Light,
                    },
                ],
                // rank 6
                [
                    Square {
                        coord: ('a', 6),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('b', 6),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('c', 6),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('d', 6),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('e', 6),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('f', 6),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('g', 6),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('h', 6),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                ],
                // rank 5
                [
                    Square {
                        coord: ('a', 5),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('b', 5),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('c', 5),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('d', 5),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('e', 5),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('f', 5),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('g', 5),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('h', 5),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                ],
                // rank 4
                [
                    Square {
                        coord: ('a', 4),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('b', 4),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('c', 4),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('d', 4),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('e', 4),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('f', 4),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('g', 4),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('h', 4),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                ],
                // rank 3
                [
                    Square {
                        coord: ('a', 3),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('b', 3),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('c', 3),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('d', 3),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('e', 3),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('f', 3),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('g', 3),
                        piece: Piece::None,
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('h', 3),
                        piece: Piece::None,
                        color: SquareColor::Light,
                    },
                ],
                // rank 2
                [
                    Square {
                        coord: ('a', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('b', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('c', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('d', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('e', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('f', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('g', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('h', 2),
                        piece: Piece::Pawn(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                ],
                // rank 1
                [
                    Square {
                        coord: ('a', 1),
                        piece: Piece::Rook(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('b', 1),
                        piece: Piece::Knight(PieceColor::White),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('c', 1),
                        piece: Piece::Bishop(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('d', 1),
                        piece: Piece::Queen(PieceColor::White),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('e', 1),
                        piece: Piece::King(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('f', 1),
                        piece: Piece::Bishop(PieceColor::White),
                        color: SquareColor::Light,
                    },
                    Square {
                        coord: ('g', 1),
                        piece: Piece::Knight(PieceColor::White),
                        color: SquareColor::Dark,
                    },
                    Square {
                        coord: ('h', 1),
                        piece: Piece::Rook(PieceColor::White),
                        color: SquareColor::Light,
                    },
                ],
            ],
        }
    }
}
