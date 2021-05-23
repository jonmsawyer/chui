#![allow(clippy::new_ret_no_self)]

//use std::fmt;
use std::convert::TryFrom;
// use std::collections::HashMap;

use crate::{ChuiResult, ChuiError};

use super::Parser;
use super::super::{Move, Piece, MoveGenerator, Color};
//use super::super::parser::ParserEngine;

/// A parser that will parse algebraic chess notation.
/// Example moves: `e4`, `Bxc6+`, `Kd6`, `e8Q#`, `a1=N`, etc.
#[derive(Debug)]
pub struct AlgebraicParser<'a>{
    pub move_generator: MoveGenerator<'a>,
    pub move_obj: Move,
}

impl<'a> Parser for AlgebraicParser<'a> {
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
    /// * Rae1
    /// * Raxe1
    /// * Rae1+
    /// * Rae1#
    /// * Raxe1+
    /// * Rae1++
    /// * Raxe1#
    /// * Raxe1++
    /// 
    /// * R1e1
    /// * R1xe1
    /// * R1e1+
    /// * R1e1#
    /// * R1xe1+
    /// * R1e1++
    /// * R1xe1#
    /// * R1xe1++
    /// 
    /// * Ra1e1
    /// * Ra1e1+
    /// * Ra1e1#
    /// * Ra1xe1
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1e1++
    /// * Ra1xe1++
    /// 
    /// Token 1: e, B, 0 (
    ///     file, piece, castle king
    /// )
    /// 
    /// Token 2: f, 4, x, - (
    ///     file, rank, capture, castle king continuation
    /// )
    /// 
    /// Token 3: f, 4, +, #, Q, =, 0, x (
    ///     file, rank, check, mate, promotiom piece,
    ///     promotion notatiomn, castle king, capture
    /// )
    /// 
    /// Token 4: 4, +, #, Q, - (
    ///     rank, check, mate, promotion piece,
    ///     castle queen continuation
    /// )
    /// 
    /// Token 5: +, #, Q, =, 0 (
    ///     check, mate, promotion piece, promotion notation,
    ///     castle queen
    /// )
    /// 
    /// Token 6: +, #, Q (
    ///     check, mate, promotion piece
    /// )
    /// 
    /// Token 7: +, # (
    ///     check, mate
    /// )
    /// 
    /// Token 8: + (
    ///     check (mate)
    /// )
    fn parse(&mut self, the_move: &str) -> ChuiResult<Move> {
        // Check the move to see it's valid. No whitespace allowed. At
        // the same time, trim any surrounding whitespace.
        let the_move = self.trim_and_check_whitespace(the_move)?;

        if the_move.len() < 2 {
            self.invalid_input("Input move is too small in length (<2)")?
        }

        if the_move.len() > 8 {
            self.invalid_input("Input move is too large in length (>8)")?
        }

        // Record the input move.
        self.move_obj.set_input_move(the_move);

        // Parse each character in the move.
        for (move_idx, token) in the_move.chars().enumerate() {
            match move_idx {
                0 => self.parse_token_1(token)?,
                1 => self.parse_token_2(token)?,
                2 => self.parse_token_3(token)?,
                3 => self.parse_token_4(token)?,
                4 => self.parse_token_5(token)?,
                5 => self.parse_token_6(token)?,
                6 => self.parse_token_7(token)?,
                7 => self.parse_token_8(token)?,
                _ => self.move_index_not_implemented(move_idx)?,
            }
        }

        // When we're done parsing, return an owned instance of
        // `self.move_obj` and reset `self.move_obj` to a new
        // instance of `Move`.
        let move_obj = self.move_obj.to_owned();
        self.move_obj = Move::new();

        Ok(move_obj)
    }

    /// The name of the parser.
    fn name(&self) -> String {
        "Algebraic Parser".to_string()
    }

    /// Some examples of the moves parsed by this parser.
    fn eg(&self) -> String {
        format!("e4, Bxc6, Qb4, exf8=Q++ ({})", self.name())
    }
}

impl<'a> AlgebraicParser<'a> {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(
            AlgebraicParser {
                move_generator: MoveGenerator::new(),
                move_obj: Move::new(),
            }
        )
    }

    /// In algebraic notation, there's no differentiation between
    /// white and black pieces. Try to obtain a piece from this token
    /// which should end up being a white piece. During processing,
    /// the color of the piece will be ignored. If the token begins
    /// with a castling move, the piece is a `King`. If no piece matches
    /// then the move is presumably a `Pawn` move.
    fn try_piece(&mut self, token: char) -> ChuiResult<()> {
        // Try to obtain a piece from the token.
        if let Ok(piece) = Piece::try_from(token) {
            if let Piece::King(Color::White) |
                   Piece::Queen(Color::White) |
                   Piece::Rook(Color::White) |
                   Piece::Bishop(Color::White) |
                   Piece::Knight(Color::White) |
                   Piece::Pawn(Color::White) = piece
            {
                self.move_obj.set_piece_move(piece);
                return Ok(());
            }
        }

        // Try to obtain a `King` from a castling move.
        for castle_notation in self.move_generator.castle_notation.iter() {
            if castle_notation.starts_with(token) {
                self.move_obj.set_castling_king();
                return Ok(());
            }
        }

        // The only other option is a `Pawn` move.
        self.move_obj.set_pawn_move();

        // If all else fails, this token could not be parsed
        // (i.e., invalid file such as 's').
        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a valid file. Must
    /// be one of \[abcdefgh\]. If a valid file is found, record
    /// it in the move.
    fn try_file(&mut self, token: char) -> ChuiResult<()> {
        if let Some(index) = self.match_file_to_index(token) {
            self.move_obj.set_to_coord_file(token);
            self.move_obj.set_to_index_file(index);
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a capture. Must be
    /// 'x'. If a valid capture is found, record it in the move.
    fn try_capture(&mut self, token: char) -> ChuiResult<()> {
        if self.move_generator.capture.starts_with(token) {
            self.move_obj.set_capture()?;
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a valid rank. Must
    /// be one of \[12345678\]. If a valid rank is found, record
    /// it in the move.
    fn try_rank(&mut self, token: char) -> ChuiResult<()> {
        if let Some(index) = self.match_rank_to_index(token) {
            self.move_obj.set_to_coord_rank(index + 1);
            self.move_obj.set_to_index_rank(index);
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a valid check. Must
    /// be '+'. If valid check is found, record it in the move.
    /// If the move's check flag is already set, record this
    /// move as a check mate.
    fn try_check(&mut self, token: char) -> ChuiResult<()> {
        if self.move_generator.check.starts_with(token) {
            if self.move_obj.is_check() {
                self.move_obj.set_check_mate();
            }
            else {
                self.move_obj.set_check();
            }
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a valid check mate.
    /// If a valid check mate is found, record it in the move.
    fn try_check_mate(&mut self, token: char) -> ChuiResult<()> {
        for check_mate in self.move_generator.check_mate.iter() {
            if check_mate.starts_with(token) {
                self.move_obj.set_check_mate();
                return Ok(());
            }
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a valid promotion
    /// notation. If a valid promotion notation is move, record
    /// it in the move. Note, we don't yet known what the
    /// promotion piece is. Next token should be the proper
    /// promotion piece.
    fn try_promotion_notation(&mut self, token: char) -> ChuiResult<()> {
        for notation in self.move_generator.promotion_notation.iter() {
            if notation.starts_with(token) {
                self.move_obj.set_promotion();
                return Ok(());
            }
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a promotion piece. The
    /// move's promotion flag need not be set. If a valid promotion
    /// is found, flag the move as a promotion and record the piece.
    fn try_promotion_piece(&mut self, token: char) -> ChuiResult<()> {
        if let Ok(piece) = Piece::try_from(token) {
            if let Piece::King(Color::White) |
                   Piece::Queen(Color::White) |
                   Piece::Rook(Color::White) |
                   Piece::Bishop(Color::White) |
                   Piece::Knight(Color::White) = piece
            {
                self.move_obj.set_promotion_piece(piece);
                return Ok(());
            }
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a castle move. Note: this
    /// is the first attempt at parsing a castling move, so
    /// tentatively  set this move to be castling king side.
    fn try_castle_king(&mut self, token: char) -> ChuiResult<()> {
        for castle_notation in self.move_generator.castle_notation.iter() {
            if castle_notation.starts_with(token) &&
               self.move_obj.is_castling()
            {
                self.move_obj.set_castling_king();
                return Ok(());
            }
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a castle king side
    /// continuation. Must have already found castle king side
    /// notation. If token is a valid castling continuation notation,
    /// do nothing (as there's nothing to do).
    fn try_castle_king_continuation(&self, token: char) -> ChuiResult<()> {
        if self.move_generator.move_notation.starts_with(token) &&
           self.move_obj.is_castling_king()
        {
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a castle queen side
    /// continuation. Must have already found castle king side
    /// notation. If token is a valid castling continuation notation,
    /// flag the move as castling queen side. Next token must be
    /// the final castling notation.
    fn try_castle_queen_continuation(&mut self, token: char) -> ChuiResult<()> {
        if self.move_generator.move_notation.starts_with(token) &&
           self.move_obj.is_castling_king()
        {
            self.move_obj.set_castling_queen();
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    /// Try to parse the current token as a castle queen side.
    /// Must have already found castle queen side continuation
    /// notation. If a valid castle move is found, do nothing.
    fn try_castle_queen(&self, token: char) -> ChuiResult<()> {
        for castle_notation in self.move_generator.castle_notation.iter() {
            if castle_notation.starts_with(token) &&
               self.move_obj.is_castling_queen()
            {
                return Ok(());
            }
        }

        self.token_not_satisfied(token)
    }

    /// Parse the first token in the input move.
    /// 
    /// Token 1: e, B, 0 (
    ///     file, piece, castle king
    /// )
    /// 
    /// Valid move types for this token:
    /// 
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
    /// * Rae1
    /// * Raxe1
    /// * Rae1+
    /// * Rae1#
    /// * Raxe1+
    /// * Rae1++
    /// * Raxe1#
    /// * Raxe1++
    /// 
    /// * R1e1
    /// * R1xe1
    /// * R1e1+
    /// * R1e1#
    /// * R1xe1+
    /// * R1e1++
    /// * R1xe1#
    /// * R1xe1++
    /// 
    /// * Ra1e1
    /// * Ra1e1+
    /// * Ra1e1#
    /// * Ra1xe1
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1e1++
    /// * Ra1xe1++
    fn parse_token_1(&mut self, token: char) -> ChuiResult<()> {
        // Try to parse the first token as a `Piece`. All pieces
        // will parse as a `White` piece. The first valid piece
        // from the move string gets registered as the piece to
        // move;
        //
        // Else, a pawn was registered in this token. This
        // token should be a valid file.
        if self.try_piece(token).is_ok() ||
           self.try_file(token).is_ok()
        {
            return Ok(());
        }

        self.invalid_pawn_or_piece_move(token)
    }

    /// Parse the second token in the input move.
    /// 
    /// Token 2: f, 4, x, - (
    ///     file, rank, capture, castle king
    /// )
    /// 
    /// Valid move types for this token:
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
    /// * Rae1
    /// * Raxe1
    /// * Rae1+
    /// * Rae1#
    /// * Raxe1+
    /// * Rae1++
    /// * Raxe1#
    /// * Raxe1++
    /// 
    /// * R1e1
    /// * R1xe1
    /// * R1e1+
    /// * R1e1#
    /// * R1xe1+
    /// * R1e1++
    /// * R1xe1#
    /// * R1xe1++
    /// 
    /// * Ra1e1
    /// * Ra1e1+
    /// * Ra1e1#
    /// * Ra1xe1
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1e1++
    /// * Ra1xe1++
    fn parse_token_2(&mut self, token: char) -> ChuiResult<()> {
        // This token can be a capture move in any variation, or is
        // already castling.
        if self.try_capture(token).is_ok() ||
           self.try_castle_king_continuation(token).is_ok()
        {
            return Ok(());
        }

        // If move is pawn move, then this token is to-rank.
        if self.move_obj.is_pawn_move() &&
           self.try_rank(token).is_ok()
        {
            return Ok(());
        }

        // If move is a piece move, then this token is to-file or
        // to-rank.
        if self.move_obj.is_piece_move() &&
           self.try_file(token).is_ok() ||
           self.move_obj.is_piece_move() &&
           self.try_rank(token).is_ok()
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
    }

    /// Parse the third token in the input move.
    /// 
    /// Token 3: f, 4, +, #, Q, =, 0 (
    ///     file, rank, check, mate, promotiom piece,
    ///     promotion notatiomn, castle king
    /// )
    /// 
    /// Valid move types for this token:
    /// 
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
    /// * Rae1
    /// * Raxe1
    /// * Rae1+
    /// * Rae1#
    /// * Raxe1+
    /// * Rae1++
    /// * Raxe1#
    /// * Raxe1++
    /// 
    /// * R1e1
    /// * R1xe1
    /// * R1e1+
    /// * R1e1#
    /// * R1xe1+
    /// * R1e1++
    /// * R1xe1#
    /// * R1xe1++
    /// 
    /// * Ra1e1
    /// * Ra1e1+
    /// * Ra1e1#
    /// * Ra1xe1
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1e1++
    /// * Ra1xe1++
    fn parse_token_3(&mut self, token: char) -> ChuiResult<()> {
        // If move is a capture, this token is to-file.
        if (
               self.move_obj.is_pawn_capture() ||
               self.move_obj.is_piece_capture()
           ) &&
           self.try_file(token).is_ok()
        {
            return Ok(());
        }

        // If move is a pawn move, this token can be check, check
        // mate, promotion notation, or promotion piece.
        if self.move_obj.is_pawn_move() &&
           (
               self.try_check(token).is_ok() ||
               self.try_check_mate(token).is_ok() ||
               self.try_promotion_notation(token).is_ok() ||
               self.try_promotion_piece(token).is_ok()
           )
        {
            return Ok(());
        }

        // If move is Piece move, then this token is to-file
        // or to-rank, or capture.
        if self.move_obj.is_piece_move() &&
           self.try_file(token).is_ok() ||
           self.try_rank(token).is_ok() ||
           self.try_capture(token).is_ok()
        {
            return Ok(());
        }

        // Castling move, king side.
        if self.try_castle_king(token).is_ok() { return Ok(()); }

        self.invalid_for_piece(token)
    }

    /// Parse the fourth token in the input move.
    /// 
    /// Token 4: 4, +, #, Q, - (
    ///     rank, check, mate, promotion piece, castle queen
    /// )
    /// 
    /// Valid move types for this token:
    /// 
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
    /// * Rae1
    /// * Raxe1
    /// * Rae1+
    /// * Rae1#
    /// * Raxe1+
    /// * Rae1++
    /// * Raxe1#
    /// * Raxe1++
    /// 
    /// * R1e1
    /// * R1xe1
    /// * R1e1+
    /// * R1e1#
    /// * R1xe1+
    /// * R1e1++
    /// * R1xe1#
    /// * R1xe1++
    /// 
    /// * Ra1e1
    /// * Ra1e1+
    /// * Ra1e1#
    /// * Ra1xe1
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1e1++
    /// * Ra1xe1++
    fn parse_token_4(&mut self, token: char) -> ChuiResult<()> {
        // If move is a capture, this token is to-rank.
        if (
                self.move_obj.is_pawn_capture() ||
                self.move_obj.is_piece_capture()
           ) && self.try_rank(token).is_ok()
        {
            return Ok(());
        }

        // If move is pawn move, this token is check, check mate,
        // or pawn promotion.
        if self.move_obj.is_pawn_move() &&
           (
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok() ||
                self.try_promotion_piece(token).is_ok()
           )
        {
            return Ok(());
        }

        // If move is a piece move, then this token is to-file,
        // to-rank, capture, check or check mate.
        if self.move_obj.is_piece_move() &&
           (
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok()
           ) ||
           self.try_file(token).is_ok() ||
           self.try_rank(token).is_ok() ||
           self.try_capture(token).is_ok()
        {
            return Ok(());
        }

        // If move is a castling move, this token is check, check
        // mate, or queen side castle continuation.
        if self.move_obj.is_castle() &&
           (
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok() ||
                self.try_castle_queen_continuation(token).is_ok()
           )
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
    }

    /// Parse the fifth token in the input move.
    /// 
    /// Token 5: +, #, Q, 0, = (
    ///     check, mate, promotion piece, castle queen
    /// )
    /// 
    /// Valid move types for this token:
    /// 
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
    /// * Rae1+
    /// * Rae1#
    /// * Rae1++
    /// 
    /// * Raxe1
    /// * Raxe1+
    /// * Raxe1#
    /// * Raxe1++
    /// 
    /// * R1e1+
    /// * R1e1#
    /// * R1e1++
    /// 
    /// * R1xe1
    /// * R1xe1+
    /// * R1xe1#
    /// * R1xe1++
    /// 
    /// * Ra1e1
    /// * Ra1e1+
    /// * Ra1e1#
    /// * Ra1e1++
    /// 
    /// * Ra1xe1
    /// * Ra1xe1+
    /// 
    /// * Ra1xe1#
    /// * Ra1xe1++
    fn parse_token_5(&mut self, token: char) -> ChuiResult<()> {
        // If move is pawn capture, token is either check, check
        // mate, piece promotion, or promotion notation.
        if self.move_obj.is_pawn_capture() &&
           (
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok() ||
                self.try_promotion_piece(token).is_ok() ||
                self.try_promotion_notation(token).is_ok()
           )
        {
            return Ok(());
        }

        // If move is a pawn move or piece capture, then this
        // token is either check or check mate.
        if (
                self.move_obj.is_piece_capture() ||
                self.move_obj.is_pawn_move()
           ) &&
           (
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok()
           )
        {
            return Ok(());
        }

        // If move is a piece move or piece capture, then this
        // token is check, to-rank, to_file, or check mate.
        if (
                self.move_obj.is_piece_move() ||
                self.move_obj.is_piece_capture()
           ) &&
           (
                self.try_file(token).is_ok() ||
                self.try_rank(token).is_ok() ||
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok()
           )
        {
            return Ok(());
        }

        // If move is a castling move, this token is either
        // to-rank, to-file, check, check mate or castling queen.
        if self.move_obj.is_castle() &&
           (
                self.try_check(token).is_ok() ||
                self.try_castle_queen(token).is_ok() ||
                self.try_check_mate(token).is_ok() ||
                self.try_file(token).is_ok() ||
                self.try_rank(token).is_ok()
            )
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
    }

    /// Parse the sixth token in the input move.
    ///
    /// Token 6: +, #, Q (
    ///     check, mate, promotion piece
    /// )
    /// 
    /// Valid move types for this token:
    /// 
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
    /// * Raxe1+
    /// * Rae1++
    /// * Raxe1#
    /// * Raxe1++
    /// 
    /// * R1xe1+
    /// * R1e1++
    /// * R1xe1#
    /// * R1xe1++
    /// 
    /// * Ra1e1+
    /// * Ra1e1#
    /// * Ra1xe1
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1e1++
    /// * Ra1xe1++
    fn parse_token_6(&mut self, token: char) -> ChuiResult<()> {
        // If move is a pawn capture, then this token is either
        // check, check mate, or promotion piece.
        if self.move_obj.is_pawn_capture() &&
           (
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok() ||
                self.try_promotion_piece(token).is_ok()
           )
        {
            return Ok(());
        }

        // If move is pawn move or piece capture, then this token
        // is check.
        if (
                self.move_obj.is_pawn_move() ||
                self.move_obj.is_piece_capture()
           ) &&
           self.try_check(token).is_ok()
        {
            return Ok(());
        }

        // If move is a piece move or piece capture, then this
        // token is to-rank, check, or check mate.
        if (
                self.move_obj.is_piece_move() ||
                self.move_obj.is_piece_capture()
           ) &&
           (
                self.try_rank(token).is_ok() ||
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok()
           )
        {
            return Ok(());
        }

        // If move is a castling move, then this token is either
        // check or check mate.
        if self.move_obj.is_castle() &&
           (
                self.try_check(token).is_ok() ||
                self.try_check_mate(token).is_ok()
           )
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
    }

    /// Parse the seventh token in the input move.
    ///
    /// Token 7: +, # (
    ///     check, mate
    /// )
    /// 
    /// Valid move types for this token:
    /// 
    /// * exf8Q++
    /// * exf8=Q+
    /// * exf8=Q++
    /// * exf8=Q#
    /// * 0-0-0++
    /// 
    /// * Raxe1++
    /// * R1xe1++
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1e1++
    /// * Ra1xe1++
    /// * Raxe1++
    /// * R1xe1++
    /// * Ra1e1++
    /// * Ra1xe1+
    /// * Ra1xe1#
    /// * Ra1xe1++
    fn parse_token_7(&mut self, token: char) -> ChuiResult<()> {
        // If move is a pawn capture, a castling move,
        // piece move, or piece capture, this token is
        // either check or check mate.
        if (
                self.move_obj.is_pawn_capture() ||
                self.move_obj.is_castle() ||
                self.move_obj.is_piece_move() ||
                self.move_obj.is_piece_capture()
           ) &&
           (
               self.try_check(token).is_ok() ||
               self.try_check_mate(token).is_ok()
           )
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
    }

    /// Parse the eigth token in the input move.
    ///
    /// Token 8: + (
    ///     check (mate)
    /// )
    /// 
    /// Valid move type for this token:
    /// 
    /// * exf8=Q++
    /// * Ra1xe1++
    fn parse_token_8(&mut self, token: char) -> ChuiResult<()> {
        // If move is pawn capture or piece capture, then this
        // token is check mate.
        if (
                self.move_obj.is_pawn_capture() ||
                self.move_obj.is_piece_capture()
           ) &&
           self.try_check(token).is_ok()
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
    }

    /// Return `ChuiError::InvalidMove` indicating that the input
    /// move is invalid for the given piece.
    fn invalid_for_piece(&self, token: char) -> ChuiResult<()> {
        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is not valid for {:?}",
                    token,
                    self.move_obj.get_piece().unwrap()
                )
            )
        )
    }

    /// Return `ChuiError::InvalidMove` indicating that the move
    /// is an invalid pawn or piece move.
    fn invalid_pawn_or_piece_move(&self, token: char) -> ChuiResult<()> {
        Err(
            ChuiError::InvalidMove(
                format!("`{}` is not a valid pawn or piece move", token)
            )
        )
    }

    /// Return `ChuiError::TokenNotSatisfied` indicating that
    /// the input token has not been reasonably satisfied in any
    /// given context. This usually indicates that further processing
    /// of the token is necessary.
    fn token_not_satisfied(&self, token: char) -> ChuiResult<()> {
        Err(
            ChuiError::TokenNotSatisfied(
                format!("`{}` token is not satisfied", token)
            )
        )
    }

    /// Return `ChuiError:NotImplemented` indicating that the move
    /// index of the current token has not been implemented. This
    /// means there are more tokens to process than are accounted for.
    fn move_index_not_implemented(&self, move_idx: usize) -> ChuiResult<()> {
        Err(
            ChuiError::NotImplemented(
                format!("move index `{}` not implemented", move_idx)
            )
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{
        parser, Move, ParserEngine, Color, Piece, MoveType,
        ChuiResult, ChuiError,
    };

    fn parse_the_move(the_move: &str) -> ChuiResult<Move> {
        let mut parser = parser::new(ParserEngine::Algebraic);
        parser.parse(&the_move)
    }

    #[test]
    fn test_invalid_move() -> ChuiResult<()> {
        let the_move = "asdf";
        if parse_the_move(the_move).is_ok() {
            return Err(
                ChuiError::InvalidInput(
                    format!(
                        "The move `{}` parsed correctly \
                        when it's not supposed to",
                        the_move
                    )
                )
            );
        }

        Ok(())
    }

    #[test]
    fn test_e4() -> ChuiResult<()> {
        let the_move = "e4";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 4),
                    from_index: (8, 8),
                    to_index: (4, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e4_check() -> ChuiResult<()> {
        let the_move = "e4+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 4),
                    from_index: (8, 8),
                    to_index: (4, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e4_mate() -> ChuiResult<()> {
        let the_move = "e4#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 4),
                    from_index: (8, 8),
                    to_index: (4, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_queen() -> ChuiResult<()> {
        let the_move = "e8Q";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e4_check_check() -> ChuiResult<()> {
        let the_move = "e4++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 4),
                    from_index: (8, 8),
                    to_index: (4, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_queen_check() -> ChuiResult<()> {
        let the_move = "e8Q+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_queen_mate() -> ChuiResult<()> {
        let the_move = "e8Q#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_equals_queen() -> ChuiResult<()> {
        let the_move = "e8=Q";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e4_takes_f4() -> ChuiResult<()> {
        let the_move = "exf4";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 4),
                    from_index: (4, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_queen_check_check() -> ChuiResult<()> {
        let the_move = "e8Q++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_equals_queen_check() -> ChuiResult<()> {
        let the_move = "e8=Q+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_equals_queen_mate() -> ChuiResult<()> {
        let the_move = "e8=Q#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f4_check() -> ChuiResult<()> {
        let the_move = "exf4+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 4),
                    from_index: (4, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f4_mate() -> ChuiResult<()> {
        let the_move = "exf4#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 4),
                    from_index: (4, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_queen() -> ChuiResult<()> {
        let the_move = "exf8Q";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e8_equals_queen_check_check() -> ChuiResult<()> {
        let the_move = "e8=Q++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('e', 8),
                    from_index: (8, 8),
                    to_index: (4, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f4_check_check() -> ChuiResult<()> {
        let the_move = "exf4++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 4),
                    from_index: (4, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_queen_check() -> ChuiResult<()> {
        let the_move = "exf8Q+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_queen_mate() -> ChuiResult<()> {
        let the_move = "exf8Q#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_equals_queen() -> ChuiResult<()> {
        let the_move = "exf8=Q";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_equals_queen_mate() -> ChuiResult<()> {
        let the_move = "exf8=Q#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_equals_queen_check() -> ChuiResult<()> {
        let the_move = "exf8=Q+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_queen_check_check() -> ChuiResult<()> {
        let the_move = "exf8Q++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_e_takes_f8_equals_queen_check_check() -> ChuiResult<()> {
        let the_move = "exf8=Q++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('e', 9),
                    to_coord: ('f', 8),
                    from_index: (4, 8),
                    to_index: (5, 7),
                    piece: Some(Piece::Pawn(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: true,
                    promotion_piece: Some(Piece::Queen(Color::White)),
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PawnCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_f4() -> ChuiResult<()> {
        let the_move = "Bf4";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_f4_check() -> ChuiResult<()> {
        let the_move = "Bf4+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_f4_mate() -> ChuiResult<()> {
        let the_move = "Bf4#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_takes_f4() -> ChuiResult<()> {
        let the_move = "Bxf4";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_f4_check_check() -> ChuiResult<()> {
        let the_move = "Bf4++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_takes_f4_check() -> ChuiResult<()> {
        let the_move = "Bxf4+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_takes_f4_mate() -> ChuiResult<()> {
        let the_move = "Bxf4#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_bishop_takes_f4_check_check() -> ChuiResult<()> {
        let the_move = "Bxf4++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('f', 4),
                    from_index: (8, 8),
                    to_index: (5, 3),
                    piece: Some(Piece::Bishop(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_king() -> ChuiResult<()> {
        let the_move = "0-0";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: true,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_king_check() -> ChuiResult<()> {
        let the_move = "0-0+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: true,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_king_mate() -> ChuiResult<()> {
        let the_move = "0-0#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: true,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_king_check_check() -> ChuiResult<()> {
        let the_move = "0-0#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: true,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_queen() -> ChuiResult<()> {
        let the_move = "0-0-0";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: false,
                    is_castling_queen: true,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_queen_check() -> ChuiResult<()> {
        let the_move = "0-0-0+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: false,
                    is_castling_queen: true,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_queen_mate() -> ChuiResult<()> {
        let the_move = "0-0-0#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: false,
                    is_castling_queen: true,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_castle_queen_check_check() -> ChuiResult<()> {
        let the_move = "0-0-0++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 9),
                    to_coord: ('-', 9),
                    from_index: (8, 8),
                    to_index: (8, 8),
                    piece: Some(Piece::King(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: true,
                    is_castling_king: false,
                    is_castling_queen: true,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::Castle),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a_e1() -> ChuiResult<()> {
        let the_move = "Rae1";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a_takes_e1() -> ChuiResult<()> {
        let the_move = "Raxe1";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a_e1_check() -> ChuiResult<()> {
        let the_move = "Rae1";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_root_a_e1_mate() -> ChuiResult<()> {
        let the_move = "Rae1#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a_takes_e1_check() -> ChuiResult<()> {
        let the_move = "Raxe1+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a_e1_check_check() -> ChuiResult<()> {
        let the_move = "Rae1++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a_takes_e1_mate() -> ChuiResult<()> {
        let the_move = "Raxe1#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a_takes_e1_check_check() -> ChuiResult<()> {
        let the_move = "Raxe1++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 9),
                    to_coord: ('e', 1),
                    from_index: (0, 8),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_e1() -> ChuiResult<()> {
        let the_move = "R1e1";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_takes_e1() -> ChuiResult<()> {
        let the_move = "R1xe1";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_e1_check() -> ChuiResult<()> {
        let the_move = "R1e1+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_e1_mate() -> ChuiResult<()> {
        let the_move = "R1e1#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_takes_e1_check() -> ChuiResult<()> {
        let the_move = "R1xe1+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_e1_check_check() -> ChuiResult<()> {
        let the_move = "R1e1++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_takes_e1_mate() -> ChuiResult<()> {
        let the_move = "R1xe1#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_1_takes_e1_check_check() -> ChuiResult<()> {
        let the_move = "R1xe1++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('-', 1),
                    to_coord: ('e', 1),
                    from_index: (8, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_e1() -> ChuiResult<()> {
        let the_move = "Ra1e1";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_e1_check() -> ChuiResult<()> {
        let the_move = "Ra1e1+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_e1_mate() -> ChuiResult<()> {
        let the_move = "Ra1e1#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_takes_e1() -> ChuiResult<()> {
        let the_move = "Ra1xe1";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_takes_e1_check() -> ChuiResult<()> {
        let the_move = "Ra1xe1+";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: true,
                    check_mate: false,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_takes_e1_mate() -> ChuiResult<()> {
        let the_move = "Ra1xe1#";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_e1_check_check() -> ChuiResult<()> {
        let the_move = "Ra1e1++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceMove),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }

    #[test]
    fn test_rook_a1_takes_e1_check_check() -> ChuiResult<()> {
        let the_move = "Ra1xe1++";
        if let Ok(the_parsed_move) = parse_the_move(the_move) {
            assert_eq!(
                the_parsed_move,
                Move {
                    from_coord: ('a', 1),
                    to_coord: ('e', 1),
                    from_index: (0, 0),
                    to_index: (4, 0),
                    piece: Some(Piece::Rook(Color::White)),
                    check: false,
                    check_mate: true,
                    promotion: false,
                    promotion_piece: None,
                    is_castling: false,
                    is_castling_king: false,
                    is_castling_queen: false,
                    move_text: String::new(),
                    input_move: the_move.to_string(),
                    move_type: Some(MoveType::PieceCapture),
                }
            );

            return Ok(());
        }

        Err(ChuiError::InvalidMove("Invalid move.".to_string()))
    }
}
