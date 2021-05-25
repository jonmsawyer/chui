//! Provides the enums for each chess `Piece` and `Color`.

use std::fmt;
use std::convert::TryFrom;

use crate::{ChuiResult, ChuiError};

/// Piece color. Either `White` or `Black` variants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

/// Piece kind. One of `King`, `Queen`, `Rook`, `Knight`,
/// `Bishop`, `Knight`, `Pawn`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}
/// Represents a piece on the chessboard. Each chess piece has
/// a `PieceKind` and `Color`.
///
/// Example:
///
/// ```
/// use chui::{Piece, Color};
/// 
/// let white_pawn = Piece::Pawn(Color::White);
/// let black_queen = Piece::Queen(Color::Black);
/// 
/// println!("White pawn: {:?}", white_pawn);
/// println!("Black queen: {:?}", black_queen);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    /// The kind of piece.
    piece: PieceKind,

    /// The color of the piece.
    color: Color,
}

impl Piece {
    pub fn new(piece: PieceKind, color: Color) -> Piece {
        Piece {
            piece,
            color,
        }
    }

    /// Get the kind of the piece.
    pub fn get_kind(&self) -> PieceKind {
        self.piece
    }

    /// Get the color of the piece.
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Set the piece kind.
    pub fn set_piece(&mut self, piece: PieceKind) {
        self.piece = piece;
    }

    /// Set the piece color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get the rendered `String` representation of the piece.
    /// E.g., `"White King".to_string()`.
    pub fn get_text(&self) -> String {
        // let color = match self.color {
        //     Color::White => "White",
        //     Color::Black => "Black",
        // };

        // let piece = match self.piece {
        //     PieceKind::King => "King",
        //     PieceKind::Queen => "Queen",

        // }
        // let piece_text = match self {
        //     Piece::King(Color::White) => "White King",
        //     Piece::King(Color::Black) => "Black King",
        //     Piece::Queen(Color::White) => "White Queen",
        //     Piece::Queen(Color::Black) => "Black Queen",
        //     Piece::Rook(Color::White) => "White Rook",
        //     Piece::Rook(Color::Black) => "Black Rook",
        //     Piece::Bishop(Color::White) => "White Bishop",
        //     Piece::Bishop(Color::Black) => "Black Bishop",
        //     Piece::Knight(Color::White) => "White Knight",
        //     Piece::Knight(Color::Black) => "Black Knight",
        //     Piece::Pawn(Color::White) => "White Pawn",
        //     Piece::Pawn(Color::Black) => "Black Pawn",
        // };

        // piece_text.to_string()
        format!("{:?} {:?}", self.color, self.piece)
    }
}

/// Returns a string containing the string representation of the chess piece.
/// (e.g., "P" for a White Pawn.)
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self.piece {
            PieceKind::King => "k",
            PieceKind::Queen => "q",
            PieceKind::Rook => "r",
            PieceKind::Bishop => "b",
            PieceKind::Knight => "n",
            PieceKind::Pawn => "p",
       };

       if let Color::White = self.color {
           write!(f, "{}", piece.to_uppercase())
       }
       else {
           write!(f, "{}", piece)
       }
   }
}

/// Returns a `Piece` given a `&str` if `&str` is one of \[PKQRBNpkqrbn\].
impl TryFrom<&str> for Piece {
    type Error = ChuiError;

    fn try_from(piece: &str) -> ChuiResult<Piece> {
        match piece {
            "K" => Ok(Piece::new(PieceKind::King, Color::White)),
            "Q" => Ok(Piece::new(PieceKind::Queen, Color::White)),
            "R" => Ok(Piece::new(PieceKind::Rook, Color::White)),
            "B" => Ok(Piece::new(PieceKind::Bishop, Color::White)),
            "N" => Ok(Piece::new(PieceKind::Knight, Color::White)),
            "P" => Ok(Piece::new(PieceKind::Pawn, Color::White)),

            "k" => Ok(Piece::new(PieceKind::King, Color::Black)),
            "q" => Ok(Piece::new(PieceKind::Queen, Color::Black)),
            "r" => Ok(Piece::new(PieceKind::Rook, Color::Black)),
            "b" => Ok(Piece::new(PieceKind::Bishop, Color::Black)),
            "n" => Ok(Piece::new(PieceKind::Knight, Color::Black)),
            "p" => Ok(Piece::new(PieceKind::Pawn, Color::Black)),

            _ => Err(
                ChuiError::InvalidPiece(
                    format!(
                        "`{}` is an invalid piece. Expected one of [PRNBQKprnbqk]",
                        piece
                    )
                )
            ),
        }
    }
}

/// Returns a `Piece` given a `char` if `char` is one of \[PKQRBNpkqrbn\].
impl TryFrom<char> for Piece {
    type Error = ChuiError;

    fn try_from(piece: char) -> ChuiResult<Piece> {
        match piece {
            'K' => Ok(Piece::new(PieceKind::King, Color::White)),
            'Q' => Ok(Piece::new(PieceKind::Queen, Color::White)),
            'R' => Ok(Piece::new(PieceKind::Rook, Color::White)),
            'B' => Ok(Piece::new(PieceKind::Bishop, Color::White)),
            'N' => Ok(Piece::new(PieceKind::Knight, Color::White)),
            'P' => Ok(Piece::new(PieceKind::Pawn, Color::White)),

            'k' => Ok(Piece::new(PieceKind::King, Color::Black)),
            'q' => Ok(Piece::new(PieceKind::Queen, Color::Black)),
            'r' => Ok(Piece::new(PieceKind::Rook, Color::Black)),
            'b' => Ok(Piece::new(PieceKind::Bishop, Color::Black)),
            'n' => Ok(Piece::new(PieceKind::Knight, Color::Black)),
            'p' => Ok(Piece::new(PieceKind::Pawn, Color::Black)),

            _ => Err(
                ChuiError::InvalidPiece(
                    format!(
                        "`{}` is an invalid piece. Expected one of [PRNBQKprnbqk]",
                        piece
                    )
                )
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_pieces() {
        assert_eq!("K", &format!("{}", Piece::new(PieceKind::King, Color::White)));
        assert_eq!("Q", &format!("{}", Piece::new(PieceKind::Queen, Color::White)));
        assert_eq!("R", &format!("{}", Piece::new(PieceKind::Rook, Color::White)));
        assert_eq!("B", &format!("{}", Piece::new(PieceKind::Bishop, Color::White)));
        assert_eq!("N", &format!("{}", Piece::new(PieceKind::Knight, Color::White)));
        assert_eq!("P", &format!("{}", Piece::new(PieceKind::Pawn, Color::White)));

        assert_eq!("k", &format!("{}", Piece::new(PieceKind::King, Color::Black)));
        assert_eq!("q", &format!("{}", Piece::new(PieceKind::Queen, Color::Black)));
        assert_eq!("r", &format!("{}", Piece::new(PieceKind::Rook, Color::Black)));
        assert_eq!("b", &format!("{}", Piece::new(PieceKind::Bishop, Color::Black)));
        assert_eq!("n", &format!("{}", Piece::new(PieceKind::Knight, Color::Black)));
        assert_eq!("p", &format!("{}", Piece::new(PieceKind::Pawn, Color::Black)));
    }

    #[test]
    fn valid_try_from_pieces() {
        assert_eq!(Piece::new(PieceKind::King, Color::White), Piece::try_from("K").unwrap());
        assert_eq!(Piece::new(PieceKind::Queen, Color::White), Piece::try_from("Q").unwrap());
        assert_eq!(Piece::new(PieceKind::Rook, Color::White), Piece::try_from("R").unwrap());
        assert_eq!(Piece::new(PieceKind::Bishop, Color::White), Piece::try_from("B").unwrap());
        assert_eq!(Piece::new(PieceKind::Knight, Color::White), Piece::try_from("N").unwrap());
        assert_eq!(Piece::new(PieceKind::Pawn, Color::White), Piece::try_from("P").unwrap());

        assert_eq!(Piece::new(PieceKind::King, Color::Black), Piece::try_from("k").unwrap());
        assert_eq!(Piece::new(PieceKind::Queen, Color::Black), Piece::try_from("q").unwrap());
        assert_eq!(Piece::new(PieceKind::Rook, Color::Black), Piece::try_from("r").unwrap());
        assert_eq!(Piece::new(PieceKind::Bishop, Color::Black), Piece::try_from("b").unwrap());
        assert_eq!(Piece::new(PieceKind::Knight, Color::Black), Piece::try_from("n").unwrap());
        assert_eq!(Piece::new(PieceKind::Pawn, Color::Black), Piece::try_from("p").unwrap());
    }

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
