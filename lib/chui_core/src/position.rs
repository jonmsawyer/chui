//! Position module.

mod easy_position;
pub use easy_position::{Array2D, EasyPosition};

mod bit_position;
pub use bit_position::BitPosition;

mod array_bit_position;
pub use array_bit_position::{ArrayBitPosition, BitmaskArray};
