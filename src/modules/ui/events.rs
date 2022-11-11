//! Chui's custom events

/// Resize board event. Triggered when the UI Scale Factor is updated, or
/// when a Window Resized event occurs.
#[derive(Default)]
pub struct ResizeBoardEvent;

/// Piece moved event. Triggered when a piece has been moved on the chessboard.
#[derive(Default)]
pub struct PieceMovedEvent;
