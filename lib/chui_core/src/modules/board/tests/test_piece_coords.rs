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
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_knight_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Knight, Color::White, E4)?;
        let expected_coords = vec![D6, D2, C5, C3, F6, F2, G5, G3];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_bishop_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Bishop, Color::White, E4)?;
        let expected_coords = vec![D5, C6, B7, A8, F3, G2, H1, F5, G6, H7, D3, C2, B1];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_king_on_e1() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::King, Color::White, E1)?;
        let rook1 = Piece::white_rook(A1)?;
        let rook2 = Piece::white_rook(H1)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        let expected_coords = vec![D1, C1, F1, G1, D2, E2, F2];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_queen_on_e1() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Queen, Color::White, E1)?;
        let rook1 = Piece::white_rook(A1)?;
        let rook2 = Piece::white_rook(H1)?;
        let black_pawn = Piece::black_pawn(E7)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(black_pawn), black_pawn.get_coord());
        let expected_coords = vec![
            B1, C1, D1, F1, G1, E2, E3, E4, E5, E6, E7, D2, C3, B4, A5, F2, G3, H4,
        ];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_king_on_e1_invalid_kingside_castle() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::King, Color::White, E1)?;
        let rook1 = Piece::white_rook(A1)?;
        let rook2 = Piece::white_rook(H1)?;
        let black_queen = Piece::black_queen(A6)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(black_queen), black_queen.get_coord());
        let expected_coords = vec![D1, C1, D2, F2];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_king_on_e1_invalid_queenside_castle() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::King, Color::White, E1)?;
        let rook1 = Piece::white_rook(A1)?;
        let rook2 = Piece::white_rook(H1)?;
        let black_queen = Piece::black_queen(H6)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(black_queen), black_queen.get_coord());
        let expected_coords = vec![D1, E2, F1, F2, G1];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_bishop_on_e5() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Bishop, Color::White, E5)?;
        let rook1 = Piece::white_rook(F4)?;
        let rook2 = Piece::white_rook(D6)?;
        let black_queen = Piece::black_queen(G7)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(black_queen), black_queen.get_coord());
        let expected_coords = vec![D4, C3, B2, A1, F6, G7];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_knight_on_a6() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Knight, Color::White, A6)?;
        let rook1 = Piece::white_rook(B8)?;
        let rook2 = Piece::white_rook(D6)?;
        let black_queen = Piece::black_queen(B4)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(black_queen), black_queen.get_coord());
        let expected_coords = vec![C7, C5, B4];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_rook_on_c7() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Rook, Color::White, C7)?;
        let rook = Piece::white_rook(C8)?;
        let knight = Piece::white_knight(C3)?;
        let black_queen = Piece::black_queen(G7)?;
        board.put_piece(Some(rook), rook.get_coord());
        board.put_piece(Some(knight), knight.get_coord());
        board.put_piece(Some(black_queen), black_queen.get_coord());
        let expected_coords = vec![B7, A7, D7, E7, F7, G7, C6, C5, C4];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_a5_en_passant() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Pawn, Color::White, A5)?;
        let black_pawn = Piece::black_pawn(B5)?;
        board.put_piece(Some(black_pawn), black_pawn.get_coord());
        board.set_en_passant(Some(Coord::try_from(B6)?), Some(black_pawn));
        let expected_coords = vec![A6, B6];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_g5_no_en_passant() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Pawn, Color::White, G5)?;
        let black_pawn = Piece::black_pawn(F5)?;
        board.put_piece(Some(black_pawn), black_pawn.get_coord());
        let expected_coords = vec![G6];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_rook_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Rook, Color::Black, E4)?;
        let expected_coords = vec![E5, E6, E7, E8, F4, G4, H4, E3, E2, E1, D4, C4, B4, A4];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_knight_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Knight, Color::Black, E4)?;
        let expected_coords = vec![D6, D2, C5, C3, F6, F2, G5, G3];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_bishop_on_e4() -> ChuiResult<()> {
        let (board, piece) = get_vars(PieceKind::Bishop, Color::Black, E4)?;
        let expected_coords = vec![D5, C6, B7, A8, F3, G2, H1, F5, G6, H7, D3, C2, B1];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_king_on_e8() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::King, Color::Black, E8)?;
        let rook1 = Piece::black_rook(A8)?;
        let rook2 = Piece::black_rook(H8)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        let expected_coords = vec![D8, C8, F8, G8, D7, E7, F7];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_queen_on_e8() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Queen, Color::Black, E8)?;
        let rook1 = Piece::black_rook(A8)?;
        let rook2 = Piece::black_rook(H8)?;
        let white_pawn = Piece::white_pawn(E2)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(white_pawn), white_pawn.get_coord());
        let expected_coords = vec![
            B8, C8, D8, F8, G8, E7, E6, E5, E4, E3, E2, D7, C6, B5, A4, F7, G6, H5,
        ];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_king_on_e8_invalid_queenside_castle() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::King, Color::Black, E8)?;
        let rook1 = Piece::black_rook(A8)?;
        let rook2 = Piece::black_rook(H8)?;
        let white_queen = Piece::white_queen(A6)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(white_queen), white_queen.get_coord());
        let expected_coords = vec![D8, G8, F8, F7, D7, E7];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_king_on_e8_invalid_kingside_castle() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::King, Color::Black, E8)?;
        let rook1 = Piece::black_rook(A8)?;
        let rook2 = Piece::black_rook(H8)?;
        let white_queen = Piece::white_queen(H6)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(white_queen), white_queen.get_coord());
        let expected_coords = vec![D8, F7, D7, E7, C8];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_bishop_on_e5() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Bishop, Color::Black, E5)?;
        let rook1 = Piece::black_rook(F4)?;
        let rook2 = Piece::black_rook(D6)?;
        let white_queen = Piece::white_queen(G7)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(white_queen), white_queen.get_coord());
        let expected_coords = vec![D4, C3, B2, A1, F6, G7];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_knight_on_a6() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Knight, Color::Black, A6)?;
        let rook1 = Piece::black_rook(B8)?;
        let rook2 = Piece::black_rook(D6)?;
        let white_queen = Piece::white_queen(B4)?;
        board.put_piece(Some(rook1), rook1.get_coord());
        board.put_piece(Some(rook2), rook2.get_coord());
        board.put_piece(Some(white_queen), white_queen.get_coord());
        let expected_coords = vec![C7, C5, B4];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_rook_on_c7() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Rook, Color::Black, C7)?;
        let rook = Piece::black_rook(C8)?;
        let knight = Piece::black_knight(C3)?;
        let white_queen = Piece::white_queen(G7)?;
        board.put_piece(Some(rook), rook.get_coord());
        board.put_piece(Some(knight), knight.get_coord());
        board.put_piece(Some(white_queen), white_queen.get_coord());
        let expected_coords = vec![B7, A7, D7, E7, F7, G7, C6, C5, C4];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_a4_en_passant() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Pawn, Color::Black, A4)?;
        let white_pawn = Piece::white_pawn(B4)?;
        board.put_piece(Some(white_pawn), white_pawn.get_coord());
        board.set_en_passant(Some(Coord::try_from(B3)?), Some(white_pawn));
        let expected_coords = vec![A3, B3];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_g4_no_en_passant() -> ChuiResult<()> {
        let (mut board, piece) = get_vars(PieceKind::Pawn, Color::Black, G4)?;
        let white_pawn = Piece::black_pawn(F4)?;
        board.put_piece(Some(white_pawn), white_pawn.get_coord());
        let expected_coords = vec![G3];
        let coords = piece.get_move_coords(&board, None);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }
}
