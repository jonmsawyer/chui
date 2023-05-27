//! Chui Core's Coordinate base type.

// use std::str::FromStr;

use num_traits::PrimInt;

pub use nonmax::NonMaxU8;

use crate::STR_FILES;

mod error;
pub use error::CoordError;

/// The main result type that is returned in this application, rather than the
/// generic Ok().
pub type CoordResult<T> = std::result::Result<T, CoordError>;

/// Main `Coord` struct used to represent chess piece and board position. We use non-max
/// u8 values because indicies are 0-indexed and values of 8 are invalid for an iterable
/// of size 7.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    file: NonMaxU8,
    rank: NonMaxU8,
}

impl Coord {
    /// Create a new [`Coord`] from a file and a rank parameter.
    pub fn new(file: u8, rank: u8) -> CoordResult<Coord> {
        if let Ok(file) = NonMaxU8::try_from(file) {
            if let Ok(rank) = NonMaxU8::try_from(rank) {
                Ok(Coord { file, rank })
            } else {
                Err(CoordError::InvalidRank(format!(
                    "{} is an invalid rank",
                    rank
                )))
            }
        } else {
            Err(CoordError::InvalidFile(format!(
                "{} is an invalid file",
                file
            )))
        }
    }

    /// Return a 2-tuple representing the `Coord` as u8 values.
    pub fn to_u8_coord(&self) -> (u8, u8) {
        (self.file.get() as u8, self.rank.get() as u8)
    }

    /// Return a 2-tuple representing the `Coord` as String values.
    pub fn to_string_coord(&self) -> (String, String) {
        (self.file.get().to_string(), self.rank.get().to_string())
    }

    /// Return a 2-tuple representing the `Coord` as char values.
    pub fn to_char_coord(&self) -> (char, char) {
        (
            char::from_digit(self.file.get() as u32, 10).unwrap(),
            char::from_digit(self.rank.get() as u32, 10).unwrap(),
        )
    }

    /// Return a 2-tuple representing the `Coord` as alphanumeric values.
    pub fn to_alphanumeric_coord(&self) -> (char, u8) {
        (
            STR_FILES[self.file.get() as usize]
                .chars()
                .collect::<Vec<char>>()[0],
            self.rank.get() as u8 + 1,
        )
    }
}

impl<T, U> TryFrom<(T, U)> for Coord
where
    T: PrimInt,
    U: PrimInt,
{
    type Error = CoordError;

    fn try_from(coord: (T, U)) -> CoordResult<Coord> {
        if let Ok(file) = u8::try_from(coord.0) {
            if let Ok(rank) = u8::try_from(coord.1) {
                if let Ok(file) = NonMaxU8::try_from(coord.0) {
                    if let Ok(rank) = NonMaxU8::try_from(coord.1) {
                        Ok(Coord { file, rank })
                    } else {
                        Err(CoordError::InvalidRank(format!(
                            "{} is an invalid rank",
                            coord.1
                        )))
                    }
                } else {
                    Err(CoordError::InvalidFile(format!(
                        "{} is an invalid file",
                        coord.0
                    )))
                }
            } else {
                Err(CoordError::InvalidTypeConversion(format!(
                    "{} could not be converted to a valid u8 type",
                    coord.1
                )))
            }
        } else {
            Err(CoordError::InvalidTypeConversion(format!(
                "{} could not be converted to a valid u8 type",
                coord.0
            )))
        }
    }
}

// impl TryFrom<(u16, u16)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (u16, u16)) -> CoordResult<Coord> {
//         if let Ok(file) = u8::try_from(coord.0) {
//             if let Ok(rank) = u8::try_from(coord.1) {
//                 Coord::try_from((file, rank))
//             } else {
//                 Err(CoordError::InvalidTypeConversion(format!(
//                     "{} could not be converted to a valid u8 type",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidTypeConversion(format!(
//                 "{} could not be converted to a valid u8 type",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(u32, u32)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (u32, u32)) -> CoordResult<Coord> {
//         if let Ok(file) = u8::try_from(coord.0) {
//             if let Ok(rank) = u8::try_from(coord.1) {
//                 Coord::try_from((file, rank))
//             } else {
//                 Err(CoordError::InvalidTypeConversion(format!(
//                     "{} could not be converted to a valid u8 type",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidTypeConversion(format!(
//                 "{} could not be converted to a valid u8 type",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(u64, u64)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (u64, u64)) -> CoordResult<Coord> {
//         if let Ok(file) = u8::try_from(coord.0) {
//             if let Ok(rank) = u8::try_from(coord.1) {
//                 Coord::try_from((file, rank))
//             } else {
//                 Err(CoordError::InvalidTypeConversion(format!(
//                     "{} could not be converted to a valid u8 type",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidTypeConversion(format!(
//                 "{} could not be converted to a valid u8 type",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(u128, u128)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (u128, u128)) -> CoordResult<Coord> {
//         if let Ok(file) = u8::try_from(coord.0) {
//             if let Ok(rank) = u8::try_from(coord.1) {
//                 Coord::try_from((file, rank))
//             } else {
//                 Err(CoordError::InvalidTypeConversion(format!(
//                     "{} could not be converted to a valid u8 type",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidTypeConversion(format!(
//                 "{} could not be converted to a valid u8 type",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(usize, usize)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (usize, usize)) -> CoordResult<Coord> {
//         if let Ok(file) = u8::try_from(coord.0) {
//             if let Ok(rank) = u8::try_from(coord.1) {
//                 Coord::try_from((file, rank))
//             } else {
//                 Err(CoordError::InvalidTypeConversion(format!(
//                     "{} could not be converted to a valid u8 type",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidTypeConversion(format!(
//                 "{} could not be converted to a valid u8 type",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(i8, i8)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (i8, i8)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::try_from(coord.0 as u8) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1 as u8) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(i16, i16)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (i16, i16)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::try_from(coord.0 as u8) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1 as u8) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(i32, i32)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (i32, i32)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::try_from(coord.0 as u8) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1 as u8) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(i64, i64)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (i64, i64)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::try_from(coord.0 as u8) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1 as u8) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(i128, i128)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (i128, i128)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::try_from(coord.0 as u8) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1 as u8) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(isize, isize)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (isize, isize)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::try_from(coord.0 as u8) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1 as u8) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(&str, &str)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (&str, &str)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::from_str(coord.0) {
//             if let Ok(rank) = NonMaxU8::from_str(coord.1) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(char, char)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (char, char)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::from_str(&coord.0.to_string()) {
//             if let Ok(rank) = NonMaxU8::from_str(&coord.1.to_string()) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.1
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.0
//             )))
//         }
//     }
// }

// impl TryFrom<(char, u8)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (char, u8)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::from_str(&coord.0.to_string()) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.0
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.1
//             )))
//         }
//     }
// }

// impl TryFrom<(&str, u8)> for Coord {
//     type Error = CoordError;

//     fn try_from(coord: (&str, u8)) -> CoordResult<Coord> {
//         if let Ok(file) = NonMaxU8::from_str(coord.0) {
//             if let Ok(rank) = NonMaxU8::try_from(coord.1) {
//                 Ok(Coord { file, rank })
//             } else {
//                 Err(CoordError::InvalidRank(format!(
//                     "{} is an invalid rank",
//                     coord.0
//                 )))
//             }
//         } else {
//             Err(CoordError::InvalidFile(format!(
//                 "{} is an invalid file",
//                 coord.1
//             )))
//         }
//     }
// }

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_u8_coord() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::new(i, j).unwrap().to_u8_coord();
                assert_eq!((i as u8, j as u8), c);
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn to_string_coord() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::new(i, j).unwrap().to_string_coord();
                assert_eq!((i.to_string(), j.to_string()), c);
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn to_char_coord() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::new(i, j).unwrap().to_char_coord();
                assert_eq!(
                    (
                        char::from_digit(i as u32, 10).unwrap(),
                        char::from_digit(j as u32, 10).unwrap()
                    ),
                    c
                );
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn to_alphanumeric_coord() {
        for i in 0..8 {
            for j in 0..8 {
                let file = STR_FILES[i].chars().collect::<Vec<char>>()[0];
                let c = Coord::new(i as u8, j as u8)
                    .unwrap()
                    .to_alphanumeric_coord();
                assert_eq!((file, j as u8 + 1), c);
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_u8_u8() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::try_from((i, j)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_str_str() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::try_from((i.to_string().as_str(), i.to_string().as_str())).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_char_char() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::try_from((
                    char::from_digit(i, 10).unwrap(),
                    char::from_digit(j, 10).unwrap(),
                ))
                .unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_char_u8() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::try_from((char::from_digit(i, 10).unwrap(), j as u8)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_str_u8() {
        for i in 0..8 {
            for j in 0..8 {
                let c = Coord::try_from((i.to_string().as_str(), j as u8)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }
}
