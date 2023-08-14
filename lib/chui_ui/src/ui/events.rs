//! Chui's custom events

use bevy::prelude::*;

/// Resize board event. Triggered when the UI Scale Factor is updated, or
/// when a Window Resized event occurs.
#[derive(Event, Debug, Default, Copy, Clone, Hash)]
pub struct ResizeBoardEvent;

/// Piece moved event. Triggered when a piece has been moved on the chessboard.
#[derive(Debug, Default, Copy, Clone, Hash)]
pub struct PieceMovedEvent;
