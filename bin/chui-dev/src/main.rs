//! Main dev.

#![allow(unused_imports)]
#![allow(clippy::shadow_unrelated)]

use std::mem::{size_of, size_of_val};

use chui_core::prelude::{coord::*, *};

/// Get a piece from the position.
fn get_piece(position: &dyn Position, coord: Coord) {
    let piece = position.get_piece(coord);
    piece.map_or_else(
        || {
            println!("Piece at {0}: None", coord);
        },
        |piece| {
            println!("Piece at {0}: {1} {1:?}", coord, piece);
        },
    );
}

/// Put a piece into the position.
fn put_piece(position: &mut dyn Position, piece: Piece) {
    let ret_piece = position.put_piece(Some(piece), piece.get_coord());
    println!(
        "Piece at {0}: {1} {1:?}",
        piece.get_coord(),
        position.get_piece(piece.get_coord()).unwrap()
    );
    ret_piece.map_or_else(
        || {
            println!("  Returned piece from {0}: None", piece.get_coord());
        },
        |piece| {
            println!(
                "  Returned piece from {0}: {1} {1:?}",
                piece.get_coord(),
                piece
            );
        },
    );
}

/// Do stuff to `position`.
fn do_position(position: &mut dyn Position) -> ChuiResult<()> {
    println!("=====================================");

    get_piece(position, Coord::try_from(A1)?);
    get_piece(position, Coord::try_from(A8)?);
    get_piece(position, Coord::try_from(H8)?);
    get_piece(position, Coord::try_from(E8)?);
    get_piece(position, Coord::try_from(D1)?);
    get_piece(position, Coord::try_from(F7)?);
    get_piece(position, Coord::try_from(E4)?);

    println!();

    put_piece(
        position,
        Piece::new(PieceKind::Pawn, Color::White, Coord::try_from(A3)?),
    );
    put_piece(
        position,
        Piece::new(PieceKind::Rook, Color::Black, Coord::try_from(G6)?),
    );
    put_piece(
        position,
        Piece::new(PieceKind::Rook, Color::Black, Coord::try_from(A3)?),
    );
    println!("{}", position);

    println!();

    get_piece(position, Coord::try_from(A3)?);
    get_piece(position, Coord::try_from(G6)?);

    println!();

    put_piece(
        position,
        Piece::new(PieceKind::Pawn, Color::White, Coord::try_from(A3)?),
    );
    put_piece(
        position,
        Piece::new(PieceKind::Rook, Color::Black, Coord::try_from(G6)?),
    );
    put_piece(
        position,
        Piece::new(PieceKind::Rook, Color::Black, Coord::try_from(A3)?),
    );
    println!("{}", position);

    Ok(())
}

fn main() -> ChuiResult<()> {
    let mut easy_position = EasyPosition::new(Variant::StandardChess);
    let mut bit_position = BitPosition::new(Variant::StandardChess);
    let mut array_bit_position = ArrayBitPosition::new(Variant::StandardChess);

    do_position(&mut easy_position).ok();
    do_position(&mut bit_position).ok();
    do_position(&mut array_bit_position).ok();

    println!(
        "Size of EasyPosition: {} bytes.",
        size_of_val(&easy_position)
    );
    println!("Size of BitPosition: {} bytes.", size_of_val(&bit_position));
    println!(
        "Size of ArrayBitPosition: {} bytes.",
        size_of_val(&array_bit_position)
    );

    let o_piece: Option<Piece> = None;
    println!(
        "Size of Option<Piece> = None: {} bytes.",
        size_of_val(&o_piece)
    );
    let o_piece: Option<Piece> = Some(Piece::new(
        PieceKind::King,
        Color::White,
        Coord::try_from(A1)?,
    ));
    println!(
        "Size of Option<Piece> = White King at a3: {} bytes.",
        size_of_val(&o_piece)
    );

    Ok(())
}
