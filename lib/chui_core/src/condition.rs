//! Conditions.

/// The win condition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinCondition {
    /// Checkmate.
    #[default]
    Checkmate,

    /// White resigns.
    WhiteResigns,

    /// Black resigns.
    BlackResigns,
}

/// The draw condition.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DrawCondition {
    /// Both players agree to a draw.
    #[default]
    AgreeToDraw,

    /// The one to move cannot make any valid moves.
    Stalemate,

    /// The board's position has repeated itself three times.
    ///
    /// According to FIDE rules, just because a position has repeated itself three times,
    /// doesn't mean the game is automatically a draw. A player must flag this condition after
    /// the position has repeated itself for the third time.
    ///
    /// Also note this has nothing to do with move order. This is a repetition of position, not
    /// moves.
    ThirdRepitition,

    /// The board's position has repeated itself five times.
    ///
    /// According to FIDE rules, a game is automatically drawn after the position has repeated
    /// itself five times. A player is not needed to flag this condition.
    FifthRepetition,

    /// 50 moves have been made with no piece capture or pawn move.
    FiftyMoveRule,

    /// Both players have insufficient material to check mate. Do note that it is still possible
    /// to checkmate an opponent King with just a bishop or a knight provided that the opponent
    /// has a blocking piece to make this possible.
    InsufficientMaterial,

    /// Both players agree to that there will be perpetual check. This draw condition is
    /// technically not needed because a perpectual check will often result in a position
    /// repetition or vai the Fifty Move Rule.
    PerpetualCheck,
}
