//! Position module.

mod easy_2d_position;
pub use easy_2d_position::{Array2D, Easy2DPosition};

mod easy_1d_position;
pub use easy_1d_position::{Array1D, Easy1DPosition};

mod bit_position;
pub use bit_position::BitPosition;

mod bitset_position;
pub use bitset_position::BitSetPosition;

mod array_bit_position;
pub use array_bit_position::{ArrayBitPosition, BitmaskArray};

mod enum_position;
pub use enum_position::{EnumArray, EnumPosition, PieceEnum};
