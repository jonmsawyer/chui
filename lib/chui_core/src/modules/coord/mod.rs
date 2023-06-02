//! Chui Core's Coordinate base type.

use std::fmt;
use std::str::FromStr;

pub use nonmax::NonMaxU8;

use crate::traits::Coordinate;
use crate::STR_FILES;

mod result;
pub use result::{CoordError, CoordResult};

/// Main [`Coord`] struct used to represent chess piece and board position. We use non-max
/// u8 values because indicies are 0-indexed and values of 8 are invalid for an iterable
/// of size 7.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Coordinate)]
pub struct Coord {
    file: NonMaxU8,
    rank: NonMaxU8,
}

impl Coord {
    /// Create a new [`Coord`] from a file and a rank parameter. File and rank parameters must
    /// evaluate to the u8 type and be less than 8 in value.
    pub fn new(file: u8, rank: u8) -> CoordResult<Coord> {
        if let Ok(file) = NonMaxU8::try_from(file) {
            if let Ok(rank) = NonMaxU8::try_from(rank) {
                if file.get() > 7 {
                    return Err(CoordError::InvalidFile(format!(
                        "{} is an invalid file index",
                        file
                    )));
                }

                if rank.get() > 7 {
                    return Err(CoordError::InvalidRank(format!(
                        "{} is an invalid rank index",
                        rank
                    )));
                }

                Ok(Coord { file, rank })
            } else {
                Err(CoordError::InvalidRank(format!(
                    "{} is an invalid rank index",
                    rank
                )))
            }
        } else {
            Err(CoordError::InvalidFile(format!(
                "{} is an invalid file index",
                file
            )))
        }
    }

    /// Create a new [`Coord`] with values set to zero.
    pub fn zero() -> Coord {
        Coord::new(0, 0).unwrap()
    }

    /// Return the value of [`Coord`]'s file.
    pub fn get_file(&self) -> u8 {
        self.file.get()
    }

    /// Return the value of [`Coord`]'s rank.
    pub fn get_rank(&self) -> u8 {
        self.rank.get()
    }

    /// Set the value of [`Coord`]'s file.
    pub fn set_file(&mut self, value: u8) -> CoordResult<u8> {
        if let Ok(file) = NonMaxU8::try_from(value) {
            if file.get() > 7 {
                return Err(CoordError::InvalidFile(format!(
                    "{} is an invalid file index",
                    file
                )));
            }

            self.file = file;
            Ok(self.file.get())
        } else {
            Err(CoordError::InvalidFile(format!(
                "{} is an invalid file index",
                value
            )))
        }
    }

    /// Set the value of [`Coord`]'s rank.
    pub fn set_rank(&mut self, value: u8) -> CoordResult<u8> {
        if let Ok(rank) = NonMaxU8::try_from(value) {
            if rank.get() > 7 {
                return Err(CoordError::InvalidRank(format!(
                    "{} is an invalid rank index",
                    rank
                )));
            }

            self.rank = rank;
            Ok(self.rank.get())
        } else {
            Err(CoordError::InvalidFile(format!(
                "{} is an invalid rank index",
                value
            )))
        }
    }

    /// Return a 2-tuple representing the [`Coord`] as index values.
    pub fn to_u8_index(&self) -> (u8, u8) {
        (self.file.get(), self.rank.get())
    }

    /// Return a 2-tuple representing the [`Coord`] as alphanumeric values via char and u8.
    pub fn to_char_u8_coord(&self) -> (char, u8) {
        (
            STR_FILES[self.file.get() as usize]
                .chars()
                .collect::<Vec<char>>()[0],
            self.rank.get() + 1,
        )
    }

    /// Return a 2-tuple representing the [`Coord`] as alphanumeric values via String and u8.
    pub fn to_string_u8_coord(&self) -> (String, u8) {
        (
            STR_FILES[self.file.get() as usize]
                .chars()
                .collect::<Vec<char>>()[0]
                .to_string(),
            self.rank.get() + 1,
        )
    }
}

/// Formats the position for white.
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (file, rank) = (self.file.get() as usize, self.rank.get());
        write!(f, "{}{}", STR_FILES[file], rank + 1)
    }
}

impl From<(NonMaxU8, NonMaxU8)> for Coord {
    fn from(coord: (NonMaxU8, NonMaxU8)) -> Coord {
        Coord {
            file: coord.0,
            rank: coord.1,
        }
    }
}

impl TryFrom<(&str, &str)> for Coord {
    type Error = CoordError;

    fn try_from(coord: (&str, &str)) -> CoordResult<Coord> {
        if let Ok(file) = NonMaxU8::from_str(coord.0) {
            if let Ok(rank) = NonMaxU8::from_str(coord.1) {
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
    }
}

impl TryFrom<(char, char)> for Coord {
    type Error = CoordError;

    fn try_from(coord: (char, char)) -> CoordResult<Coord> {
        if let Ok(file) = NonMaxU8::from_str(&coord.0.to_string()) {
            if let Ok(rank) = NonMaxU8::from_str(&coord.1.to_string()) {
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
    }
}

impl TryFrom<(char, u8)> for Coord {
    type Error = CoordError;

    fn try_from(coord: (char, u8)) -> CoordResult<Coord> {
        if let Ok(file) = NonMaxU8::from_str(&coord.0.to_string()) {
            if let Ok(rank) = NonMaxU8::try_from(coord.1) {
                Ok(Coord { file, rank })
            } else {
                Err(CoordError::InvalidRank(format!(
                    "{} is an invalid rank",
                    coord.0
                )))
            }
        } else {
            Err(CoordError::InvalidFile(format!(
                "{} is an invalid file",
                coord.1
            )))
        }
    }
}

impl TryFrom<(&str, u8)> for Coord {
    type Error = CoordError;

    fn try_from(coord: (&str, u8)) -> CoordResult<Coord> {
        if let Ok(file) = NonMaxU8::from_str(coord.0) {
            if let Ok(rank) = NonMaxU8::try_from(coord.1) {
                Ok(Coord { file, rank })
            } else {
                Err(CoordError::InvalidRank(format!(
                    "{} is an invalid rank",
                    coord.0
                )))
            }
        } else {
            Err(CoordError::InvalidFile(format!(
                "{} is an invalid file",
                coord.1
            )))
        }
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c_string = Coord::new(i, j).unwrap().to_string();
                let file = STR_FILES[i as usize];
                assert_eq!(format!("{}{}", file, j + 1), c_string);
                println!("({}, {}): {:?}", i, j, c_string);
            }
        }
    }

    #[test]
    fn to_str() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c_string = Coord::new(i, j).unwrap().to_string();
                let file = STR_FILES[i as usize];
                assert_eq!(format!("{}{}", file, j).as_str(), c_string.as_str());
                println!("({}, {}): {:?}", i, j, c_string);
            }
        }
    }

    #[test]
    fn to_char_u8_coord() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let file = STR_FILES[i as usize].chars().collect::<Vec<char>>()[0];
                let c = Coord::new(i as u8, j as u8).unwrap().to_char_u8_coord();
                assert_eq!((file, j + 1), c);
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn from_NonMaxU8() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let file = NonMaxU8::try_from(i).unwrap();
                let rank = NonMaxU8::try_from(j).unwrap();
                let c = Coord::from((file, rank));
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_u8_u8() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i, j)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_u16_u16() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as u16, j as u16)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_u32_u32() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as u32, j as u32)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_u64_u64() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as u64, j as u64)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_u128_u128() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as u128, j as u128)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_usize_usize() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as usize, j as usize)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_i8_i8() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as i8, j as i8)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_i16_i16() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as i16, j as i16)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_i32_i32() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as i32, j as i32)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_i64_i64() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as i64, j as i64)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_i128_i128() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as i64, j as i64)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_isize_isize() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i as isize, j as isize)).unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_from_u8_isize() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i, j as isize)).unwrap();
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

    #[test]
    #[allow(non_snake_case)]
    fn into_NonMaxU8() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let file = NonMaxU8::try_from(i).unwrap();
                let rank = NonMaxU8::try_from(j).unwrap();
                let c: Coord = (file, rank).into();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_u8_u8() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i, j).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_u16_u16() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as u16, j as u16).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_u32_u32() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as u32, j as u32).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_u64_u64() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as u64, j as u64).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_u128_u128() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as u128, j as u128).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_usize_usize() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as usize, j as usize).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_i8_i8() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as i8, j as i8).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_i16_i16() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as i16, j as i16).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_i32_i32() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as i32, j as i32).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_i64_i64() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as i64, j as i64).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_i128_i128() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as i128, j as i128).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_isize_isize() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i as isize, j as isize).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }

    #[test]
    fn try_into_u8_isize() {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i, j as isize).try_into().unwrap();
                println!("({}, {}): {:?}", i, j, c);
            }
        }
    }
}
