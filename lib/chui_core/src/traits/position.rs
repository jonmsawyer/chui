//! Position trait.

use std::fmt;

use crate::prelude::{coord::*, *};

/// trait Position.
pub trait Position: fmt::Display {
    /// Get the piece at the given coordinate.
    fn get_piece(&self, coord: Option<Coord>) -> Option<Piece>;

    // /// Get the available [`Piece`]s for a [`Color`].
    // fn get_pieces(&self, piece: Piece) -> Vec<Piece>;

    /// Take a piece off of the board.
    fn take_piece(&mut self, coord: Option<Coord>) -> Option<Piece> {
        self.put_piece(None, coord)
    }

    /// Put a piece onto the board. Return any piece on the given square if it's occupied
    /// already.
    fn put_piece(&mut self, piece: Option<Piece>, coord: Option<Coord>) -> Option<Piece>;

    /// Replace the given piece from one square to another.
    ///
    /// # Errors
    ///
    /// This method does not error.
    fn replace_piece(
        &mut self,
        mut piece_from: Piece,
        move_obj: &ChessMove,
    ) -> ChuiResult<Option<Piece>> {
        self.take_piece(Some(piece_from.get_coord()));
        piece_from.set_coord(move_obj.to_coord);
        Ok(self.put_piece(Some(piece_from), move_obj.to_coord))
    }

    /// Get all [`Piece`]s attacking a given coordinate.
    fn get_pieces_attacking_coord(
        &self,
        _board: &Board,
        _piece: Piece,
        _coord: Option<Coord>,
    ) -> Vec<Piece> {
        Vec::<Piece>::new()
    }

    //
    // Piece move Coords.
    //

    #[allow(clippy::similar_names)]
    /// Get a King's available move Coordinates.
    ///
    /// # Panics
    ///
    /// This method should never panic because of the use of defined constants.
    fn get_king_move_coords(&self, board: &Board, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(piece));
        coords.extend(self.get_top_coords(piece));
        coords.extend(self.get_top_right_coords(piece));
        coords.extend(self.get_right_coords(piece));
        coords.extend(self.get_bottom_right_coords(piece));
        coords.extend(self.get_bottom_coords(piece));
        coords.extend(self.get_bottom_left_coords(piece));
        coords.extend(self.get_left_coords(piece));

        // Trim the move coordinates which are attacking the King.
        let mut coords: Vec<Coord> = coords
            .into_iter()
            .filter(|&coord| {
                self.get_pieces_attacking_coord(board, *piece, Some(coord))
                    .is_empty()
            })
            .collect();

        // Add any valid castling coordinates
        match piece.get_color() {
            Color::White => {
                if board.white_can_castle_kingside {
                    let d1 = Coord::try_from(D1).ok();
                    let c1 = Coord::try_from(C1).ok();

                    if self.get_piece(d1).is_none() && self.get_piece(c1).is_none() {
                        let d1_pieces = self.get_pieces_attacking_coord(board, *piece, d1);
                        let c1_pieces = self.get_pieces_attacking_coord(board, *piece, c1);

                        if d1_pieces.is_empty() && c1_pieces.is_empty() {
                            coords.extend(vec![c1.unwrap()]);
                        }
                    }
                }

                if board.white_can_castle_queenside {
                    let f1 = Coord::try_from(F1).ok();
                    let g1 = Coord::try_from(G1).ok();

                    if self.get_piece(f1).is_none() && self.get_piece(g1).is_none() {
                        let f1_pieces = self.get_pieces_attacking_coord(board, *piece, f1);
                        let g1_pieces = self.get_pieces_attacking_coord(board, *piece, g1);

                        if f1_pieces.is_empty() && g1_pieces.is_empty() {
                            coords.extend(vec![g1.unwrap()]);
                        }
                    }
                }
            }
            Color::Black => {
                if board.black_can_castle_kingside {
                    let d8 = Coord::try_from(D8).ok();
                    let c8 = Coord::try_from(C8).ok();

                    if self.get_piece(d8).is_none() && self.get_piece(c8).is_none() {
                        let d8_pieces = self.get_pieces_attacking_coord(board, *piece, d8);
                        let c8_pieces = self.get_pieces_attacking_coord(board, *piece, c8);

                        if d8_pieces.is_empty() && c8_pieces.is_empty() {
                            coords.extend(vec![c8.unwrap()]);
                        }
                    }
                }

                if board.black_can_castle_queenside {
                    let f8 = Coord::try_from(F8).ok();
                    let g8 = Coord::try_from(G8).ok();

                    if self.get_piece(f8).is_none() && self.get_piece(g8).is_none() {
                        let f8_pieces = self.get_pieces_attacking_coord(board, *piece, f8);
                        let g8_pieces = self.get_pieces_attacking_coord(board, *piece, g8);

                        if f8_pieces.is_empty() && g8_pieces.is_empty() {
                            coords.extend(vec![g8.unwrap()]);
                        }
                    }
                }
            }
        }

        coords
    }

    /// Get a Queen's available move Coordinates.
    fn get_queen_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(piece));
        coords.extend(self.get_top_coords(piece));
        coords.extend(self.get_top_right_coords(piece));
        coords.extend(self.get_right_coords(piece));
        coords.extend(self.get_bottom_right_coords(piece));
        coords.extend(self.get_bottom_coords(piece));
        coords.extend(self.get_bottom_left_coords(piece));
        coords.extend(self.get_left_coords(piece));

        coords
    }

    /// Get a Rook's available move Coordinates.
    fn get_rook_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_coords(piece));
        coords.extend(self.get_right_coords(piece));
        coords.extend(self.get_bottom_coords(piece));
        coords.extend(self.get_left_coords(piece));

        coords
    }

    /// Get a Bishop's available move Coordinates.
    fn get_bishop_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(piece));
        coords.extend(self.get_top_right_coords(piece));
        coords.extend(self.get_bottom_right_coords(piece));
        coords.extend(self.get_bottom_left_coords(piece));

        coords
    }

    /// Get a Knight's available move Coordinates.
    fn get_knight_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let file_idx = piece.get_file();
        let rank_idx = piece.get_rank();

        if let Ok(n_coord) = Coord::new(file_idx + 1, rank_idx + 2) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 1, rank_idx.wrapping_sub(2)) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(1), rank_idx + 2) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(1), rank_idx.wrapping_sub(2)) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 2, rank_idx + 1) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 2, rank_idx.wrapping_sub(1)) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(2), rank_idx + 1) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(2), rank_idx.wrapping_sub(1)) {
            if let Some(n_piece) = self.get_piece(Some(n_coord)) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        coords
    }

    #[allow(clippy::cognitive_complexity)]
    /// Get a Pawn's available move Coordinates. Also accounts for en passant.
    fn get_pawn_move_coords(&self, board: &Board, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let file_idx = piece.get_file();
        let rank_idx = piece.get_rank();

        if piece.get_color() == Color::White {
            let new_coord_1 = Coord::new(file_idx, rank_idx + 1);
            let new_coord_2 = Coord::new(file_idx, rank_idx + 2);

            if let Ok(new_coord) = new_coord_1 {
                if self.get_piece(Some(new_coord)).is_none() {
                    coords.push(new_coord);
                }
            }

            // Pawn starting rank for White.
            if let Ok(new_coord) = new_coord_1 {
                if let Ok(new_coord2) = new_coord_2 {
                    if rank_idx == 1
                        && self.get_piece(Some(new_coord)).is_none()
                        && self.get_piece(Some(new_coord2)).is_none()
                    {
                        coords.push(new_coord2);
                    }
                }
            }

            let capture_1 = Coord::new(file_idx.wrapping_sub(1), rank_idx + 1);
            let capture_2 = Coord::new(file_idx + 1, rank_idx + 1);

            if let Ok(capture_1) = capture_1 {
                if let Some(o_piece) = self.get_piece(Some(capture_1)) {
                    if o_piece.get_color() == Color::Black {
                        coords.push(capture_1);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_1 == en_passant {
                        coords.push(capture_1);
                    }
                }
            }

            if let Ok(capture_2) = capture_2 {
                if let Some(o_piece) = self.get_piece(Some(capture_2)) {
                    if o_piece.get_color() == Color::Black {
                        coords.push(capture_2);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_2 == en_passant {
                        coords.push(capture_2);
                    }
                }
            }
        } else {
            let new_coord = Coord::new(file_idx, rank_idx.wrapping_sub(1));
            let new_coord2 = Coord::new(file_idx, rank_idx.wrapping_sub(2));

            if let Ok(new_coord) = new_coord {
                if self.get_piece(Some(new_coord)).is_none() {
                    coords.push(new_coord);
                }
            }

            // Pawn starting rank for Black.
            if let Ok(new_coord) = new_coord {
                if let Ok(new_coord2) = new_coord2 {
                    if rank_idx == 6
                        && self.get_piece(Some(new_coord)).is_none()
                        && self.get_piece(Some(new_coord2)).is_none()
                    {
                        coords.push(new_coord2);
                    }
                }
            }

            let capture_1 = Coord::new(file_idx.wrapping_sub(1), rank_idx.wrapping_sub(1));
            let capture_2 = Coord::new(file_idx + 1, rank_idx.wrapping_sub(1));

            if let Ok(capture_1) = capture_1 {
                if let Some(o_piece) = self.get_piece(Some(capture_1)) {
                    if o_piece.get_color() == Color::White {
                        coords.push(capture_1);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_1 == en_passant {
                        coords.push(capture_1);
                    }
                }
            }

            if let Ok(capture_2) = capture_2 {
                if let Some(o_piece) = self.get_piece(Some(capture_2)) {
                    if o_piece.get_color() == Color::White {
                        coords.push(capture_2);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_2 == en_passant {
                        coords.push(capture_2);
                    }
                }
            }
        }

        coords
    }

    //
    // Position Coords.
    //

    /// Get any Coordates North of the indicated indices that a piece can move.
    fn get_top_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(piece.get_file(), rank_idx) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            rank_idx += 1;
        }

        coords
    }

    /// Get any Coordates East of the indicated indices that a piece can move.
    fn get_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file() + 1; // we're moving East

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, piece.get_rank()) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx += 1;
        }

        coords
    }

    /// Get any Coordates South of the indicated indices that a piece can move.
    fn get_bottom_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(piece.get_file(), rank_idx) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            rank_idx = rank_idx.wrapping_sub(1);
        }

        coords
    }

    /// Get any Coordates West of the indicated indices that a piece can move.
    fn get_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, piece.get_rank()) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx = file_idx.wrapping_sub(1);
        }

        coords
    }

    /// Get any Coordates North West of the indicated indices that a piece can move.
    fn get_top_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx = file_idx.wrapping_sub(1);
            rank_idx += 1;
        }

        coords
    }

    /// Get any Coordates North East of the indicated indices that a piece can move.
    fn get_top_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file() + 1; // we're moving East
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx += 1;
            rank_idx += 1;
        }

        coords
    }

    /// Get any Coordates South East of the indicated indices that a piece can move.
    fn get_bottom_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file() + 1; // we're moving East
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx += 1;
            rank_idx = rank_idx.wrapping_sub(1);
        }

        coords
    }

    /// Get any Coordates South West of the indicated indices that a piece can move.
    fn get_bottom_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(Some(new_coord)) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx = file_idx.wrapping_sub(1);
            rank_idx = rank_idx.wrapping_sub(1);
        }

        coords
    }

    /// Return the formatted board for a given `Color` as a `String`.
    ///
    /// # Panics
    ///
    /// Panice if a new [`Coord`] could not be constructed.
    ///
    /// TODO: Mitigate panics.
    fn to_string(&self, color: Color) -> String {
        let alpha_coords: Vec<char> = match color {
            Color::White => ('a'..='h').collect(),
            Color::Black => ('a'..='h').rev().collect(),
        };

        let numeric_coords: Vec<u8> = (1..=8).collect();

        let row_vec: Vec<u8> = match color {
            Color::White => (0..8).rev().collect(),
            Color::Black => (0..8).collect(),
        };

        let col_vec: Vec<u8> = match color {
            Color::White => (0..8).collect(),
            Color::Black => (0..8).rev().collect(),
        };

        let mut output = "╔═════════════════════════╗\n║    ".to_string();

        for coord in alpha_coords.iter() {
            output = format!("{} {}", output, *coord);
        }

        output = format!("{}     ║\n║   ┌─────────────────┐   ║\n", output);

        for i in row_vec.iter() {
            output = format!("{}║ {} │", output, numeric_coords[*i as usize]);
            for j in col_vec.iter() {
                output = self.get_piece(Coord::try_from((*j, *i)).ok()).map_or_else(
                    || format!("{} ·", output),
                    |piece| format!("{} {}", output, piece),
                );
            }
            output = format!("{} │ {} ║\n", output.trim(), numeric_coords[*i as usize]);
        }

        output = format!("{}║   └─────────────────┘   ║\n║    ", output);

        for coord in alpha_coords.iter() {
            output = format!("{} {}", output, *coord);
        }

        output = format!("{} {}", output, "    ║\n╚═════════════════════════╝");

        let output = output.trim();

        format!(
            "Position:\n\
            {}\n",
            output,
        )
    }

    /// Display the chessboard for `White`.
    fn white_to_string(&self) -> String {
        self.to_string(Color::White)
    }

    /// Display the chessboard for `Black`.
    fn black_to_string(&self) -> String {
        self.to_string(Color::Black)
    }
}
