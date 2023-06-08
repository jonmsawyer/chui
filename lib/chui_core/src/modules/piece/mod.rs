//! Provides the enums for each chess `Piece` and `Color`.

use std::convert::TryFrom;
use std::fmt;

use colored::{ColoredString, Colorize};

use crate::{Board, ChuiError, ChuiResult, Coord};

/// Piece color. Either `White` or `Black` variants.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Color {
    /// Player color White.
    White,

    /// Player color Black.
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => {
                write!(f, "{}", "♔".yellow().bold())
            }
            Color::Black => {
                write!(f, "{}", "♚".magenta().bold())
            }
        }
    }
}

/// Piece kind. One of `Pawn`, `Knight`, `Bishop`, `Rook`, `Queen`, `King`.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PieceKind {
    /// A Pawn.
    Pawn,
    /// A Knight.
    Knight,
    /// A Bishop.
    Bishop,
    /// A Rook.
    Rook,
    /// The Queen.
    Queen,
    /// The King.
    King,
}

/// Represents a piece on the chessboard. Each chess piece has
/// a `PieceKind` and `Color`.
///
/// Example:
///
/// ```
/// use chui::{Piece, PieceKind, Color};
///
/// let white_pawn = Piece::new(PieceKind::Pawn, Color::White);
/// let black_queen = Piece::new(PieceKind::Queen, Color::Black);
///
/// println!("{}: {:?}", white_pawn.get_text(), white_pawn);
/// println!("{}: {:?}", black_queen.get_text(), black_queen);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece {
    /// The kind of piece.
    piece: PieceKind,

    /// The color of the piece.
    color: Color,

    /// The Coordinates (by index) of the piece.
    coord: Coord,

    /// The index of the sprite in the sprite sheet.
    sprite_index: usize,

    /// The maximum amount of squares a piece can move.
    move_max: u8,

    /// Has the piece moved or is it on its initial square?
    on_initial_square: bool,
}

impl Piece {
    //
    // Constructors.
    //

    /// Return a new Piece given a `PieceKind` and a `Color`.
    pub fn new(piece: PieceKind, color: Color, coord: Coord) -> Piece {
        let (sprite_index, move_max, on_initial_square) = match (piece, color) {
            (PieceKind::King, Color::White) => {
                if coord == Coord::new(4, 0).unwrap() {
                    (7, 2, true)
                } else {
                    (7, 2, false)
                }
            }
            (PieceKind::Queen, Color::White) => {
                if coord == Coord::new(3, 0).unwrap() {
                    (6, 7, true)
                } else {
                    (6, 7, false)
                }
            }
            (PieceKind::Rook, Color::White) => {
                if coord == Coord::new(0, 0).unwrap() || coord == Coord::new(7, 0).unwrap() {
                    (5, 7, true)
                } else {
                    (5, 7, false)
                }
            }
            (PieceKind::Bishop, Color::White) => {
                if coord == Coord::new(2, 0).unwrap() || coord == Coord::new(5, 0).unwrap() {
                    (4, 7, true)
                } else {
                    (4, 7, false)
                }
            }
            (PieceKind::Knight, Color::White) => {
                if coord == Coord::new(1, 0).unwrap() || coord == Coord::new(6, 0).unwrap() {
                    (3, 1, true)
                } else {
                    (3, 1, false)
                }
            }
            (PieceKind::Pawn, Color::White) => {
                if coord.get_rank() == 1 {
                    (2, 2, true)
                } else {
                    (2, 2, false)
                }
            }
            (PieceKind::King, Color::Black) => {
                if coord == Coord::new(4, 7).unwrap() {
                    (13, 2, true)
                } else {
                    (13, 2, false)
                }
            }
            (PieceKind::Queen, Color::Black) => {
                if coord == Coord::new(3, 7).unwrap() {
                    (12, 7, true)
                } else {
                    (12, 7, false)
                }
            }
            (PieceKind::Rook, Color::Black) => {
                if coord == Coord::new(0, 7).unwrap() || coord == Coord::new(7, 7).unwrap() {
                    (11, 7, true)
                } else {
                    (11, 7, false)
                }
            }
            (PieceKind::Bishop, Color::Black) => {
                if coord == Coord::new(2, 7).unwrap() || coord == Coord::new(5, 7).unwrap() {
                    (10, 7, true)
                } else {
                    (10, 7, false)
                }
            }
            (PieceKind::Knight, Color::Black) => {
                if coord == Coord::new(1, 7).unwrap() || coord == Coord::new(6, 7).unwrap() {
                    (9, 1, true)
                } else {
                    (9, 1, false)
                }
            }
            (PieceKind::Pawn, Color::Black) => {
                if coord.get_rank() == 6 {
                    (8, 2, true)
                } else {
                    (8, 2, false)
                }
            }
        };

        Piece {
            piece,
            color,
            coord,
            sprite_index,
            move_max,
            on_initial_square,
        }
    }

    //
    // Getters.
    //

    /// Get the kind of the piece.
    pub const fn get_kind(&self) -> PieceKind {
        self.piece
    }

    /// Get the color of the piece.
    pub const fn get_color(&self) -> Color {
        self.color
    }

    /// Get the Coordinates ([`Coord`]) of the piece.
    pub const fn get_coord(&self) -> Coord {
        self.coord
    }

    /// Get the file of this piece.
    pub fn get_file(&self) -> u8 {
        self.coord.get_file()
    }

    /// Get the rank of this piece.
    pub fn get_rank(&self) -> u8 {
        self.coord.get_rank()
    }

    /// Get the sprite index of the piece.
    pub const fn get_sprite_index(&self) -> usize {
        self.sprite_index
    }

    /// Get the maximum number of squares this piece can move.
    pub const fn get_move_max(&self) -> u8 {
        self.move_max
    }

    /// Has the piece moved?
    pub const fn has_moved(&self) -> bool {
        if self.on_initial_square {
            false
        } else {
            true
        }
    }

    /// Are the pieces the same color?
    pub fn is_same_color(&self, piece: Piece) -> bool {
        self.get_color() == piece.get_color()
    }

    /// Are the pieces the same kind?
    pub fn is_same_kind(&self, piece: Piece) -> bool {
        self.get_kind() == piece.get_kind()
    }

    /// Are the pieces the same according to their kind and color?
    pub fn is_same_piece(&self, piece: Piece) -> bool {
        self.is_same_kind(piece) && self.is_same_color(piece)
    }

    //
    // Setters.
    //

    /// Set the piece kind.
    pub fn set_piece(&mut self, piece: PieceKind) {
        self.piece = piece;
    }

    /// Set the piece color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Set the piece Coordinates.
    pub fn set_coord(&mut self, coord: Coord) {
        self.coord = coord;
    }

    /// Set to true if the piece has moved.
    pub fn set_has_moved(&mut self, has_moved: bool) {
        self.on_initial_square = !has_moved;
    }

    /// Get the rendered `String` representation of the piece.
    /// E.g., `"White King".to_string()`.
    pub fn get_text(&self) -> String {
        format!("{:?} {:?}", self.color, self.piece)
    }

    /// Get move Coords for piece.
    pub fn get_move_coords(&self, board: &Board) -> Vec<Coord> {
        let move_coords = match self.piece {
            PieceKind::King => board.get_king_move_coords(self),
            PieceKind::Queen => board.get_queen_move_coords(self),
            PieceKind::Rook => board.get_rook_move_coords(self),
            PieceKind::Bishop => board.get_bishop_move_coords(self),
            PieceKind::Knight => board.get_knight_move_coords(self),
            PieceKind::Pawn => board.get_pawn_move_coords(self),
        };

        move_coords
    }

    /// Return a colored string containing the alpha representation of a piece
    /// and a UTF-8 representation of a piece.
    pub fn repr_colored(&self) -> (ColoredString, ColoredString) {
        match (self.piece, self.color) {
            (PieceKind::King, Color::White) => ("K".yellow().bold(), "♔".yellow().bold()),
            (PieceKind::Queen, Color::White) => ("Q".yellow().bold(), "♕".yellow().bold()),
            (PieceKind::Rook, Color::White) => ("R".yellow().bold(), "♖".yellow().bold()),
            (PieceKind::Bishop, Color::White) => ("B".yellow().bold(), "♗".yellow().bold()),
            (PieceKind::Knight, Color::White) => ("N".yellow().bold(), "♘".yellow().bold()),
            (PieceKind::Pawn, Color::White) => ("P".yellow().bold(), "♙".yellow().bold()),
            (PieceKind::King, Color::Black) => ("k".magenta().bold(), "♚".magenta().bold()),
            (PieceKind::Queen, Color::Black) => ("q".magenta().bold(), "♛".magenta().bold()),
            (PieceKind::Rook, Color::Black) => ("r".magenta().bold(), "♜".magenta().bold()),
            (PieceKind::Bishop, Color::Black) => ("b".magenta().bold(), "♝".magenta().bold()),
            (PieceKind::Knight, Color::Black) => ("n".magenta().bold(), "♞".magenta().bold()),
            (PieceKind::Pawn, Color::Black) => ("p".magenta().bold(), "♟".magenta().bold()),
        }
    }

    /// Return a string containing the alpha representation of a piece
    /// and a UTF-8 representation of a piece.
    pub fn repr(&self) -> (String, String) {
        match (self.piece, self.color) {
            (PieceKind::King, Color::White) => ("K".to_string(), "♔".to_string()),
            (PieceKind::Queen, Color::White) => ("Q".to_string(), "♕".to_string()),
            (PieceKind::Rook, Color::White) => ("R".to_string(), "♖".to_string()),
            (PieceKind::Bishop, Color::White) => ("B".to_string(), "♗".to_string()),
            (PieceKind::Knight, Color::White) => ("N".to_string(), "♘".to_string()),
            (PieceKind::Pawn, Color::White) => ("P".to_string(), "♙".to_string()),
            (PieceKind::King, Color::Black) => ("k".to_string(), "♚".to_string()),
            (PieceKind::Queen, Color::Black) => ("q".to_string(), "♛".to_string()),
            (PieceKind::Rook, Color::Black) => ("r".to_string(), "♜".to_string()),
            (PieceKind::Bishop, Color::Black) => ("b".to_string(), "♝".to_string()),
            (PieceKind::Knight, Color::Black) => ("n".to_string(), "♞".to_string()),
            (PieceKind::Pawn, Color::Black) => ("p".to_string(), "♟".to_string()),
        }
    }
}

/// Returns a UTF-8, colored, string containing the string
/// representation of the chess piece. (E.g., yellow "♙"
/// for a White Pawn.)
///
/// TODO: Make representation configurable.
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.repr().0)
    }
}

/// Returns a `Piece` given a `&str` if `&str` is one of `[PKQRBNpkqrbn]`.
impl TryFrom<&str> for Piece {
    type Error = ChuiError;

    fn try_from(piece: &str) -> ChuiResult<Piece> {
        match piece {
            "K" | "♔" => Ok(Piece::new(
                PieceKind::King,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            "Q" | "♕" => Ok(Piece::new(
                PieceKind::Queen,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            "R" | "♖" => Ok(Piece::new(
                PieceKind::Rook,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            "B" | "♗" => Ok(Piece::new(
                PieceKind::Bishop,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            "N" | "♘" => Ok(Piece::new(
                PieceKind::Knight,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            "P" | "♙" => Ok(Piece::new(
                PieceKind::Pawn,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            "k" | "♚" => Ok(Piece::new(
                PieceKind::King,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            "q" | "♛" => Ok(Piece::new(
                PieceKind::Queen,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            "r" | "♜" => Ok(Piece::new(
                PieceKind::Rook,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            "b" | "♝" => Ok(Piece::new(
                PieceKind::Bishop,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            "n" | "♞" => Ok(Piece::new(
                PieceKind::Knight,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            "p" | "♟" => Ok(Piece::new(
                PieceKind::Pawn,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),

            _ => Err(ChuiError::InvalidPiece(format!(
                "`{}` is an invalid piece. Expected one of [PRNBQKprnbqk]",
                piece
            ))),
        }
    }
}

/// Returns a `Piece` given a `char` if `char` is one of `[PKQRBNpkqrbn]`.
impl TryFrom<char> for Piece {
    type Error = ChuiError;

    fn try_from(piece: char) -> ChuiResult<Piece> {
        match piece {
            'K' => Ok(Piece::new(
                PieceKind::King,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            'Q' => Ok(Piece::new(
                PieceKind::Queen,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            'R' => Ok(Piece::new(
                PieceKind::Rook,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            'B' => Ok(Piece::new(
                PieceKind::Bishop,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            'N' => Ok(Piece::new(
                PieceKind::Knight,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),
            'P' => Ok(Piece::new(
                PieceKind::Pawn,
                Color::White,
                Coord::new(0, 0).unwrap(),
            )),

            'k' => Ok(Piece::new(
                PieceKind::King,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            'q' => Ok(Piece::new(
                PieceKind::Queen,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            'r' => Ok(Piece::new(
                PieceKind::Rook,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            'b' => Ok(Piece::new(
                PieceKind::Bishop,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            'n' => Ok(Piece::new(
                PieceKind::Knight,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),
            'p' => Ok(Piece::new(
                PieceKind::Pawn,
                Color::Black,
                Coord::new(0, 0).unwrap(),
            )),

            _ => Err(ChuiError::InvalidPiece(format!(
                "`{}` is an invalid piece. Expected one of [PRNBQKprnbqk]",
                piece
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn format_pieces() {
    //     assert_eq!(
    //         "K",
    //         format!("{}", Piece::new(PieceKind::King, Color::White)).as_str()
    //     );
    //     assert_eq!(
    //         "Q",
    //         format!("{}", Piece::new(PieceKind::Queen, Color::White)).as_str()
    //     );
    //     assert_eq!(
    //         "R",
    //         format!("{}", Piece::new(PieceKind::Rook, Color::White)).as_str()
    //     );
    //     assert_eq!(
    //         "B",
    //         format!("{}", Piece::new(PieceKind::Bishop, Color::White)).as_str()
    //     );
    //     assert_eq!(
    //         "N",
    //         format!("{}", Piece::new(PieceKind::Knight, Color::White)).as_str()
    //     );
    //     assert_eq!(
    //         "P",
    //         format!("{}", Piece::new(PieceKind::Pawn, Color::White)).as_str()
    //     );

    //     assert_eq!(
    //         "k",
    //         format!("{}", Piece::new(PieceKind::King, Color::Black)).as_str()
    //     );
    //     assert_eq!(
    //         "q",
    //         format!("{}", Piece::new(PieceKind::Queen, Color::Black)).as_str()
    //     );
    //     assert_eq!(
    //         "r",
    //         format!("{}", Piece::new(PieceKind::Rook, Color::Black)).as_str()
    //     );
    //     assert_eq!(
    //         "b",
    //         format!("{}", Piece::new(PieceKind::Bishop, Color::Black)).as_str()
    //     );
    //     assert_eq!(
    //         "n",
    //         format!("{}", Piece::new(PieceKind::Knight, Color::Black)).as_str()
    //     );
    //     assert_eq!(
    //         "p",
    //         format!("{}", Piece::new(PieceKind::Pawn, Color::Black)).as_str()
    //     );
    // }

    // #[test]
    // fn valid_try_from_pieces() {
    //     assert_eq!(
    //         Piece::new(PieceKind::King, Color::White),
    //         Piece::try_from("K").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Queen, Color::White),
    //         Piece::try_from("Q").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Rook, Color::White),
    //         Piece::try_from("R").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Bishop, Color::White),
    //         Piece::try_from("B").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Knight, Color::White),
    //         Piece::try_from("N").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Pawn, Color::White),
    //         Piece::try_from("P").unwrap()
    //     );

    //     assert_eq!(
    //         Piece::new(PieceKind::King, Color::Black),
    //         Piece::try_from("k").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Queen, Color::Black),
    //         Piece::try_from("q").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Rook, Color::Black),
    //         Piece::try_from("r").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Bishop, Color::Black),
    //         Piece::try_from("b").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Knight, Color::Black),
    //         Piece::try_from("n").unwrap()
    //     );
    //     assert_eq!(
    //         Piece::new(PieceKind::Pawn, Color::Black),
    //         Piece::try_from("p").unwrap()
    //     );
    // }

    #[test]
    #[should_panic]
    fn invalid_try_from_piece_l() {
        Piece::try_from("l").expect("invalid piece");
    }

    #[test]
    #[should_panic]
    fn invalid_try_from_piece_j() {
        Piece::try_from("j").expect("invalid piece");
    }

    #[test]
    #[should_panic]
    #[allow(non_snake_case)]
    fn invalid_try_from_piece_T() {
        Piece::try_from("T").expect("invalid piece");
    }
}
