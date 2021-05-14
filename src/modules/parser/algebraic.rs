#![allow(clippy::new_ret_no_self)]

//use std::fmt;
use std::convert::TryFrom;
// use std::collections::HashMap;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::{Move, MoveType, Piece, MoveGenerator, Color, Engine};

/// A parser that will parse algebraic chess notation.
/// Example moves: `e4`, `Bxc6+`, `Kd6`, `e8Q#`, `a1=N`, etc.
pub struct AlgebraicParser;

impl Parser for AlgebraicParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    /// 
    /// Examples:
    /// 
    /// * e4
    /// * e4+
    /// * e4#
    /// * e8Q
    /// * Bf4
    /// * 0-0
    /// * e4++
    /// * e8Q+
    /// * Bf4+
    /// * Bf4#
    /// * e8Q#
    /// * e8=Q
    /// * exf4
    /// * Bxf4
    /// * 0-0+
    /// * 0-0#
    /// * e8Q++
    /// * e8=Q+
    /// * e8=Q#
    /// * exf4+
    /// * exf4#
    /// * exf8Q
    /// * Bf4++
    /// * Bxf4+
    /// * Bxf4#
    /// * 0-0++
    /// * 0-0-0
    /// * e8=Q++
    /// * exf4++
    /// * exf8Q+
    /// * exf8Q#
    /// * exf8=Q
    /// * Bxf4++
    /// * 0-0-0+
    /// * 0-0-0#
    /// * exf8=Q#
    /// * exf8=Q+
    /// * exf8Q++
    /// * 0-0-0++
    /// * exf8=Q++
    /// 
    ///     Token 1: e, B, 0 (
    ///         file, piece, castle king
    ///     )
    /// 
    ///     Token 2: f, 4, x, - (
    ///         file, rank, capture, castle king
    ///     )
    /// 
    ///     Token 3: f, 4, +, #, Q, =, 0 (
    ///         file, rank, check, mate, promotiom piece,
    ///         promotion notatiomn, castle king
    ///     )
    /// 
    ///     Token 4: 4, +, #, Q, - (
    ///         rank, check, mate, promotion piece, castle queen
    ///     )
    /// 
    ///     Token 5: +, #, Q, =, 0 (
    ///         check, mate, promotion piece, promotion notation, castle queen
    ///     )
    /// 
    ///     Token 6: +, #, Q (
    ///         check, mate, promotion piece
    ///     )
    /// 
    ///     Token 7: +, # (
    ///         check, mate
    ///     )
    /// 
    ///     Token 8: + (
    ///         check (mate)
    ///     )
    fn parse(&self, the_move: &str, _engine: &Engine)
    -> ChuiResult<Move>
    {
        println!("Parsing: `{}` with AlgebraicParser...", the_move);

        // Check the move to see it's valid. No whitespace allowed. At
        // the same time, trim any surrounding whitespace.
        let the_move = self.trim_and_check_whitespace(the_move)?;

        if the_move.len() < 2 || the_move.len() > 8 {
            return Err(
                ChuiError::InvalidMove(
                    "Input move is either too small or too large in length"
                        .to_string()
                )
            );
        }
        let mut move_obj = Move::new();

        // Record the input move.
        move_obj.input_move = the_move.to_string();

        for (move_idx, token) in the_move.chars().enumerate() {
            println!("\n > move_idx = {}, token = `{}`", move_idx, token);

            match move_idx {
                0 => self.process_token_1(token, &mut move_obj)?,
                1 => self.process_token_2(token, &mut move_obj)?,
                2 => self.process_token_3(token, &mut move_obj)?,
                3 => self.process_token_4(token, &mut move_obj)?,
                4 => self.process_token_5(token, &mut move_obj)?,
                5 => self.process_token_6(token, &mut move_obj)?,
                6 => self.process_token_7(token, &mut move_obj)?,
                7 => self.process_token_8(token, &mut move_obj)?,
                _ => {
                    println!(" >> Move index `{}` not implemented", move_idx);
                    break;
                },
            }

            continue;
        }

        Ok(move_obj)
    }
}

impl AlgebraicParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(AlgebraicParser { })
    }

    fn token_not_satisfied(token: char) -> ChuiError {
        ChuiError::TokenNotSatisfied(
            format!("`{}` token is not satisfied", token)
        )
    }

    fn try_piece(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        if let Ok(piece) = Piece::try_from(&*format!("{}", token)) {
            if let Piece::King(Color::White) |
                   Piece::Queen(Color::White) |
                   Piece::Rook(Color::White) |
                   Piece::Bishop(Color::White) |
                   Piece::Knight(Color::White) |
                   Piece::Pawn(Color::White) = piece
            {
                println!(" >> Found piece: {:?}", piece);

                move_obj.piece = Some(piece);
                move_obj.move_type = Some(MoveType::PieceMove);

                return Ok(());
            }
        }

        for castle_notation in g.castle_notation.iter() {
            if &&*format!("{}", token) == castle_notation {
                println!(" >> Found piece: King for castle");

                move_obj.piece = Some(Piece::King(Color::White));
                move_obj.move_type = Some(MoveType::Castle);
                move_obj.is_castling = true;
                move_obj.is_castling_king = true;

                return Ok(());
            }
        }

        println!(
            " >> No piece (move must be a pawn move), assigning White Pawn"
        );

        move_obj.piece = Some(Piece::Pawn(Color::White));
        move_obj.move_type = Some(MoveType::PawnMove);

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_file(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        if let Some(index) = self.match_file_to_index(token) {
            println!(" >> to_coord.0 = {}", token);
            move_obj.to_coord = (
                token,
                move_obj.to_coord.1
            );

            println!(" >> to_index.0 = {}", index);
            move_obj.to_index = (
                index,
                move_obj.to_index.1
            );

            return Ok(());
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_capture(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        if format!("{}", token) == *g.capture.to_string() {
            println!(" >> Found capture move");

            move_obj.move_type = match move_obj.move_type {
                Some(MoveType::PawnMove) => Some(MoveType::PawnCapture),
                Some(MoveType::PieceMove) => Some(MoveType::PieceCapture),
                move_type => {
                    return Err(
                        ChuiError::InvalidMove(
                            format!(
                                "`{:?}` move type is invalid for capture",
                                move_type
                            )
                        )
                    );
                },
            };

            return Ok(());
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_rank(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        if let Some(index) = self.match_rank_to_index(token) {
            println!(" >> to_coord.1 = {}", index + 1);
            move_obj.to_coord = (
                move_obj.to_coord.0,
                index + 1,
            );

            println!(" >> to_index.1 = {}", index);
            move_obj.to_index = (
                move_obj.to_index.0,
                index
            );

            return Ok(());
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_check(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        if format!("{}", token) == *g.check.to_string() {
            if move_obj.check {
                println!(" >> Found check mate");

                move_obj.check = false;
                move_obj.check_mate = true;
            }
            else {
                println!(" >> Found check");

                move_obj.check = true;
                move_obj.check_mate = false;
            }

            return Ok(());
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_check_mate(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        for check_mate in g.check_mate.iter() {
            if format!("{}", token) == *check_mate.to_string() {
                println!(" >> Found check mate");

                move_obj.check = false;
                move_obj.check_mate = true;

                return Ok(());
            }
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_promotion_notation(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        for notation in g.promotion_notation.iter() {
            if format!("{}", token) == *notation.to_string() {
                println!(" >> Found promotion notation: {:?}", notation);
                move_obj.promotion = true;

                return Ok(());
            }
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_promotion_piece(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        if let Ok(piece) = Piece::try_from(&*format!("{}", token)) {
            if let Piece::King(Color::White) |
                   Piece::Queen(Color::White) |
                   Piece::Rook(Color::White) |
                   Piece::Bishop(Color::White) |
                   Piece::Knight(Color::White) = piece
            {
                println!(" >> Found promotion piece: {:?}", piece);

                move_obj.promotion = true;
                move_obj.promotion_piece = Some(piece);

                return Ok(());
            }
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_castle_king(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        for castle in g.castle_notation.iter() {
            if format!("{}", token) == *castle.to_string() &&
               move_obj.is_castling
            {
                println!(" >> Found castle king side");

                move_obj.is_castling_king = true;
                move_obj.is_castling_queen = false;

                return Ok(());
            }
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_castle_king_continuation(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        if format!("{}", token) == *g.move_notation.to_string() &&
           move_obj.is_castling &&
           move_obj.is_castling_king
        {
            println!(" >> Found castle king side continuation");

            return Ok(());
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_castle_queen_continuation(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        if format!("{}", token) == *g.move_notation.to_string() &&
           move_obj.is_castling &&
           move_obj.is_castling_king
        {
            println!(" >> Found castle queen side continuation");

            move_obj.is_castling_king = false;
            move_obj.is_castling_queen = true;

            return Ok(());
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    fn try_castle_queen(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        let g = MoveGenerator::new();

        for castle in g.castle_notation.iter() {
            if format!("{}", token) == *castle.to_string() &&
               move_obj.is_castling &&
               !move_obj.is_castling_king &&
               move_obj.is_castling_queen
            {
                println!(" >> Found castle queen side");

                return Ok(());
            }
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    /// * e4
    /// * e4+
    /// * e4#
    /// * e8Q
    /// * e4++
    /// * e8Q+
    /// * e8Q#
    /// * e8=Q
    /// * exf4
    /// * e8Q++
    /// * e8=Q+
    /// * e8=Q#
    /// * exf4+
    /// * exf4#
    /// * exf8Q
    /// * e8=Q++
    /// * exf4++
    /// * exf8Q+
    /// * exf8Q#
    /// * exf8=Q
    /// * exf8=Q#
    /// * exf8=Q+
    /// * exf8Q++
    /// * exf8=Q++
    /// * Bf4
    /// * Bf4+
    /// * Bf4#
    /// * Bxf4
    /// * Bf4++
    /// * Bxf4+
    /// * Bxf4#
    /// * Bxf4++
    /// * 0-0
    /// * 0-0+
    /// * 0-0#
    /// * 0-0++
    /// * 0-0-0
    /// * 0-0-0+
    /// * 0-0-0#
    /// * 0-0-0++
    /// 
    ///     Token 1: e, B, 0 (
    ///         file, piece, castle king
    ///     )
    fn process_token_1(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * Bf4
        // * Bf4+
        // * Bf4++
        // * Bf4#
        // * Bxf4
        // * Bxf4+
        // * Bxf4++
        // * Bxf4#
        // * 0-0
        // * 0-0+
        // * 0-0++
        // * 0-0#
        // * 0-0-0
        // * 0-0-0+
        // * 0-0-0++
        // * 0-0-0#
        // Try to parse the first token as a `Piece`. All pieces
        // will parse as a `White` piece. The first valid piece
        // from the move string gets registered as the piece to
        // move.
        if self.try_piece(token, move_obj).is_ok() {
            return Ok(());
        }

        // * e4
        // * e4+
        // * e4++
        // * e4#
        // * e8Q
        // * e8Q+
        // * e8Q++
        // * e8Q#
        // * e8=Q
        // * e8=Q+
        // * e8=Q++
        // * e8=Q#
        // * exf4
        // * exf4+
        // * exf4++
        // * exf4#
        // * exf8Q
        // * exf8Q+
        // * exf8Q++
        // * exf8Q#
        // * exf8=Q
        // * exf8=Q+
        // * exf8=Q++
        // * exf8=Q#
        // A pawn was registered in this token. This
        // token should be a valid file.
        if self.try_file(token, move_obj).is_ok() {
            return Ok(());
        }

        Err(
            ChuiError::InvalidMove(
                format!("`{}` is not a valid pawn or piece move", token)
            )
        )
    }

    /// * e4
    /// * e4+
    /// * e4#
    /// * e8Q
    /// * Bf4
    /// * 0-0
    /// * e4++
    /// * e8Q+
    /// * Bf4+
    /// * Bf4#
    /// * e8Q#
    /// * e8=Q
    /// * exf4
    /// * Bxf4
    /// * 0-0+
    /// * 0-0#
    /// * e8Q++
    /// * e8=Q+
    /// * e8=Q#
    /// * exf4+
    /// * exf4#
    /// * exf8Q
    /// * Bf4++
    /// * Bxf4+
    /// * Bxf4#
    /// * 0-0++
    /// * 0-0-0
    /// * e8=Q++
    /// * exf4++
    /// * exf8Q+
    /// * exf8Q#
    /// * exf8=Q
    /// * Bxf4++
    /// * 0-0-0+
    /// * 0-0-0#
    /// * exf8=Q#
    /// * exf8=Q+
    /// * exf8Q++
    /// * 0-0-0++
    /// * exf8=Q++
    /// 
    ///     Token 2: f, 4, x, - (
    ///         file, rank, capture, castle king
    ///     )
    fn process_token_2(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * exf4
        // * exf4+
        // * exf4++
        // * exf4#
        // * exf8Q
        // * exf8Q+
        // * exf8Q++
        // * exf8Q#
        // * exf8=Q
        // * exf8=Q++
        // * exf8=Q#
        // * Bxf4
        // * Bxf4+
        // * Bxf4++
        // * Bxf4#
        // This token can be a capture move in any variation.
        if self.try_capture(token, move_obj).is_ok() {
            return Ok(());
        }

        // * e4
        // * e4+
        // * e4++
        // * e4#
        // * e8Q
        // * e8Q+
        // * e8Q++
        // * e8Q#
        // * e8=Q
        // * e8=Q+
        // * e8=Q++
        // * e8=Q#
        // If move is Pawn move, then this token is to-rank.
        if let Some(MoveType::PawnMove) = move_obj.move_type {
            if self.try_rank(token, move_obj).is_ok() {
                return Ok(());
            }

            return Err(
                ChuiError::InvalidMove(
                    format!(
                        "`{}` is not a valid rank for piece `{:?}`",
                        token,
                        move_obj.piece.unwrap(),
                    )
                )
            );
        }

        // * Bf4
        // * Bf4+
        // * Bf4++
        // * Bf4#
        // If move is a Piece move, then this token is to-file.
        if let Some(MoveType::PieceMove) = move_obj.move_type {
            if self.try_file(token, move_obj).is_ok() {
                return Ok(());
            }

            return Err(
                ChuiError::InvalidMove(
                    format!(
                        "`{}` is not a valid file for piece `{:?}`",
                        token,
                        move_obj.piece.unwrap(),
                    )
                )
            )
        }

        // * 0-0
        // * 0-0+
        // * 0-0#
        // * 0-0++
        // * 0-0-0
        // * 0-0-0+
        // * 0-0-0#
        // * 0-0-0++
        // Must already be castling.
        if self.try_castle_king_continuation(token, move_obj).is_ok() {
            return Ok(());
        }

        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is not a valid for {:?}",
                    token,
                    move_obj.piece.unwrap()
                )
            )
        )
    }

    /// * e4+
    /// * e4#
    /// * e8Q
    /// * Bf4
    /// * 0-0
    /// * e4++
    /// * e8Q+
    /// * Bf4+
    /// * Bf4#
    /// * e8Q#
    /// * e8=Q
    /// * exf4
    /// * Bxf4
    /// * 0-0+
    /// * 0-0#
    /// * e8Q++
    /// * e8=Q+
    /// * e8=Q#
    /// * exf4+
    /// * exf4#
    /// * exf8Q
    /// * Bf4++
    /// * Bxf4+
    /// * Bxf4#
    /// * 0-0++
    /// * 0-0-0
    /// * e8=Q++
    /// * exf4++
    /// * exf8Q+
    /// * exf8Q#
    /// * exf8=Q
    /// * Bxf4++
    /// * 0-0-0+
    /// * 0-0-0#
    /// * exf8=Q#
    /// * exf8=Q+
    /// * exf8Q++
    /// * 0-0-0++
    /// * exf8=Q++
    /// 
    ///     Token 3: f, 4, +, #, Q, =, 0 (
    ///         file, rank, check, mate, promotiom piece,
    ///         promotion notatiomn, castle king
    ///     )
    fn process_token_3(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * exf4
        // * exf4+
        // * exf4#
        // * exf4++
        // * exf8Q+
        // * exf8Q#
        // * exf8=Q
        // * exf8Q
        // * exf8=Q#
        // * exf8=Q+
        // * exf8Q++
        // * Bxf4
        // * Bxf4+
        // * Bxf4#
        // * Bxf4++
        // * exf8=Q++
        // If move is a capture, this token is to-file.
        if let Some(MoveType::PawnCapture) |
               Some(MoveType::PieceCapture) = move_obj.move_type
        {
            if self.try_file(token, move_obj).is_ok() {
                return Ok(());
            }

            return Err(
                ChuiError::InvalidMove(
                    format!(
                        "`{}` is an invalid file for {:?} capture",
                        token,
                        move_obj.piece.unwrap(),
                    )
                )
            );
        }

        // * e4+
        // * e4++
        // * e4#
        // * e8=Q
        // * e8=Q+
        // * e8=Q#
        // * e8=Q++
        // * e8Q
        // * e8Q+
        // * e8Q#
        // * e8Q++
        // If move is a pawn move, this token can be check, check
        // mate, promotion notation, or promotion piece.
        if let Some(MoveType::PawnMove) = move_obj.move_type {
            // * e4+
            // * e4++
            // Check.
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * e4#
            // Check mate.
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }

            // * e8=Q
            // * e8=Q+
            // * e8=Q#
            // * e8=Q++
            // Promotion notation.
            if self.try_promotion_notation(token, move_obj).is_ok() {
                return Ok(());
            }

            // * e8Q
            // * e8Q+
            // * e8Q#
            // * e8Q++
            // If white piece results, then found Pawn promotion.
            if self.try_promotion_piece(token, move_obj).is_ok() {
                return Ok(());
            }

            return Err(
                ChuiError::InvalidMove(
                    format!(
                        "`{}` is an invalid for {:?}",
                        token,
                        move_obj.piece.unwrap(),
                    )
                )
            );
        }

        // * Bf4
        // * Bf4+
        // * Bf4#
        // * Bf4++
        // If move is Piece move, this token is to-rank.
        if let Some(MoveType::PieceMove) = move_obj.move_type {
            if self.try_rank(token, move_obj).is_ok() {
                return Ok(());
            }

            return Err(
                ChuiError::InvalidMove(
                    format!(
                        "`{}` is an invalid to-rank for {:?} move",
                        token,
                        move_obj.piece.unwrap(),
                    )
                )
            );
        }

        // * 0-0
        // * 0-0+
        // * 0-0#
        // * 0-0++
        // * 0-0-0
        // * 0-0-0+
        // * 0-0-0#
        // * 0-0-0++
        // Castling move, king side.
        if self.try_castle_king(token, move_obj).is_ok() {
            return Ok(());
        }

        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is invalid for {:?}", token, move_obj.piece.unwrap(),
                )
            )
        )
    }

    /// * e4++
    /// * e8Q+
    /// * Bf4+
    /// * Bf4#
    /// * e8Q#
    /// * e8=Q
    /// * exf4
    /// * Bxf4
    /// * 0-0+
    /// * 0-0#
    /// * e8Q++
    /// * e8=Q+
    /// * e8=Q#
    /// * exf4+
    /// * exf4#
    /// * exf8Q
    /// * Bf4++
    /// * Bxf4+
    /// * Bxf4#
    /// * 0-0++
    /// * 0-0-0
    /// * e8=Q++
    /// * exf4++
    /// * exf8Q+
    /// * exf8Q#
    /// * exf8=Q
    /// * Bxf4++
    /// * 0-0-0+
    /// * 0-0-0#
    /// * exf8=Q#
    /// * exf8=Q+
    /// * exf8Q++
    /// * 0-0-0++
    /// * exf8=Q++
    /// 
    ///     Token 4: 4, +, #, Q, - (
    ///         rank, check, mate, promotion piece, castle queen
    ///     )
    fn process_token_4(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * exf4
        // * exf4+
        // * exf4++
        // * exf4#
        // * exf8Q
        // * exf8Q+
        // * exf8Q++
        // * exf8Q#
        // * exf8=Q
        // * exf8=Q+
        // * exf8=Q++
        // * exf8=Q#
        // * Bxf4
        // * Bxf4+
        // * Bxf4++
        // * Bxf4#
        // If move is a capture, this token is to-rank.
        if let Some(MoveType::PawnCapture) |
               Some(MoveType::PieceCapture) = move_obj.move_type
        {
            if self.try_rank(token, move_obj).is_ok() {
                return Ok(());
            }

            return Err(
                ChuiError::InvalidMove(
                    format!(
                        "`{}` is an invalid rank for {:?} capture",
                        token,
                        move_obj.piece.unwrap(),
                    )
                )
            );
        }

        // * e4++
        // If move is a Pawn move, this token can be check mate if
        // `move_obj.check` is already set.
        if let Some(MoveType::PawnMove) = move_obj.move_type {
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        // * e8Q+
        // * e8Q#
        // * e8=Q
        // * e8Q++
        // * e8=Q+
        // * e8=Q#
        // * e8=Q++
        // If move is pawn move, this token is check, check mate,
        // or pawn promotion.
        if let Some(MoveType::PawnMove) = move_obj.move_type {
            // * e8Q+
            // * e8Q++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * e8Q#
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }

            // * e8=Q
            // * e8=Q+
            // * e8=Q#
            // * e8=Q++
            if self.try_promotion_piece(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        // * Bf4+
        // * Bf4#
        // * Bf4++
        // If move is a piece move, then this token is check or
        // check mate.
        if let Some(MoveType::PieceMove) = move_obj.move_type {
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }

            return Err(
                ChuiError::InvalidMove(
                    format!(
                        "`{}` is an invalid check or check mate move",
                        token
                    )
                )
            )
        }

        // * 0-0+
        // * 0-0++
        // * 0-0#
        // * 0-0-0
        // * 0-0-0+
        // * 0-0-0#
        // * 0-0-0++
        // If move is a castling move, this token is check, check mate
        // or queen side castle.
        if let Some(MoveType::Castle) = move_obj.move_type {
            // * 0-0+
            // * 0-0++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * 0-0#
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }

            // * 0-0-0
            // * 0-0-0+
            // * 0-0-0#
            // * 0-0-0++
            if self.try_castle_queen_continuation(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is an invalid for {:?} move",
                    token,
                    move_obj.piece.unwrap(),
                )
            )
        )
    }

    /// * e8Q++
    /// * e8=Q+
    /// * e8=Q#
    /// * exf4+
    /// * exf4#
    /// * exf8Q
    /// * Bf4++
    /// * Bxf4+
    /// * Bxf4#
    /// * 0-0++
    /// * 0-0-0
    /// * e8=Q++
    /// * exf4++
    /// * exf8Q+
    /// * exf8Q#
    /// * exf8=Q
    /// * Bxf4++
    /// * 0-0-0+
    /// * 0-0-0#
    /// * exf8=Q#
    /// * exf8=Q+
    /// * exf8Q++
    /// * 0-0-0++
    /// * exf8=Q++
    /// 
    ///     Token 5: +, #, Q, 0, = (
    ///         check, mate, promotion piece, castle queen
    ///     )
    fn process_token_5(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * exf4+
        // * exf4++
        // * exf4#
        // * exf8Q
        // * exf8Q+
        // * exf8Q++
        // * exf8Q#
        // * exf8=Q
        // * exf8=Q+
        // * exf8=Q++
        // * exf8=Q#
        // If move is pawn capture, token is either check, check mate
        // piece promotion, or promotion notation.
        if let Some(MoveType::PawnCapture) = move_obj.move_type {
            // * exf4+
            // * exf4++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * exf4#
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }

            // * exf8Q
            // * exf8Q+
            // * exf8Q++
            // * exf8Q#
            if self.try_promotion_piece(token, move_obj).is_ok() {
                return Ok(());
            }

            // * exf8=Q
            // * exf8=Q+
            // * exf8=Q++
            // * exf8=Q#
            if self.try_promotion_notation(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        // * Bxf4+
        // * Bxf4++
        // * Bxf4#
        if let Some(MoveType::PieceCapture) = move_obj.move_type {
            // * Bxf4+
            // * Bxf4++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * Bxf4#
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        // * e8Q++
        // * e8=Q+
        // * e8=Q++
        // * e8=Q#
        if let Some(MoveType::PawnMove) = move_obj.move_type {
            // * e8Q++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * e8=Q+
            // * e8=Q++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * e8=Q#
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        // * Bf4++
        if let Some(MoveType::PieceMove) = move_obj.move_type {
            // * Bf4++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        // * 0-0++
        // * 0-0-0
        // * 0-0-0+
        // * 0-0-0++
        // * 0-0-0#
        if let Some(MoveType::Castle) = move_obj.move_type {
            // * 0-0++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * 0-0-0
            // * 0-0-0+
            // * 0-0-0++
            // * 0-0-0#
            if self.try_castle_queen(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        Err(AlgebraicParser::token_not_satisfied(token))
    }

    /// * exf4++
    /// * exf8Q+
    /// * exf8Q++
    /// * exf8Q#
    /// * exf8=Q
    /// * exf8=Q+
    /// * exf8=Q++
    /// * exf8=Q#
    /// * e8=Q++
    /// * Bxf4++
    /// * 0-0-0+
    /// * 0-0-0#
    /// * 0-0-0++
    ///
    ///     Token 6: +, #, Q (
    ///         check, mate, promotion piece
    ///     )
    fn process_token_6(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * exf4++
        // * exf8Q+
        // * exf8Q++
        // * exf8Q#
        // * exf8=Q+
        // * exf8=Q++
        // * exf8=Q#
        if let Some(MoveType::PawnCapture) = move_obj.move_type {
            // * exf4++
            // * exf8Q+
            // * exf8Q++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * exf8Q#
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }

            // * exf8=Q
            // * exf8=Q+
            // * exf8=Q++
            // * exf8=Q#
            if self.try_promotion_piece(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        // * e8=Q++
        // * Bxf4++
        if let Some(MoveType::PawnMove) |
               Some(MoveType::PieceCapture) = move_obj.move_type
        {
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }
        }
        
        // * 0-0-0+
        // * 0-0-0++
        // * 0-0-0#
        if let Some(MoveType::Castle) = move_obj.move_type {
            // * 0-0-0+
            // * 0-0-0++
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            // * 0-0-0#
            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is an invalid for {:?} move",
                    token,
                    move_obj.piece.unwrap(),
                )
            )
        )
    }

    /// * exf8Q++
    /// * exf8=Q+
    /// * exf8=Q++
    /// * exf8=Q#
    /// * 0-0-0++
    ///
    ///     Token 7: +, # (
    ///         check, mate
    ///     )
    fn process_token_7(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * exf8Q++
        // * exf8=Q+
        // * exf8=Q++
        // * exf8=Q#
        // * 0-0-0++
        if let Some(MoveType::PawnCapture) |
               Some(MoveType::Castle) = move_obj.move_type
        {
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }

            if self.try_check_mate(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        
        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is an invalid for {:?} move",
                    token,
                    move_obj.piece.unwrap(),
                )
            )
        )
    }

    /// * exf8=Q++
    ///
    ///     Token 8: + (
    ///         check (mate)
    ///     )
    fn process_token_8(&self, token: char, move_obj: &mut Move)
    -> ChuiResult<()>
    {
        // * exf8=Q++
        if let Some(MoveType::PawnCapture) = move_obj.move_type {
            if self.try_check(token, move_obj).is_ok() {
                return Ok(());
            }
        }

        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is an invalid for {:?} move",
                    token,
                    move_obj.piece.unwrap(),
                )
            )
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{
        parser::{self, Parser},
        ParserEngine,
        Color,
        Player,
        Engine,
        ChuiResult,
    };

    fn parser() -> (Box<dyn Parser>, Engine<'static>) {
        let white = Player::new(
            Color::White,
            Some("Camina Drummer"),
            Some(37),
            None,
        );
    
        let black = Player::new(
            Color::Black,
            Some("Klaes Ashford"),
            Some(72),
            Some(1500),
        );
        
        let engine = Engine::new(white, black).unwrap();
    
        (parser::new(ParserEngine::Algebraic), engine)
    }

    fn parse_the_move(the_move: &str) -> ChuiResult<()> {
        let (parser, engine) = parser();
        match parser.parse(the_move, &engine) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    #[test]
    fn test_e4() -> ChuiResult<()> {
        parse_the_move("e4")
    }

    #[test]
    fn test_e4_check() -> ChuiResult<()> {
        parse_the_move("e4+")
    }
    
    #[test]
    fn test_e4_mate() -> ChuiResult<()> {
        parse_the_move("e4#")
    }
    
    #[test]
    fn test_e4_queen() -> ChuiResult<()> {
        parse_the_move("e4Q")
    }
    
    #[test]
    fn test_e4_check_check() -> ChuiResult<()> {
        parse_the_move("e4++")
    }
    
    #[test]
    fn test_e8_queen_check() -> ChuiResult<()> {
        parse_the_move("e8Q+")
    }
    
    #[test]
    fn test_e8_queen_mate() -> ChuiResult<()> {
        parse_the_move("e8Q#")
    }
    
    #[test]
    fn test_e8_equals_queen() -> ChuiResult<()> {
        parse_the_move("e8=Q")
    }
    
    #[test]
    fn test_e4_takes_f4() -> ChuiResult<()> {
        parse_the_move("exf4")
    }
    
    #[test]
    fn test_e8_queen_check_check() -> ChuiResult<()> {
        parse_the_move("e8Q++")
    }
    
    #[test]
    fn test_e8_equals_queen_check() -> ChuiResult<()> {
        parse_the_move("e8=Q+")
    }
    
    #[test]
    fn test_e8_equals_queen_mate() -> ChuiResult<()> {
        parse_the_move("e8=Q#")
    }
    
    #[test]
    fn test_e_takes_f4_check() -> ChuiResult<()> {
        parse_the_move("exf4+")
    }
    
    #[test]
    fn test_e_takes_f4_mate() -> ChuiResult<()> {
        parse_the_move("exf4#")
    }
    
    #[test]
    fn test_e_takes_f8_queen() -> ChuiResult<()> {
        parse_the_move("exf8Q")
    }
    
    #[test]
    fn test_e8_equals_queen_check_check() -> ChuiResult<()> {
        parse_the_move("e8=Q++")
    }
    
    #[test]
    fn test_e_takes_f4_check_check() -> ChuiResult<()> {
        parse_the_move("exf4++")
    }
    
    #[test]
    fn test_e_takes_f8_queen_check() -> ChuiResult<()> {
        parse_the_move("exf8Q+")
    }
    
    #[test]
    fn test_e_takes_f8_queen_mate() -> ChuiResult<()> {
        parse_the_move("exf8Q#")
    }
    
    #[test]
    fn test_e_takes_f8_equals_queen() -> ChuiResult<()> {
        parse_the_move("exf8=Q")
    }
    
    #[test]
    fn test_e_takes_f8_equals_queen_mate() -> ChuiResult<()> {
        parse_the_move("exf8=Q#")
    }
    
    #[test]
    fn test_e_takes_f8_equals_queen_check() -> ChuiResult<()> {
        parse_the_move("exf8=Q+")
    }
    
    #[test]
    fn test_e_takes_f8_queen_check_check() -> ChuiResult<()> {
        parse_the_move("exf8Q++")
    }
    
    #[test]
    fn test_e_takes_f8_equals_queen_check_check() -> ChuiResult<()> {
        parse_the_move("exf8=Q++")
    }
    
    #[test]
    fn test_bishop_f4() -> ChuiResult<()> {
        parse_the_move("Bf4")
    }
    
    #[test]
    fn test_bishop_f4_check() -> ChuiResult<()> {
        parse_the_move("Bf4+")
    }
    
    #[test]
    fn test_bishop_f4_mate() -> ChuiResult<()> {
        parse_the_move("Bf4#")
    }
    
    #[test]
    fn test_bishop_takes_f4() -> ChuiResult<()> {
        parse_the_move("Bxf4")
    }

    #[test]
    fn test_bishop_f4_check_check() -> ChuiResult<()> {
        parse_the_move("Bf4++")
    }

    #[test]
    fn test_bishop_takes_f4_check() -> ChuiResult<()> {
        parse_the_move("Bxf4+")
    }

    #[test]
    fn test_bishop_takes_f4_mate() -> ChuiResult<()> {
        parse_the_move("Bxf4#")
    }

    #[test]
    fn test_bishop_takes_f4_check_check() -> ChuiResult<()> {
        parse_the_move("Bxf4++")
    }

    #[test]
    fn test_castle_king() -> ChuiResult<()> {
        parse_the_move("0-0")
    }

    #[test]
    fn test_castle_king_check() -> ChuiResult<()> {
        parse_the_move("0-0+")
    }

    #[test]
    fn test_castle_king_mate() -> ChuiResult<()> {
        parse_the_move("0-0#")
    }

    #[test]
    fn test_castle_king_check_check() -> ChuiResult<()> {
        parse_the_move("0-0++")
    }

    #[test]
    fn test_castle_queen() -> ChuiResult<()> {
        parse_the_move("0-0-0")
    }

    #[test]
    fn test_castle_queen_check() -> ChuiResult<()> {
        parse_the_move("0-0-0+")
    }

    #[test]
    fn test_castle_queen_mate() -> ChuiResult<()> {
        parse_the_move("0-0-0#")
    }

    #[test]
    fn test_castle_queen_check_check() -> ChuiResult<()> {
        parse_the_move("0-0-0++")
    }
}
