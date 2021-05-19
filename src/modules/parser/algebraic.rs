#![allow(clippy::new_ret_no_self)]

//use std::fmt;
use std::convert::TryFrom;
// use std::collections::HashMap;

use crate::{ChuiResult, ChuiError};

use super::Parser;
use super::super::{Move, Piece, MoveGenerator, Color, Engine};

/// A parser that will parse algebraic chess notation.
/// Example moves: `e4`, `Bxc6+`, `Kd6`, `e8Q#`, `a1=N`, etc.
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
    ///     Token 1: e, B, 0 (
    ///         file, piece, castle king
    ///     )
    /// 
    ///     Token 2: f, 4, x, - (
    ///         file, rank, capture, castle king continuation
    ///     )
    /// 
    ///     Token 3: f, 4, +, #, Q, =, 0 (
    ///         file, rank, check, mate, promotiom piece,
    ///         promotion notatiomn, castle king
    ///     )
    /// 
    ///     Token 4: 4, +, #, Q, - (
    ///         rank, check, mate, promotion piece,
    ///         castle queen continuation
    ///     )
    /// 
    ///     Token 5: +, #, Q, =, 0 (
    ///         check, mate, promotion piece, promotion notation,
    ///         castle queen
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
    fn parse(&mut self, the_move: &str, _engine: &Engine) -> ChuiResult<Move> {
        // Check the move to see it's valid. No whitespace allowed. At
        // the same time, trim any surrounding whitespace.
        let the_move = self.trim_and_check_whitespace(the_move)?;

        if the_move.len() < 2 || the_move.len() > 8 {
            self.invalid_input()?
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

    fn try_piece(&mut self, token: char) -> ChuiResult<()> {
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

        for castle_notation in self.move_generator.castle_notation.iter() {
            if castle_notation.starts_with(token) {
                self.move_obj.set_castling_king();
                return Ok(());
            }
        }

        self.move_obj.set_pawn_move();

        self.token_not_satisfied(token)
    }

    fn try_file(&mut self, token: char) -> ChuiResult<()> {
        if let Some(index) = self.match_file_to_index(token) {
            self.move_obj.set_to_coord_file(token);
            self.move_obj.set_to_index_file(index);
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    fn try_capture(&mut self, token: char) -> ChuiResult<()> {
        if self.move_generator.capture.starts_with(token) {
            self.move_obj.set_capture()?;
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    fn try_rank(&mut self, token: char) -> ChuiResult<()> {
        if let Some(index) = self.match_rank_to_index(token) {
            self.move_obj.set_to_coord_rank(index + 1);
            self.move_obj.set_to_index_rank(index);
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

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

    fn try_check_mate(&mut self, token: char) -> ChuiResult<()> {
        for check_mate in self.move_generator.check_mate.iter() {
            if check_mate.starts_with(token) {
                self.move_obj.set_check_mate();
                return Ok(());
            }
        }

        self.token_not_satisfied(token)
    }

    fn try_promotion_notation(&mut self, token: char) -> ChuiResult<()> {
        for notation in self.move_generator.promotion_notation.iter() {
            if notation.starts_with(token) {
                self.move_obj.set_promotion();
                return Ok(());
            }
        }

        self.token_not_satisfied(token)
    }

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

    fn try_castle_king_continuation(&self, token: char) -> ChuiResult<()> {
        if self.move_generator.move_notation.starts_with(token) &&
           self.move_obj.is_castling_king()
        {
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

    fn try_castle_queen_continuation(&mut self, token: char) -> ChuiResult<()> {
        if self.move_generator.move_notation.starts_with(token) &&
           self.move_obj.is_castling_king()
        {
            self.move_obj.set_castling_queen();
            return Ok(());
        }

        self.token_not_satisfied(token)
    }

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

        // If move is a piece move, then this token is to-file.
        if self.move_obj.is_piece_move() &&
           self.try_file(token).is_ok()
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
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

        // If move is Piece move, then this token is to-rank.
        if self.move_obj.is_piece_move() &&
           self.try_rank(token).is_ok()
        {
            return Ok(());
        }

        // Castling move, king side.
        if self.try_castle_king(token).is_ok() { return Ok(()); }

        self.invalid_for_piece(token)
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

        // If move is a piece move, then this token is check or
        // check mate.
        if self.move_obj.is_piece_move() &&
           (
               self.try_check(token).is_ok() ||
               self.try_check_mate(token).is_ok()
           )
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

        // If move is a piece move, then this token is check.
        if self.move_obj.is_piece_move() &&
           self.try_check(token).is_ok()
        {
            return Ok(());
        }

        // If move is a castling move, this token is either
        // check or castling queen.
        if self.move_obj.is_castle() &&
           (
               self.try_check(token).is_ok() ||
               self.try_castle_queen(token).is_ok()
           )
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
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

    /// * exf8Q++
    /// * exf8=Q+
    /// * exf8=Q++
    /// * exf8=Q#
    /// * 0-0-0++
    ///
    ///     Token 7: +, # (
    ///         check, mate
    ///     )
    fn parse_token_7(&mut self, token: char) -> ChuiResult<()> {
        // If move is a pawn capture or a castling move, this
        // token is either check or check mate.
        if (
               self.move_obj.is_pawn_capture() ||
               self.move_obj.is_castle()
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

    /// * exf8=Q++
    ///
    ///     Token 8: + (
    ///         check (mate)
    ///     )
    fn parse_token_8(&mut self, token: char) -> ChuiResult<()> {
        // If move is pawn capture, then this token is check mate.
        if self.move_obj.is_pawn_capture() &&
           self.try_check(token).is_ok()
        {
            return Ok(());
        }

        self.invalid_for_piece(token)
    }

    fn invalid_for_piece(&self, token: char) -> ChuiResult<()> {
        Err(
            ChuiError::InvalidMove(
                format!(
                    "`{}` is not a valid for {:?}",
                    token,
                    self.move_obj.get_piece().unwrap()
                )
            )
        )
    }

    fn invalid_pawn_or_piece_move(&self, token: char) -> ChuiResult<()> {
        Err(
            ChuiError::InvalidMove(
                format!("`{}` is not a valid pawn or piece move", token)
            )
        )
    }

    fn token_not_satisfied(&self, token: char) -> ChuiResult<()> {
        Err(
            ChuiError::TokenNotSatisfied(
                format!("`{}` token is not satisfied", token)
            )
        )
    }

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
        let (mut parser, engine) = parser();
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
