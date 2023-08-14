//! Provides the enums for each chess `Piece` and `Color`.

use std::convert::TryFrom;
use std::fmt;

use colored::{ColoredString, Colorize};

#[allow(clippy::wildcard_imports)]
use crate::{constants::*, Board, ChuiError, ChuiResult, Coord};

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

impl PieceKind {
    /// Get the sprite index of the board asset.
    pub const fn get_sprite_index(&self, color: Color) -> usize {
        match (self, color) {
            (PieceKind::King, Color::Black) => 13,
            (PieceKind::Queen, Color::Black) => 12,
            (PieceKind::Rook, Color::Black) => 11,
            (PieceKind::Bishop, Color::Black) => 10,
            (PieceKind::Knight, Color::Black) => 9,
            (PieceKind::Pawn, Color::Black) => 8,
            (PieceKind::King, Color::White) => 7,
            (PieceKind::Queen, Color::White) => 6,
            (PieceKind::Rook, Color::White) => 5,
            (PieceKind::Bishop, Color::White) => 4,
            (PieceKind::Knight, Color::White) => 3,
            (PieceKind::Pawn, Color::White) => 2,
        }
    }

    /// Get the maximum number of squares this piece kind can move.
    pub const fn get_move_max(&self) -> u8 {
        match self {
            PieceKind::Pawn => 2,
            PieceKind::Knight | PieceKind::King => 1,
            PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => 7,
        }
    }

    /// Return true if the piece kind, with color, if the passed in coord represents an initial
    /// square for that piece.
    pub fn on_initial_square(&self, color: Color, coord: Coord) -> bool {
        match (self, color) {
            (PieceKind::King, Color::White) => coord.eq(&E1),
            (PieceKind::Queen, Color::White) => coord.eq(&D1),
            (PieceKind::Rook, Color::White) => coord.eq(&A1) || coord.eq(&H1),
            (PieceKind::Bishop, Color::White) => coord.eq(&C1) || coord.eq(&F1),
            (PieceKind::Knight, Color::White) => coord.eq(&B1) || coord.eq(&G1),
            (PieceKind::Pawn, Color::White) => coord.get_rank() == 1,
            (PieceKind::King, Color::Black) => coord.eq(&E8),
            (PieceKind::Queen, Color::Black) => coord.eq(&D8),
            (PieceKind::Rook, Color::Black) => coord.eq(&A8) || coord.eq(&H8),
            (PieceKind::Bishop, Color::Black) => coord.eq(&C8) || coord.eq(&F8),
            (PieceKind::Knight, Color::Black) => coord.eq(&B8) || coord.eq(&G8),
            (PieceKind::Pawn, Color::Black) => coord.get_rank() == 6,
        }
    }
}

/// Represents a piece on the chessboard. Each chess piece has
/// a `PieceKind` and `Color`.
///
/// Example:
///
/// ```
/// #[allow(clippy::wildcard_imports)]
/// use chui_core::{Piece, PieceKind, Color, Coord, constants::*};
///
/// // `.unwrap()` should not panic if using our constants.
/// let white_pawn = Piece::new(PieceKind::Pawn, Color::White, Coord::try_from(A2).unwrap());
/// let black_queen = Piece::new(PieceKind::Queen, Color::Black, Coord::try_from(D8).unwrap());
///
/// println!("{}: {:?}", white_pawn.get_text(), white_pawn);
/// println!("{}: {:?}", black_queen.get_text(), black_queen);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece {
    /// The kind of piece.
    kind: PieceKind,

    /// The color of the piece.
    color: Color,

    /// The Coordinates (by index) of the piece.
    coord: Coord,

    /// Has the piece moved or is it on its initial square?
    on_initial_square: bool,
}

impl Piece {
    //
    // Constructors.
    //

    /// Return a new [`Piece`] given a [`PieceKind`], [`Color`], and [`Coord`].
    pub fn new(kind: PieceKind, color: Color, coord: Coord) -> Piece {
        Piece {
            kind,
            color,
            coord,
            on_initial_square: kind.on_initial_square(color, coord),
        }
    }

    /// Return a new White Pawn on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn white_pawn(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Pawn,
            Color::White,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new White Knight on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn white_knight(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Knight,
            Color::White,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new White Bishop on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn white_bishop(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Bishop,
            Color::White,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new White Rook on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn white_rook(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Rook,
            Color::White,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new White Queen on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn white_queen(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Queen,
            Color::White,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new White King on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn white_king(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::King,
            Color::White,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new Black Pawn on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn black_pawn(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Pawn,
            Color::Black,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new Black Knight on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn black_knight(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Knight,
            Color::Black,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new Black Bishop on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn black_bishop(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Bishop,
            Color::Black,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new Black Rook on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn black_rook(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Rook,
            Color::Black,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new Black Queen on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn black_queen(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::Queen,
            Color::Black,
            Coord::try_from(coord)?,
        ))
    }

    /// Return a new Black King on a given (char, u8) coord.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if a new [`Coord`] could not be constructed.
    pub fn black_king(coord: (char, u8)) -> ChuiResult<Piece> {
        Ok(Piece::new(
            PieceKind::King,
            Color::Black,
            Coord::try_from(coord)?,
        ))
    }

    //
    // Getters.
    //

    /// Get the kind of the piece.
    pub const fn get_kind(&self) -> PieceKind {
        self.kind
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
    pub const fn get_file(&self) -> u8 {
        self.coord.get_file()
    }

    /// Get the rank of this piece.
    pub const fn get_rank(&self) -> u8 {
        self.coord.get_rank()
    }

    /// Get the sprite index of the piece.
    pub const fn get_sprite_index(&self) -> usize {
        self.kind.get_sprite_index(self.get_color())
    }

    /// Get the maximum number of squares this piece can move.
    pub const fn get_move_max(&self) -> u8 {
        self.kind.get_move_max()
    }

    /// Has the piece moved?
    pub const fn has_moved(&self) -> bool {
        !self.on_initial_square
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
        self.kind = piece;
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
        format!("{:?} {:?}", self.color, self.kind)
    }

    /// Get move Coords for piece.
    pub fn get_move_coords(&self, board: &Board) -> Vec<Coord> {
        match self.kind {
            PieceKind::King => board.get_king_move_coords(self),
            PieceKind::Queen => board.get_queen_move_coords(self),
            PieceKind::Rook => board.get_rook_move_coords(self),
            PieceKind::Bishop => board.get_bishop_move_coords(self),
            PieceKind::Knight => board.get_knight_move_coords(self),
            PieceKind::Pawn => board.get_pawn_move_coords(self),
        }
    }

    /// Return a colored string containing the alpha representation of a piece
    /// and a UTF-8 representation of a piece.
    pub fn repr_colored(&self) -> (ColoredString, ColoredString) {
        match (self.kind, self.color) {
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
        match (self.kind, self.color) {
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
        write!(f, "{}", self.repr_colored().0)
    }
}

/// Returns a `Piece` given a `&str` if `&str` is one of `[PKQRBNpkqrbn]`.
impl TryFrom<&str> for Piece {
    type Error = ChuiError;

    fn try_from(piece: &str) -> ChuiResult<Piece> {
        match piece {
            "K" | "♔" => Piece::white_king(E1),
            "Q" | "♕" => Piece::white_queen(D1),
            "R" | "♖" => Piece::white_rook(A1),
            "B" | "♗" => Piece::white_bishop(C1),
            "N" | "♘" => Piece::white_knight(B1),
            "P" | "♙" => Piece::white_pawn(A2),

            "k" | "♚" => Piece::black_king(E8),
            "q" | "♛" => Piece::black_queen(D8),
            "r" | "♜" => Piece::black_rook(A8),
            "b" | "♝" => Piece::black_bishop(C8),
            "n" | "♞" => Piece::black_knight(B8),
            "p" | "♟" => Piece::black_pawn(A7),

            _ => Err(ChuiError::InvalidPiece(format!(
                "`{}` is an invalid piece. Expected one of [PRNBQKprnbqk♙♘♗♖♕♔♟♞♝♜♛♚]",
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
            'K' | '♔' => Piece::white_king(E1),
            'Q' | '♕' => Piece::white_queen(D1),
            'R' | '♖' => Piece::white_rook(A1),
            'B' | '♗' => Piece::white_bishop(C1),
            'N' | '♘' => Piece::white_knight(B1),
            'P' | '♙' => Piece::white_pawn(A2),

            'k' | '♚' => Piece::black_king(E8),
            'q' | '♛' => Piece::black_queen(D8),
            'r' | '♜' => Piece::black_rook(A8),
            'b' | '♝' => Piece::black_bishop(C8),
            'n' | '♞' => Piece::black_knight(B8),
            'p' | '♟' => Piece::black_pawn(A7),

            _ => Err(ChuiError::InvalidPiece(format!(
                "`{}` is an invalid piece. Expected one of [PRNBQKprnbqk♙♘♗♖♕♔♟♞♝♜♛♚]",
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
