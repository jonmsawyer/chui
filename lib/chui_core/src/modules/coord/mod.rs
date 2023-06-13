//! Chui Core's Coordinate base type.

use std::fmt;
// use std::str::FromStr;

pub use nonmax::NonMaxU8;

use crate::{traits::Coordinate, ChuiError, ChuiResult};

/// Main [`Coord`] struct used to represent chess piece and board position. We use non-max
/// u8 values because indicies are 0-indexed and values of 8 are invalid for an iterable
/// of size 7.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Coordinate)]
pub struct Coord {
    /// The file index of the board, restricted to values between 0..=7.
    file: NonMaxU8,

    /// The rank index of the board, restricted to values between 0..=7.
    rank: NonMaxU8,
}

impl Coord {
    /// Create a new [`Coord`] from a file and a rank parameter. File and rank parameters must
    /// evaluate to the u8 type and be less than 8 in value.
    pub fn new(file: u8, rank: u8) -> ChuiResult<Coord> {
        if file > 7 {
            return Err(ChuiError::InvalidFile(format!(
                "{} is an invalid file index (Coord::new)",
                file
            )));
        }

        if rank > 7 {
            return Err(ChuiError::InvalidRank(format!(
                "{} is an invalid rank index (Coord::new)",
                rank
            )));
        }

        let file = NonMaxU8::try_from(file).map_err(|_| {
            ChuiError::InvalidFile(format!("{} is an invalid file index (Coord::new)", file))
        })?;
        let rank = NonMaxU8::try_from(rank).map_err(|_| {
            ChuiError::InvalidRank(format!("{} is an invalid rank index (Coord::new)", rank))
        })?;

        Ok(Coord {
            file: NonMaxU8::try_from(file)?,
            rank: NonMaxU8::try_from(rank)?,
        })
    }

    /// Create a new [`Coord`] from an index that in the range of (0..64).
    pub fn new_from_idx(idx: u8) -> ChuiResult<Coord> {
        if idx >= 64 {
            return Err(ChuiError::IndexOutOfRange(format!(
                "{} is out of range (0..=63) (Coord::new_from_idx)",
                idx
            )));
        }

        let file_idx = idx % 8;
        let rank_idx = idx / 8;

        Coord::new(file_idx, rank_idx)
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
    pub fn set_file(&mut self, value: u8) -> ChuiResult<()> {
        if let Ok(file) = NonMaxU8::try_from(value) {
            if file.get() > 7 {
                return Err(ChuiError::InvalidFile(format!(
                    "{} is an invalid file index (Coord::set_file)",
                    file
                )));
            }

            self.file = file;

            Ok(())
        } else {
            Err(ChuiError::InvalidFile(format!(
                "{} is an invalid file index (Coord::set_file)",
                value
            )))
        }
    }

    /// Set the value of [`Coord`]'s rank.
    pub fn set_rank(&mut self, value: u8) -> ChuiResult<()> {
        if let Ok(rank) = NonMaxU8::try_from(value) {
            if rank.get() > 7 {
                return Err(ChuiError::InvalidRank(format!(
                    "{} is an invalid rank index (Coord::set_rank)",
                    rank
                )));
            }

            self.rank = rank;

            Ok(())
        } else {
            Err(ChuiError::InvalidFile(format!(
                "{} is an invalid rank index (Coord::set_rank)",
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
        ((self.file.get() + 'a' as u8) as char, self.rank.get() + 1)
    }

    /// Return true if the given coordinate is equal to this coordinate.
    pub fn is_eq(&self, coord: (char, u8)) -> bool {
        *self == Coord::try_from(coord).unwrap()
    }
}

/// Formats the position for a coordinate in Algebraic notation.
///
/// TODO: Change this to behave with the selected [`Parser`] in a given [`Game`].
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (self.file.get() + 'a' as u8) as char,
            self.rank.get() + 1
        )
    }
}

impl TryFrom<(NonMaxU8, NonMaxU8)> for Coord {
    type Error = ChuiError;

    fn try_from(coord: (NonMaxU8, NonMaxU8)) -> ChuiResult<Coord> {
        Coord::new(coord.0.get(), coord.1.get())
    }
}

impl TryFrom<(char, u8)> for Coord {
    type Error = ChuiError;

    fn try_from(coord: (char, u8)) -> ChuiResult<Coord> {
        Coord::new(
            (coord.0 as u8).wrapping_sub('a' as u8),
            coord.1.wrapping_sub(1),
        )
    }
}

impl TryFrom<(&str, u8)> for Coord {
    type Error = ChuiError;

    fn try_from(coord: (&str, u8)) -> ChuiResult<Coord> {
        if let Some(file) = coord.0.chars().next() {
            Coord::try_from((file, coord.1))
        } else {
            Err(ChuiError::InvalidCoords(format!(
                "{:?} is an invalid coordinate (try_from((&str, u8)))",
                coord
            )))
        }
    }
}

impl TryFrom<&str> for Coord {
    type Error = ChuiError;

    fn try_from(coord: &str) -> ChuiResult<Coord> {
        if coord.len() > 2 {
            return Err(ChuiError::InvalidCoords(format!(
                "{:?} is an invalid coordinate (try_from(&str))",
                coord
            )));
        }

        let mut c = coord.chars();

        if let (Some(file), Some(rank)) = (c.next(), c.next()) {
            Coord::try_from((file, rank))
        } else {
            Err(ChuiError::InvalidCoords(format!(
                "{:?} is an invalid coordinate (try_from(&str))",
                coord
            )))
        }
    }
}

impl TryFrom<(&str, &str)> for Coord {
    type Error = ChuiError;

    fn try_from(coord: (&str, &str)) -> ChuiResult<Coord> {
        if coord.0.len() > 1 || coord.1.len() > 1 {
            return Err(ChuiError::InvalidCoords(format!(
                "{:?} is an invalid coordinate (try_from((&str, &str)))",
                coord
            )));
        }

        if let (Some(file), Some(rank)) = (coord.0.chars().next(), coord.1.chars().next()) {
            Coord::try_from((file, rank))
        } else {
            Err(ChuiError::InvalidCoords(format!(
                "{:?} is an invalid coordinate (try_from((&str, &str)))",
                coord
            )))
        }
    }
}

impl TryFrom<(char, char)> for Coord {
    type Error = ChuiError;

    fn try_from(coord: (char, char)) -> ChuiResult<Coord> {
        Coord::try_from((coord.0, (coord.1 as u8).wrapping_sub('0' as u8)))
    }
}

impl PartialEq<(NonMaxU8, NonMaxU8)> for Coord {
    fn eq(&self, coord: &(NonMaxU8, NonMaxU8)) -> bool {
        if let Ok(new_coord) = Coord::try_from(*coord) {
            *self == new_coord
        } else {
            false
        }
    }
}

impl PartialEq<(char, u8)> for Coord {
    fn eq(&self, coord: &(char, u8)) -> bool {
        if let Ok(new_coord) = Coord::try_from(*coord) {
            *self == new_coord
        } else {
            false
        }
    }
}

impl PartialEq<(&str, u8)> for Coord {
    fn eq(&self, coord: &(&str, u8)) -> bool {
        if let Ok(new_coord) = Coord::try_from(*coord) {
            *self == new_coord
        } else {
            false
        }
    }
}

impl PartialEq<&str> for Coord {
    fn eq(&self, coord: &&str) -> bool {
        if let Ok(new_coord) = Coord::try_from(*coord) {
            *self == new_coord
        } else {
            false
        }
    }
}

impl PartialEq<(&str, &str)> for Coord {
    fn eq(&self, coord: &(&str, &str)) -> bool {
        if let Ok(new_coord) = Coord::try_from(*coord) {
            *self == new_coord
        } else {
            false
        }
    }
}

impl PartialEq<(char, char)> for Coord {
    fn eq(&self, coord: &(char, char)) -> bool {
        if let Ok(new_coord) = Coord::try_from(*coord) {
            *self == new_coord
        } else {
            false
        }
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::super::constants::STR_FILES;
    use super::*;

    #[test]
    fn to_string() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c_string = Coord::new(i, j)?.to_string();
                let file = STR_FILES[i as usize];
                assert_eq!(format!("{}{}", file, j + 1), c_string);
                println!("({}, {}): {:?}", i, j, c_string);
            }
        }
        Ok(())
    }

    #[test]
    fn to_str() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c_string = Coord::new(i, j)?.to_string();
                let file = STR_FILES[i as usize];
                assert_eq!(format!("{}{}", file, j + 1).as_str(), c_string.as_str());
                println!("({}, {}): {:?}", i, j, c_string);
            }
        }
        Ok(())
    }

    #[test]
    fn to_char_u8_coord() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let file = (i + 'a' as u8) as char;
                let c = Coord::new(i, j)?.to_char_u8_coord();
                assert_eq!((file, j + 1), c);
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn Coord_try_from_NonMaxU8() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let file = NonMaxU8::try_from(i)?;
                let rank = NonMaxU8::try_from(j)?;
                let c = Coord::try_from((file, rank))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn NonMaxU8_try_into_Coord() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let file = NonMaxU8::try_from(i)?;
                let rank = NonMaxU8::try_from(j)?;
                let c: Coord = (file, rank).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_str() -> ChuiResult<()> {
        for i in 'a'..='h' {
            for j in '1'..='8' {
                let c_string = format!("{}{}", i, j);
                let c = Coord::try_from(c_string.as_str())?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_str_str() -> ChuiResult<()> {
        for i in 'a'..='h' {
            for j in '1'..='8' {
                let c = Coord::try_from((i.to_string().as_str(), j.to_string().as_str()))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_str_u8() -> ChuiResult<()> {
        for i in 'a'..='h' {
            for j in 1..=8_u8 {
                let c = Coord::try_from((i.to_string().as_str(), j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_char_char() -> ChuiResult<()> {
        for i in 'a'..='h' {
            for j in '1'..='8' {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_char_u8() -> ChuiResult<()> {
        for i in 'a'..='h' {
            for j in 1..=8_u8 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_u8_u8() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_u16_u16() -> ChuiResult<()> {
        for i in 0..8_u16 {
            for j in 0..8_u16 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_u32_u32() -> ChuiResult<()> {
        for i in 0..8_u32 {
            for j in 0..8_u32 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_u64_u64() -> ChuiResult<()> {
        for i in 0..8_u64 {
            for j in 0..8_u64 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_u128_u128() -> ChuiResult<()> {
        for i in 0..8_u128 {
            for j in 0..8_u128 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_usize_usize() -> ChuiResult<()> {
        for i in 0..8_usize {
            for j in 0..8_usize {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_i8_i8() -> ChuiResult<()> {
        for i in 0..8_i8 {
            for j in 0..8_i8 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_i16_i16() -> ChuiResult<()> {
        for i in 0..8_i16 {
            for j in 0..8_i16 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_i32_i32() -> ChuiResult<()> {
        for i in 0..8_i32 {
            for j in 0..8_i32 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_i64_i64() -> ChuiResult<()> {
        for i in 0..8_i64 {
            for j in 0..8_i64 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_i128_i128() -> ChuiResult<()> {
        for i in 0..8_i128 {
            for j in 0..8_i128 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_isize_isize() -> ChuiResult<()> {
        for i in 0..8_isize {
            for j in 0..8_isize {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_u8_isize() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_isize {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_from_isize_u8() -> ChuiResult<()> {
        for i in 0..8_isize {
            for j in 0..8_u8 {
                let c = Coord::try_from((i, j))?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_u8_u8() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_u16_u16() -> ChuiResult<()> {
        for i in 0..8_u16 {
            for j in 0..8_u16 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_u32_u32() -> ChuiResult<()> {
        for i in 0..8_u32 {
            for j in 0..8_u32 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_u64_u64() -> ChuiResult<()> {
        for i in 0..8_u64 {
            for j in 0..8_u64 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_u128_u128() -> ChuiResult<()> {
        for i in 0..8_u128 {
            for j in 0..8_u128 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_usize_usize() -> ChuiResult<()> {
        for i in 0..8_usize {
            for j in 0..8_usize {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_i8_i8() -> ChuiResult<()> {
        for i in 0..8_i8 {
            for j in 0..8_i8 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_i16_i16() -> ChuiResult<()> {
        for i in 0..8_i16 {
            for j in 0..8_i16 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_i32_i32() -> ChuiResult<()> {
        for i in 0..8_i32 {
            for j in 0..8_i32 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_i64_i64() -> ChuiResult<()> {
        for i in 0..8_i64 {
            for j in 0..8_i64 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_i128_i128() -> ChuiResult<()> {
        for i in 0..8_i128 {
            for j in 0..8_i128 {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_isize_isize() -> ChuiResult<()> {
        for i in 0..8_isize {
            for j in 0..8_isize {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }

    #[test]
    fn try_into_u8_isize() -> ChuiResult<()> {
        for i in 0..8_u8 {
            for j in 0..8_isize {
                let c: Coord = (i, j).try_into()?;
                println!("({}, {}): {:?}", i, j, c);
            }
        }
        Ok(())
    }
}
