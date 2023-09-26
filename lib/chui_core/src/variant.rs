//! Chess variants.

/// The various chess variants available in Chui.
#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Variant {
    /// Standard Chess is the default chess variant. Used in all tournaments
    /// and official gameplay.
    #[default]
    StandardChess,

    /// Empty chessboard.
    Empty,
    //Chess960,
}
