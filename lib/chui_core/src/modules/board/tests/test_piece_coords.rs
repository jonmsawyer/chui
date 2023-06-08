#[allow(unused_imports)]
use crate::constants::*;
use crate::{Board, ChessVariant, ChuiError, ChuiResult, Color, Coord, Piece, PieceKind};

pub mod piece_coords {
    use super::*;

    fn new_board() -> Board {
        Board::new(ChessVariant::Empty)
    }

    fn print_info(piece: &Piece, coords: &Vec<Coord>) {
        Board::print_piece(piece);
        Board::print_coords(coords);
    }

    fn get_piece(board: &Board, coord: Coord) -> ChuiResult<Piece> {
        board
            .get_piece(coord)
            .ok_or(ChuiError::InvalidPiece(format!(
                "Invalid piece on {}",
                coord
            )))
    }

    fn get_vars(
        piece_kind: PieceKind,
        color: Color,
        coord: (char, u8),
    ) -> ChuiResult<(Board, Piece)> {
        let (mut board, coord) = (new_board(), Coord::try_from(coord)?);
        board.put_piece(Some(Piece::new(piece_kind, color, coord)), coord);
        Ok((board, get_piece(&board, coord)?))
    }

    fn assert_coords(expected_coords: &Vec<(char, u8)>, coords: &Vec<Coord>) -> ChuiResult<()> {
        assert_eq!(expected_coords.len(), coords.len());
        assert!(expected_coords.iter().all(|e_coord| {
            let e_coord = Coord::try_from(*e_coord).unwrap();
            coords.iter().any(|c| e_coord == *c)
        }));
        Ok(())
    }

    #[test]
    fn test_white_rook_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Rook, Color::White, E4)?;
        let expected_coords = vec![E5, E6, E7, E8, F4, G4, H4, E3, E2, E1, D4, C4, B4, A4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_knight_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Knight, Color::White, E4)?;
        let expected_coords = vec![D6, D2, C5, C3, F6, F2, G5, G3];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_bishop_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Bishop, Color::White, E4)?;
        let expected_coords = vec![D5, C6, B7, A8, F3, G2, H1, F5, G6, H7, D3, C2, B1];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_king_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::King, Color::White, E4)?;
        let expected_coords = vec![D5, E5, F5, F4, F3, E3, D3, D4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }
}
