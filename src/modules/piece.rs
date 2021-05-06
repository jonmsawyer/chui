//! Provides the enums for each chess `Piece` and `Color`.

use std::fmt;
use std::convert::TryFrom;

use crate::ChuiError;

/// Piece color. Either `White` or `Black` variants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

/// Represents a piece on the chessboard. One of `King`, `Queen`,
/// `Rook`, `Knight`, `Bishop`, `Knight`, `Pawn`, or `None`.
///
/// Each chess piece has a `Color`.
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
pub enum Piece {
    /// A king with its `Color`.
    King(Color),

    /// A queen with its `Color`.
    Queen(Color),

    /// A rook with its `Color`.
    Rook(Color),

    /// A bishop with its `Color`.
    Bishop(Color),

    /// A knight with its `Color`.
    Knight(Color),

    /// A pawn with its `Color`.
    Pawn(Color),
}

/// Returns a string containing the string representation of the chess piece.
/// (e.g., "P" for a White Pawn.)
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn(Color::White) => write!(f, "P"),
            Piece::Rook(Color::White) => write!(f, "R"),
            Piece::Knight(Color::White) => write!(f, "N"),
            Piece::Bishop(Color::White) => write!(f, "B"),
            Piece::Queen(Color::White) => write!(f, "Q"),
            Piece::King(Color::White) => write!(f, "K"),
            Piece::Pawn(Color::Black) => write!(f, "p"),
            Piece::Rook(Color::Black) => write!(f, "r"),
            Piece::Knight(Color::Black) => write!(f, "n"),
            Piece::Bishop(Color::Black) => write!(f, "b"),
            Piece::Queen(Color::Black) => write!(f, "q"),
            Piece::King(Color::Black) => write!(f, "k"),
       }
   }
}

/// Returns a `Piece` given a `&str` if `&str` is one of \[PKQRBNpkqrbn\].
/// May never be needed within the crate, but here for convenience if
/// necessary.
impl TryFrom<&str> for Piece {
    type Error = ChuiError;

    fn try_from(piece: &str) -> crate::Result<Piece> {
        match piece {
            "P" => Ok(Piece::Pawn(Color::White)),
            "R" => Ok(Piece::Rook(Color::White)),
            "N" => Ok(Piece::Knight(Color::White)),
            "B" => Ok(Piece::Bishop(Color::White)),
            "Q" => Ok(Piece::Queen(Color::White)),
            "K" => Ok(Piece::King(Color::White)),
            "p" => Ok(Piece::Pawn(Color::Black)),
            "r" => Ok(Piece::Rook(Color::Black)),
            "n" => Ok(Piece::Knight(Color::Black)),
            "b" => Ok(Piece::Bishop(Color::Black)),
            "q" => Ok(Piece::Queen(Color::Black)),
            "k" => Ok(Piece::King(Color::Black)),
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
        assert_eq!("P", &format!("{}", Piece::Pawn(Color::White)));
        assert_eq!("K", &format!("{}", Piece::King(Color::White)));
        assert_eq!("Q", &format!("{}", Piece::Queen(Color::White)));
        assert_eq!("R", &format!("{}", Piece::Rook(Color::White)));
        assert_eq!("B", &format!("{}", Piece::Bishop(Color::White)));
        assert_eq!("N", &format!("{}", Piece::Knight(Color::White)));
        assert_eq!("p", &format!("{}", Piece::Pawn(Color::Black)));
        assert_eq!("k", &format!("{}", Piece::King(Color::Black)));
        assert_eq!("q", &format!("{}", Piece::Queen(Color::Black)));
        assert_eq!("r", &format!("{}", Piece::Rook(Color::Black)));
        assert_eq!("b", &format!("{}", Piece::Bishop(Color::Black)));
        assert_eq!("n", &format!("{}", Piece::Knight(Color::Black)));
    }

    #[test]
    fn valid_try_from_pieces() {
        assert_eq!(Piece::Pawn(Color::White), Piece::try_from("P").unwrap());
        assert_eq!(Piece::King(Color::White), Piece::try_from("K").unwrap());
        assert_eq!(Piece::Queen(Color::White), Piece::try_from("Q").unwrap());
        assert_eq!(Piece::Rook(Color::White), Piece::try_from("R").unwrap());
        assert_eq!(Piece::Bishop(Color::White), Piece::try_from("B").unwrap());
        assert_eq!(Piece::Knight(Color::White), Piece::try_from("N").unwrap());
        assert_eq!(Piece::Pawn(Color::Black), Piece::try_from("p").unwrap());
        assert_eq!(Piece::King(Color::Black), Piece::try_from("k").unwrap());
        assert_eq!(Piece::Queen(Color::Black), Piece::try_from("q").unwrap());
        assert_eq!(Piece::Rook(Color::Black), Piece::try_from("r").unwrap());
        assert_eq!(Piece::Bishop(Color::Black), Piece::try_from("b").unwrap());
        assert_eq!(Piece::Knight(Color::Black), Piece::try_from("n").unwrap());
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
