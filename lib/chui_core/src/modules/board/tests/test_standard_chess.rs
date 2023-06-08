#[allow(unused_imports)]
use crate::constants::*;
use crate::{Board, ChuiError, ChuiResult, Coord, Piece};

pub mod standard_chess {
    use super::*;

    fn new_board() -> Board {
        Board::default()
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

    fn get_vars(coord: (char, u8)) -> ChuiResult<(Board, Piece)> {
        let (board, coord) = (new_board(), Coord::try_from(coord)?);
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
    fn test_white_rook_on_a1() -> ChuiResult<()> {
        let (board, piece) = get_vars(A1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_knight_on_b1() -> ChuiResult<()> {
        let (board, piece) = get_vars(B1)?;
        let expected_coords = vec![A3, C3];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_bishop_on_c1() -> ChuiResult<()> {
        let (board, piece) = get_vars(C1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_queen_on_d1() -> ChuiResult<()> {
        let (board, piece) = get_vars(D1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_king_on_e1() -> ChuiResult<()> {
        let (board, piece) = get_vars(E1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_bishop_on_f1() -> ChuiResult<()> {
        let (board, piece) = get_vars(F1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_knight_on_g1() -> ChuiResult<()> {
        let (board, piece) = get_vars(G1)?;
        let expected_coords = vec![F3, H3];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_rook_on_h1() -> ChuiResult<()> {
        let (board, piece) = get_vars(H1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_a2() -> ChuiResult<()> {
        let (board, piece) = get_vars(A2)?;
        let expected_coords = vec![A3, A4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_b2() -> ChuiResult<()> {
        let (board, piece) = get_vars(B2)?;
        let expected_coords = vec![B3, B4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_c2() -> ChuiResult<()> {
        let (board, piece) = get_vars(C2)?;
        let expected_coords = vec![C3, C4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_d2() -> ChuiResult<()> {
        let (board, piece) = get_vars(D2)?;
        let expected_coords = vec![D3, D4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_e2() -> ChuiResult<()> {
        let (board, piece) = get_vars(E2)?;
        let expected_coords = vec![E3, E4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_f2() -> ChuiResult<()> {
        let (board, piece) = get_vars(F2)?;
        let expected_coords = vec![F3, F4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_g2() -> ChuiResult<()> {
        let (board, piece) = get_vars(G2)?;
        let expected_coords = vec![G3, G4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_h2() -> ChuiResult<()> {
        let (board, piece) = get_vars(H2)?;
        let expected_coords = vec![H3, H4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_rook_on_a8() -> ChuiResult<()> {
        let (board, piece) = get_vars(A8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_knight_on_b8() -> ChuiResult<()> {
        let (board, piece) = get_vars(B8)?;
        let expected_coords = vec![A6, C6];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_bishop_on_c8() -> ChuiResult<()> {
        let (board, piece) = get_vars(C8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_queen_on_d8() -> ChuiResult<()> {
        let (board, piece) = get_vars(D8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_king_on_e8() -> ChuiResult<()> {
        let (board, piece) = get_vars(E8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_bishop_on_f8() -> ChuiResult<()> {
        let (board, piece) = get_vars(F8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_knight_on_g8() -> ChuiResult<()> {
        let (board, piece) = get_vars(G8)?;
        let expected_coords = vec![F6, H6];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_rook_on_h8() -> ChuiResult<()> {
        let (board, piece) = get_vars(H8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_a7() -> ChuiResult<()> {
        let (board, piece) = get_vars(A7)?;
        let expected_coords = vec![A6, A5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_b7() -> ChuiResult<()> {
        let (board, piece) = get_vars(B7)?;
        let expected_coords = vec![B6, B5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_c7() -> ChuiResult<()> {
        let (board, piece) = get_vars(C7)?;
        let expected_coords = vec![C6, C5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_d7() -> ChuiResult<()> {
        let (board, piece) = get_vars(D7)?;
        let expected_coords = vec![D6, D5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_e7() -> ChuiResult<()> {
        let (board, piece) = get_vars(E7)?;
        let expected_coords = vec![E6, E5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_f7() -> ChuiResult<()> {
        let (board, piece) = get_vars(F7)?;
        let expected_coords = vec![F6, F5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_g7() -> ChuiResult<()> {
        let (board, piece) = get_vars(G7)?;
        let expected_coords = vec![G6, G5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_h7() -> ChuiResult<()> {
        let (board, piece) = get_vars(H7)?;
        let expected_coords = vec![H6, H5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_empty_squares() -> ChuiResult<()> {
        let board = new_board();
        for i in 2..6 {
            for j in 0..8 {
                assert_eq!(None, board.get_piece(Coord::try_from((j, i))?));
            }
        }
        Ok(())
    }
}
