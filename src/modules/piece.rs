//! Provides the enums for each chess `Piece` and `Color`.

use std::fmt;
use std::convert::TryFrom;

use colored::Colorize;

use crate::{ChuiResult, ChuiError};
use super::Board;

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
/// use chui::{Piece, PieceKind, Color};
/// 
/// let white_pawn = Piece::new(PieceKind::Pawn, Color::White);
/// let black_queen = Piece::new(PieceKind::Queen, Color::Black);
/// 
/// println!("{}: {:?}", white_pawn.get_text(), white_pawn);
/// println!("{}: {:?}", black_queen.get_text(), black_queen);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    /// The kind of piece.
    piece: PieceKind,

    /// The color of the piece.
    color: Color,

    /// The coordinates (by index) of the piece.
    coords: (usize, usize),
}

impl Piece {
    //
    // Constructors.
    //

    pub fn new(piece: PieceKind, color: Color) -> Piece {
        Piece {
            piece,
            color,
            coords: (8, 8),
        }
    }

    //
    // Getters.
    //

    /// Get the kind of the piece.
    pub fn get_piece(&self) -> PieceKind {
        self.piece
    }

    /// Get the color of the piece.
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Get the coordinates of the piece.
    pub fn get_coords(&self) -> (usize, usize) {
        self.coords
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

    /// Set the piece coordinates.
    pub fn set_coords(&mut self, file_idx: usize, rank_idx: usize) {
        self.coords = (file_idx, rank_idx)
    }

    /// Get the rendered `String` representation of the piece.
    /// E.g., `"White King".to_string()`.
    pub fn get_text(&self) -> String {
        format!("{:?} {:?}", self.color, self.piece)
    }

    /// Get move coords for piece.
    pub fn get_move_coords(&self, board: &Board) -> Vec<(usize, usize)> {
        let (file_idx, rank_idx) = self.coords;
        println!("Self.coords = {:?}", self.coords);

        let move_coords;

        match self.piece {
            PieceKind::King => {
                move_coords = board.get_king_move_coords(file_idx, rank_idx);
            },

            PieceKind::Queen => {
                move_coords = board.get_queen_move_coords(file_idx, rank_idx);
            },

            PieceKind::Rook => {
                move_coords = board.get_rook_move_coords(file_idx, rank_idx);
            },

            PieceKind::Bishop => {
                move_coords = board.get_bishop_move_coords(file_idx, rank_idx);
            },

            PieceKind::Knight => {
                move_coords = board.get_knight_move_coords(file_idx, rank_idx);
            },

            PieceKind::Pawn => {
                move_coords = board.get_pawn_move_coords(
                    file_idx,
                    rank_idx,
                    self.color
                );
            },
        }

        println!("Move coords: {:?}", move_coords);
        move_coords
    }
}

/// Returns a UTF-8, colored, string containing the string
/// representation of the chess piece. (E.g., yellow "♙"
/// for a White Pawn.)
/// 
/// TODO: Make representation configurable.
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ♔  U+2654  White Chess King
        // ♕  U+2655  White Chess Queen
        // ♖  U+2656  White Chess Rook
        // ♗  U+2657  White Chess Bishop
        // ♘  U+2658  White Chess Knight
        // ♙  U+2659  White Chess Pawn
        // ♚  U+265A  Black Chess King
        // ♛  U+265B  Black Chess Queen
        // ♜  U+265C  Black Chess Rook
        // ♝  U+265D  Black Chess Bishop
        // ♞  U+265E  Black Chess Knight
        // ♟  U+265F  Black Chess Pawn
        match self {
            Piece { piece: PieceKind::King, color: Color::White, .. } => {
                write!(f, "{}", "♔".yellow().bold())
            },
            Piece { piece: PieceKind::Queen, color: Color::White, .. } => {
                write!(f, "{}", "♕".yellow().bold())
            },
            Piece { piece: PieceKind::Rook, color: Color::White, .. } => {
                write!(f, "{}", "♖".yellow().bold())
            },
            Piece { piece: PieceKind::Bishop, color: Color::White, .. } => {
                write!(f, "{}", "♗".yellow().bold())
            },
            Piece { piece: PieceKind::Knight, color: Color::White, .. } => {
                write!(f, "{}", "♘".yellow().bold())
            },
            Piece { piece: PieceKind::Pawn, color: Color::White, .. } => {
                write!(f, "{}", "♙".yellow().bold())
                //write!(f, "{}", "▲".yellow().bold())
            },
            Piece { piece: PieceKind::King, color: Color::Black, .. } => {
                write!(f, "{}", "♚".magenta().bold())
            },
            Piece { piece: PieceKind::Queen, color: Color::Black, .. } => {
                write!(f, "{}", "♛".magenta().bold())
            },
            Piece { piece: PieceKind::Rook, color: Color::Black, .. } => {
                write!(f, "{}", "♜".magenta().bold())
            },
            Piece { piece: PieceKind::Bishop, color: Color::Black, .. } => {
                write!(f, "{}", "♝".magenta().bold())
            },
            Piece { piece: PieceKind::Knight, color: Color::Black, .. } => {
                write!(f, "{}", "♞".magenta().bold())
            },
            Piece { piece: PieceKind::Pawn, color: Color::Black, .. } => {
                write!(f, "{}", "♟".magenta().bold())
                //write!(f, "{}", "▲".magenta().bold())
            },
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
