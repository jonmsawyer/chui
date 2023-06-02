//! Provides the enums for each chess `Piece` and `Color`.

use std::convert::TryFrom;
use std::fmt;

use colored::{ColoredString, Colorize};

use crate::{Board, ChuiError, ChuiResult, Coord, Move};

/// Piece color. Either `White` or `Black` variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Piece kind. One of `King`, `Queen`, `Rook`, `Knight`, `Bishop`, `Knight`, `Pawn`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    /// The King.
    King,

    /// The Queen.
    Queen,

    /// A Rook.
    Rook,

    /// A Bishop.
    Bishop,

    /// A Knight.
    Knight,

    /// A Pawn.
    Pawn,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    /// The kind of piece.
    piece: PieceKind,

    /// The color of the piece.
    color: Color,

    /// The Coordinates (by index) of the piece.
    coord: Coord,

    /// The index of the sprite in the sprite sheet.
    sprite_index: usize,
}

impl Piece {
    //
    // Constructors.
    //

    /// Return a new Piece given a `PieceKind` and a `Color`.
    pub const fn new(piece: PieceKind, color: Color, coord: Coord) -> Piece {
        let sprite_index = match (piece, color) {
            (PieceKind::King, Color::White) => 7,
            (PieceKind::Queen, Color::White) => 6,
            (PieceKind::Rook, Color::White) => 5,
            (PieceKind::Bishop, Color::White) => 4,
            (PieceKind::Knight, Color::White) => 3,
            (PieceKind::Pawn, Color::White) => 2,
            (PieceKind::King, Color::Black) => 13,
            (PieceKind::Queen, Color::Black) => 12,
            (PieceKind::Rook, Color::Black) => 11,
            (PieceKind::Bishop, Color::Black) => 10,
            (PieceKind::Knight, Color::Black) => 9,
            (PieceKind::Pawn, Color::Black) => 8,
        };

        Piece {
            piece,
            color,
            coord,
            sprite_index,
        }
    }

    //
    // Getters.
    //

    /// Get the kind of the piece.
    pub const fn get_piece(&self) -> PieceKind {
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

    /// Get the sprite index of the piece.
    pub const fn get_sprite_index(&self) -> usize {
        self.sprite_index
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

    /// Get the rendered `String` representation of the piece.
    /// E.g., `"White King".to_string()`.
    pub fn get_text(&self) -> String {
        format!("{:?} {:?}", self.color, self.piece)
    }

    /// Get move Coords for piece.
    pub fn get_move_coords(&self, board: &Board, current_move: &Option<Move>) -> Vec<Coord> {
        let move_coords = match self.piece {
            PieceKind::King => board.get_king_move_coords(self.coord, current_move),
            PieceKind::Queen => board.get_queen_move_coords(self.coord, current_move),
            PieceKind::Rook => board.get_rook_move_coords(self.coord, current_move),
            PieceKind::Bishop => board.get_bishop_move_coords(self.coord, current_move),
            PieceKind::Knight => board.get_knight_move_coords(self.coord),
            PieceKind::Pawn => board.get_pawn_move_coords(self.coord, self.color),
        };

        println!("Found {} at {}", self, self.coord);
        println!(" > Move Coords: {:?}", move_coords);

        move_coords
    }

    /// Get the file and rank from board Coordinates.
    pub fn get_file_rank_from_coords(coord: Coord) -> (char, u8) {
        coord.to_char_u8_coord()
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
