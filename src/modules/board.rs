use crate::{ChuiResult, ChuiError};
use super::{Piece, Color, PieceKind, Move};

const RANKS: usize = 8;
const FILES: usize = 8;

#[derive(Debug)]
pub enum ChessVariant {
    StandardChess,

    //Chess960,
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    board: [[Option<Piece>; FILES]; RANKS],
}

impl Board {
    //
    // Constructors.
    //

    /// Return a new `Board` given a chess variant.
    pub fn new(variant: ChessVariant) -> Board {
        match variant {
            ChessVariant::StandardChess => {
                let mut board = Board { board: Board::new_standard_chess() };

                board.set_coords()
                     .expect("Failed to set coordinates for board.");

                board
            }
        }
    }

    /// New standard chess setup.
    pub fn new_standard_chess() -> [[Option<Piece>; FILES]; RANKS] {
        [
            // rank 1
            Board::standard_row_of_pieces(Color::White),

            // rank 2
            [Some(Piece::new(PieceKind::Pawn, Color::White)); FILES],

            [None; FILES], // rank 3
            [None; FILES], // rank 4
            [None; FILES], // rank 5
            [None; FILES], // rank 6

            // rank 7
            [Some(Piece::new(PieceKind::Pawn, Color::Black)); FILES],

            // rank 8
            Board::standard_row_of_pieces(Color::Black),
        ]
    }

    //
    // Conditionals.
    //

    pub fn apply_move(&mut self, current_move: &Option<Move>)
    -> ChuiResult<()>
    {
        let move_obj = current_move.as_ref().unwrap();

        if move_obj.get_piece().is_none() {
            return Err(
                ChuiError::InvalidMove(
                    "No piece to apply move.".to_string()
                )
            );
        }

        let pieces = self.get_pieces(&mut move_obj.get_piece().unwrap());

        // println!("Pieces: {:?}", pieces);

        let mut pieces_can_move = Vec::<Piece>::new();

        for piece in pieces.iter() {
            if piece.get_move_coords(&self, current_move)
                    .iter()
                    .any(|&coord|
                {
                    coord.0 == move_obj.to_index.0 as usize &&
                    coord.1 == move_obj.to_index.1 as usize
                }
            )
            {
                pieces_can_move.push(*piece);
            }
        }

        // println!("Pieces can move: {:?}", pieces_can_move);

        let (file, rank) = move_obj.to_coord;

        if pieces_can_move.is_empty(){
            Err(
                ChuiError::InvalidMove(
                    format!(
                        "No {} can move to target square {}{}",
                        move_obj.get_piece().unwrap(),
                        file,
                        rank
                    )
                )
            )
        }
        else if pieces_can_move.len() == 1 {
            self.replace_piece(&mut pieces_can_move[0], &move_obj);
            Ok(())
        }
        else {
            Err(
                ChuiError::InvalidMove(
                    format!(
                        "Ambiguous move. More than one piece can \
                        move to target square {}{}",
                        file,
                        rank
                    )
                )
            )
        }
    }

    pub fn replace_piece(&mut self, piece_from: &mut Piece, move_obj: &Move) {
        let (from_file_idx, from_rank_idx) = piece_from.get_coords();
        let (to_file_idx, to_rank_idx) = move_obj.to_index;

        piece_from.set_coords(to_file_idx as usize, to_rank_idx as usize);

        self.board[from_rank_idx][from_file_idx] = None;
        self.board[to_rank_idx as usize][to_file_idx as usize] =
            Some(*piece_from);
    }

    //
    // Getters.
    //

    /// Get a refereance to the board.
    pub fn get_board(&self) -> &[[Option<Piece>; FILES]; RANKS] {
        &self.board
    }

    /// Get the piece in the defined indicies. Remember that
    /// this is index-based, not coordinate-based.
    pub fn get_piece(&self, file_idx: usize, rank_idx: usize) -> Option<Piece> {
        if file_idx >= FILES || rank_idx >= RANKS {
            return None;
        }

        self.board[rank_idx][file_idx]
    }

    /// Get the available `Piece`s for a `Color`.
    pub fn get_pieces(&self, piece: &mut Piece) -> Vec<Piece>
    {
        let mut pieces = Vec::<Piece>::new();

        for (_, rank_arr) in self.board.iter().enumerate() {
            for (_, some_piece) in rank_arr.iter().enumerate() {
                if some_piece.is_some() {
                    let some_piece = some_piece.unwrap();

                    if some_piece.get_piece() == piece.get_piece() &&
                       some_piece.get_color() == piece.get_color()
                    {
                        pieces.push(some_piece);
                    }
                }
            }
        }

        pieces
    }

    //
    // Setters.
    //

    /// Set the coordinates for all `Piece`s.
    pub fn set_coords(&mut self) -> ChuiResult<()>
    {
        for (rank_idx, rank_arr) in self.board.iter_mut().enumerate() {
            for (file_idx, piece) in rank_arr.iter_mut().enumerate() {
                if piece.is_some() {
                    let piece = piece.as_mut().unwrap();
                    piece.set_coords(file_idx, rank_idx);
                }
            }
        }

        Ok(())
    }

    //
    // Piece move coords.
    //

    /// Get a King's available move coordinates.
    pub fn get_king_move_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        coords.extend(
            self.get_top_left_coords(file_idx, rank_idx, &mut 1, current_move)
        );
        coords.extend(self.get_top_coords(file_idx, rank_idx, 1, false));
        coords.extend(
            self.get_top_right_coords(file_idx, rank_idx, &mut 1, current_move)
        );
        coords.extend(
            self.get_right_coords(file_idx, rank_idx, &mut 1, false, current_move)
        );
        coords.extend(
            self.get_bottom_right_coords(file_idx, rank_idx, &mut 1, current_move)
        );
        coords.extend(self.get_bottom_coords(file_idx, rank_idx, 1, false));
        coords.extend(
            self.get_bottom_left_coords(file_idx, rank_idx, &mut 1, current_move)
        );
        coords.extend(
            self.get_left_coords(
                file_idx,
                rank_idx,
                &mut 1,
                false,
                current_move,
            )
        );

        coords
    }

    /// Get a Queen's available move coordinates.
    pub fn get_queen_move_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();
        let mut max = if FILES <= RANKS { RANKS as usize }
                      else { FILES as usize };

        coords.extend(
            self.get_top_left_coords(
                file_idx, rank_idx, &mut max, current_move
            )
        );
        coords.extend(
            self.get_top_coords(
                file_idx, rank_idx, max, false
            )
        );
        coords.extend(
            self.get_top_right_coords(
                file_idx, rank_idx, &mut max, current_move
            )
        );
        coords.extend(
            self.get_right_coords(
                file_idx, rank_idx, &mut max, false, current_move
            )
        );
        coords.extend(
            self.get_bottom_right_coords(
                file_idx, rank_idx, &mut max, current_move
            )
        );
        coords.extend(
            self.get_bottom_coords(
                file_idx, rank_idx, max, false
            )
        );
        coords.extend(
            self.get_bottom_left_coords(
                file_idx, rank_idx, &mut max, current_move
            )
        );
        coords.extend(
            self.get_left_coords(
                file_idx, rank_idx, &mut max, false, current_move
            )
        );

        coords
    }

    /// Get a Rook's available move coordinates.
    pub fn get_rook_move_coords(
        &self,
        file_idx:
        usize,
        rank_idx: usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();
        let mut max = if FILES <= RANKS { RANKS as usize }
                      else { FILES as usize };

        coords.extend(self.get_top_coords(file_idx, rank_idx, max, false));
        coords.extend(
            self.get_right_coords(
                file_idx,
                rank_idx,
                &mut max,
                false,
                current_move,
            )
        );
        coords.extend(self.get_bottom_coords(file_idx, rank_idx, max, false));
        coords.extend(
            self.get_left_coords(
                file_idx,
                rank_idx,
                &mut max,
                false,
                current_move,
            )
        );

        coords
    }

    /// Get a Bishop's available move coordinates.
    pub fn get_bishop_move_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();
        let mut max = if FILES <= RANKS { RANKS as usize }
                      else { FILES as usize };

        coords.extend(
            self.get_top_left_coords(file_idx, rank_idx, &mut max, current_move)
        );
        coords.extend(
            self.get_top_right_coords(file_idx, rank_idx, &mut max, current_move)
        );
        coords.extend(
            self.get_bottom_right_coords(file_idx, rank_idx, &mut max, current_move)
        );
        coords.extend(
            self.get_bottom_left_coords(file_idx, rank_idx, &mut max, current_move)
        );

        coords
    }

    /// Get a Knight's available move coordinates.
    pub fn get_knight_move_coords(&self, file_idx: usize, rank_idx: usize)
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();
        let file_idx = file_idx as isize;
        let rank_idx = rank_idx as isize;

        if file_idx + 1 < FILES as isize &&
           rank_idx + 2 < RANKS as isize &&
           self.get_piece(file_idx as usize + 1, rank_idx as usize + 2)
               .is_none()
        {
            coords.push((file_idx as usize + 1, rank_idx as usize + 2));
        }

        if file_idx + 1 < FILES as isize  &&
           rank_idx - 2 >= 0 &&
           self.get_piece(file_idx as usize + 1, rank_idx as usize - 2)
               .is_none()
        {
            coords.push((file_idx as usize + 1, rank_idx as usize - 2));
        }

        if file_idx > 0 &&
           rank_idx + 2 < RANKS as isize &&
           self.get_piece(file_idx as usize - 1, rank_idx as usize + 2)
               .is_none()
        {
            coords.push((file_idx as usize - 1, rank_idx as usize + 2));
        }

        if file_idx > 0  &&
           rank_idx - 2 >= 0 &&
           self.get_piece(file_idx as usize - 1, rank_idx as usize - 2)
               .is_none()
        {
            coords.push((file_idx as usize - 1, rank_idx as usize - 2));
        }

        if file_idx + 2 < FILES as isize  &&
           rank_idx + 1 < RANKS as isize &&
           self.get_piece(file_idx as usize + 2, rank_idx as usize + 1)
               .is_none()
        {
            coords.push((file_idx as usize + 2, rank_idx as usize + 1));
        }

        if file_idx + 2 < FILES as isize &&
           rank_idx > 0 &&
           self.get_piece(file_idx as usize + 2, rank_idx as usize - 1)
               .is_none()
        {
            coords.push((file_idx as usize + 2, rank_idx as usize - 1));
        }

        if file_idx - 2 >= 0  &&
           rank_idx + 1 < RANKS as isize &&
           self.get_piece(file_idx as usize - 2, rank_idx as usize + 1)
               .is_none()
        {
            coords.push((file_idx as usize - 2, rank_idx as usize + 1));
        }

        if file_idx - 2 >= 0  &&
           rank_idx > 0 &&
           self.get_piece(file_idx as usize -2, rank_idx as usize - 1)
               .is_none()
        {
            coords.push((file_idx as usize - 2, rank_idx as usize - 1));
        }

        coords
    }

    /// Get a Pawn's available move coordinates.
    pub fn get_pawn_move_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        color: Color
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        if let Color::White = color {
            if rank_idx + 1 < RANKS &&
               self.get_piece(file_idx, rank_idx + 1).is_none()
            {
                coords.push((file_idx, rank_idx + 1));
            }

            // Pawn starting rank for White.
            if rank_idx == 1 &&
               self.get_piece(file_idx, rank_idx + 1).is_none() &&
               self.get_piece(file_idx, rank_idx + 2).is_none()
            {
                coords.push((file_idx, rank_idx + 2));
            }
        }
        else {
            if rank_idx - 1 > 0 &&
               self.get_piece(file_idx, rank_idx - 1).is_none()
            {
                coords.push((file_idx, rank_idx - 1));
            }

            // Pawn starting rank for Black.
            if rank_idx == 6 &&
               self.get_piece(file_idx, rank_idx - 1).is_none() &&
               self.get_piece(file_idx, rank_idx - 2).is_none()
            {
                coords.push((file_idx, rank_idx - 2));
            }
        }

        coords
    }

    //
    // Position coords.
    //

    pub fn get_top_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: usize,
        ignore_pieces: bool
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        let mut limit_counter: usize = 0;
        let mut rank_idx_counter = rank_idx + 1;

        while rank_idx_counter < RANKS && limit_counter < limit {
            if ignore_pieces ||
               self.get_piece(file_idx, rank_idx_counter).is_none()
            {
                coords.push((file_idx, rank_idx_counter));
            }
            else {
                let move_coords = self.get_piece(
                    file_idx,
                    rank_idx_counter
                )
                .unwrap()
                .get_file_rank_from_coords(
                    &(file_idx, rank_idx_counter)
                );
                println!(
                    "(Top) Breaking on {}{}",
                    move_coords.0,
                    move_coords.1
                );
                break;
            }

            rank_idx_counter += 1;
            limit_counter += 1;
        }
        
        coords
    }

    pub fn get_right_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: &mut usize,
        ignore_pieces: bool,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        let mut limit_counter: usize = 0;
        let mut file_idx_counter = file_idx + 1;

        if let Some(move_obj) = current_move {
            if let PieceKind::King = move_obj.piece.unwrap().get_piece() {
                *limit = 2;
            }
        }

        while file_idx_counter < FILES && limit_counter < *limit {
            if ignore_pieces ||
               self.get_piece(file_idx_counter, rank_idx).is_none()
            {
                coords.push((file_idx_counter, rank_idx));
            }
            else {
                let move_coords = self.get_piece(
                    file_idx_counter,
                    rank_idx
                )
                .unwrap()
                .get_file_rank_from_coords(
                    &(file_idx_counter, rank_idx)
                );
                println!(
                    "(Right) Breaking on {}{}",
                    move_coords.0,
                    move_coords.1
                );
                break;
            }

            file_idx_counter += 1;
            limit_counter += 1;
        }
        
        coords
    }

    pub fn get_bottom_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: usize,
        ignore_pieces: bool
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        let mut limit_counter: usize = 0;
        let mut rank_idx_counter = rank_idx as isize - 1;

        while rank_idx_counter >= 0 && limit_counter < limit {
            if ignore_pieces ||
               self.get_piece(file_idx, rank_idx_counter as usize).is_none()
            {
                coords.push((file_idx, rank_idx_counter as usize));
            }
            else {
                let move_coords = self.get_piece(
                    file_idx,
                    rank_idx_counter as usize
                )
                .unwrap()
                .get_file_rank_from_coords(
                    &(file_idx, rank_idx_counter as usize)
                );
                println!(
                    "(Bottom) Breaking on {}{}",
                    move_coords.0,
                    move_coords.1
                );
                break;
            }

            rank_idx_counter -= 1;
            limit_counter += 1;
        }
        
        coords
    }

    pub fn get_left_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: &mut usize,
        ignore_pieces: bool,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        let mut limit_counter: usize = 0;
        let mut file_idx_counter = file_idx as isize - 1;

        if let Some(move_obj) = current_move {
            if let PieceKind::King = move_obj.get_piece().unwrap().get_piece()
            {
                *limit = 2;
            }
        }

        while file_idx_counter >= 0 && limit_counter < *limit {
            if ignore_pieces ||
               self.get_piece(file_idx_counter as usize, rank_idx).is_none()
            {
                coords.push((file_idx_counter as usize, rank_idx));
            }
            else {
                let move_coords = self.get_piece(
                    file_idx_counter as usize,
                    rank_idx
                )
                .unwrap()
                .get_file_rank_from_coords(
                    &(file_idx_counter as usize, rank_idx)
                );
                println!(
                    "(Left) Breaking on {}{}",
                    move_coords.0,
                    move_coords.1
                );
                break;
            }

            file_idx_counter -= 1;
            limit_counter += 1;
        }
        
        coords
    }

    pub fn get_top_left_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: &mut usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        Board::zip_top_left_coords(
            self,
            self.get_top_coords(file_idx, rank_idx, *limit, true),
            self.get_left_coords(file_idx, rank_idx, limit, true, current_move)
        )
    }

    pub fn get_top_right_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: &mut usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        Board::zip_top_right_coords(
            self,
            self.get_top_coords(file_idx, rank_idx, *limit, true),
            self.get_right_coords(file_idx, rank_idx, limit, true, current_move)
        )
    }

    pub fn get_bottom_right_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: &mut usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        Board::zip_bottom_right_coords(
            self,
            self.get_bottom_coords(file_idx, rank_idx, *limit, true),
            self.get_right_coords(file_idx, rank_idx, limit, true, current_move)
        )
    }

    pub fn get_bottom_left_coords(
        &self,
        file_idx: usize,
        rank_idx: usize,
        limit: &mut usize,
        current_move: &Option<Move>,
    )
    -> Vec<(usize, usize)>
    {
        Board::zip_bottom_left_coords(
            self,
            self.get_bottom_coords(file_idx, rank_idx, *limit, true),
            self.get_left_coords(file_idx, rank_idx, limit, true, current_move)
        )
    }

    //
    // Utilities.
    //

    /// Produces a row (`[Option<Piece>; FILES]`) of pieces
    /// according their color.
    pub fn standard_row_of_pieces(color: Color) -> [Option<Piece>; FILES] {
        [
            Some(Piece::new(PieceKind::Rook, color)),
            Some(Piece::new(PieceKind::Knight, color)),
            Some(Piece::new(PieceKind::Bishop, color)),
            Some(Piece::new(PieceKind::Queen, color)),
            Some(Piece::new(PieceKind::King, color)),
            Some(Piece::new(PieceKind::Bishop, color)),
            Some(Piece::new(PieceKind::Knight, color)),
            Some(Piece::new(PieceKind::Rook, color)),
        ]
    }

    pub fn zip_top_left_coords(
        &self,
        top_coords: Vec<(usize, usize)>,
        left_coords: Vec<(usize, usize)>
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        for ((_, rank), (file, _)) in top_coords.iter().zip(left_coords) {
            if let Some(piece) = self.get_piece(file, *rank) {
                let move_coords = piece.get_file_rank_from_coords(
                    &(file, *rank)
                );
                println!("(Top Left) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            coords.push((file, *rank));
        }

        coords
    }

    pub fn zip_top_right_coords(
        &self,
        top_coords: Vec<(usize, usize)>,
        right_coords: Vec<(usize, usize)>
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        for ((_, rank), (file, _)) in top_coords.iter().zip(right_coords) {
            if let Some(piece) = self.get_piece(file, *rank) {
                let move_coords = piece.get_file_rank_from_coords(
                    &(file, *rank)
                );
                println!("(Top Right) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            coords.push((file, *rank));
        }

        coords
    }

    pub fn zip_bottom_right_coords(
        &self,
        bottom_coords: Vec<(usize, usize)>,
        right_coords: Vec<(usize, usize)>
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        for ((_, rank), (file, _)) in bottom_coords.iter()
                                                   .zip(right_coords)
        {
            if let Some(piece) = self.get_piece(file, *rank) {
                let move_coords = piece.get_file_rank_from_coords(
                    &(file, *rank)
                );
                println!("(Bottom Right) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            coords.push((file, *rank));
        }

        coords
    }

    pub fn zip_bottom_left_coords(
        &self,
        bottom_coords: Vec<(usize, usize)>,
        left_coords: Vec<(usize, usize)>
    )
    -> Vec<(usize, usize)>
    {
        let mut coords = Vec::<(usize, usize)>::new();

        for ((_, rank), (file, _)) in bottom_coords
                                          .iter()
                                          .zip(left_coords)
        {
            if let Some(piece) = self.get_piece(file, *rank) {
                let move_coords = piece.get_file_rank_from_coords(
                    &(file, *rank)
                );
                println!(
                    "(Bottom Left) Breaking on {}{}",
                    move_coords.0,
                    move_coords.1
                );
                break;
            }

            coords.push((file, *rank));
        }

        coords
    }
    
    /// Test function to display the board colors by a straight
    /// index from `0..64` range.
    /// 
    /// Thanks to Kromey (https://github.com/Kromey).
    pub fn display_board_colors_by_index() {
        for idx in 0..64 {
            let color_id = ((idx / 8) % 2 + idx % 2) % 2;
            print!("{}  ", color_id);

            if (idx + 1) % 8 == 0 {
                println!();
            }
        }
    }
}
