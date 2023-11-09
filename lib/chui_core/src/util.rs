//! Chui Core Utilities.

use rand::distributions::{Distribution, Uniform};

use crate::prelude::*;

/// Add thousands comma separators to a number. The number must match the following
/// regex: `^-?\d*(\.\d*)?$`. Returns None if it does not match that format.
/// Note that empty strings and just `-` are allowed.
///
/// # Credit
///
/// * Thanks to Timmmm: <https://stackoverflow.com/users/265521/timmmm>
/// * Code shared from: <https://stackoverflow.com/a/71500273/2085166>
/// * Modified by Jonathan Sawyer: <https://github.com/jonmsawyer>
pub fn num_sep(s: &str, separator: Option<char>) -> Option<String> {
    let sep = separator.map_or(',', |v| v);
    // Position of the `.`
    let dot = s.bytes().position(|c| c == b'.').unwrap_or(s.len());
    // Is the number negative (starts with `-`)?
    let negative = s.bytes().next() == Some(b'-');
    // The dot cannot be at the front if it is negative.
    if negative && dot == 0 {
        return None;
    };
    // Number of integer digits remaning (between the `-` or start and the `.`).
    let mut integer_digits_remaining = dot - negative as usize;
    // Output. Add capacity for commas. It's a slight over-estimate but that's fine.
    let mut out = String::with_capacity(s.len() + integer_digits_remaining / 3);

    // We can iterate on bytes because everything must be ASCII. Slightly faster.
    for (i, c) in s.bytes().enumerate() {
        match c {
            b'-' => {
                // `-` can only occur at the start of the string.
                if i != 0 {
                    return None;
                }
            }
            b'.' => {
                // Check we only have a dot at the expected position.
                // This return may happen if there are multiple dots.
                if i != dot {
                    return None;
                }
            }
            b'0'..=b'9' => {
                // Possibly add a comma.
                if integer_digits_remaining > 0 {
                    // Don't add a comma at the start of the string.
                    if i != negative as usize && integer_digits_remaining % 3 == 0 {
                        out.push(sep);
                    }
                    integer_digits_remaining -= 1;
                }
            }
            _ => {
                // No other characters allowed.
                return None;
            }
        }
        out.push(c as char);
    }
    Some(out)
}

/// Generate a vector of `Coordinate` pairs.
///
/// # Panics
///
/// This function will not panic.
pub fn gen_coords(num_coords: u64) -> Vec<(Coord, Coord)> {
    let mut rng = rand::thread_rng();
    let coord = Uniform::from(0..8);
    let mut coords: Vec<(Coord, Coord)> = Vec::new();

    for _ in 0..num_coords {
        coords.push((
            Coord::try_from((coord.sample(&mut rng), coord.sample(&mut rng))).unwrap(),
            Coord::try_from((coord.sample(&mut rng), coord.sample(&mut rng))).unwrap(),
        ));
    }

    coords
}

/// Benchmark the input Position.
pub fn piece_operation(position: &mut dyn Position, coords: &(Coord, Coord)) {
    let p1 = position.get_piece(Some(coords.0));
    let p2 = position.put_piece(p1, Some(coords.1));
    position.put_piece(p2, Some(coords.0));
}

/// `ArrayBitPosition` copies.
///
/// # Errors
///
/// This function will not error.
pub const fn array_bit_position_copy(position: &ArrayBitPosition) -> ChuiResult<ArrayBitPosition> {
    Ok(*position)
}

/// `BitPosition` copies.
///
/// # Errors
///
/// This function will not error.
pub const fn bit_position_copy(position: &BitPosition) -> ChuiResult<BitPosition> {
    Ok(*position)
}

/// `BitSetPosition` copies.
///
/// # Errors
///
/// This function will not error.
pub const fn bitset_position_copy(position: &BitSetPosition) -> ChuiResult<BitSetPosition> {
    Ok(*position)
}

/// `EasyPosition` copies.
///
/// # Errors
///
/// This function will not error.
pub const fn easy_2d_position_copy(position: &Easy2DPosition) -> ChuiResult<Easy2DPosition> {
    Ok(*position)
}

/// `Easy1DPosition` copies.
///
/// # Errors
///
/// This function will not error.
pub const fn easy_1d_position_copy(position: &Easy1DPosition) -> ChuiResult<Easy1DPosition> {
    Ok(*position)
}

/// `EnumPosition` copies.
///
/// # Errors
///
/// This function will not error.
pub const fn enum_position_copy(position: &EnumPosition) -> ChuiResult<EnumPosition> {
    Ok(*position)
}

#[cfg(test)]
mod test {
    use super::num_sep;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn number_separator() {
        //
        // With default separator.
        //

        assert_eq!(
            num_sep("12345678900.1234", None).as_deref(),
            Some("12,345,678,900.1234")
        );
        assert_eq!(num_sep("123.45", None).as_deref(), Some("123.45"));
        assert_eq!(num_sep("1234.56", None).as_deref(), Some("1,234.56"));
        assert_eq!(num_sep(".56", None).as_deref(), Some(".56"));
        assert_eq!(num_sep("56", None).as_deref(), Some("56"));
        assert_eq!(num_sep("567", None).as_deref(), Some("567"));
        assert_eq!(num_sep("5678", None).as_deref(), Some("5,678"));
        assert_eq!(num_sep("12345678", None).as_deref(), Some("12,345,678"));
        assert_eq!(num_sep("5678.", None).as_deref(), Some("5,678."));
        assert_eq!(num_sep(".0123", None).as_deref(), Some(".0123"));

        assert_eq!(num_sep("-123.45", None).as_deref(), Some("-123.45"));
        assert_eq!(num_sep("-1234.56", None).as_deref(), Some("-1,234.56"));
        assert_eq!(num_sep("-.56", None).as_deref(), Some("-.56"));
        assert_eq!(num_sep("-56", None).as_deref(), Some("-56"));
        assert_eq!(num_sep("-567", None).as_deref(), Some("-567"));
        assert_eq!(num_sep("-5678", None).as_deref(), Some("-5,678"));
        assert_eq!(num_sep("-12345678", None).as_deref(), Some("-12,345,678"));
        assert_eq!(num_sep("-5678.", None).as_deref(), Some("-5,678."));
        assert_eq!(num_sep("-.0123", None).as_deref(), Some("-.0123"));

        assert_eq!(num_sep("", None).as_deref(), Some(""));
        assert_eq!(num_sep("-", None).as_deref(), Some("-"));

        assert_eq!(num_sep("a", None).as_deref(), None);
        assert_eq!(num_sep("0-", None).as_deref(), None);
        assert_eq!(num_sep("0..1", None).as_deref(), None);
        assert_eq!(num_sep("0..1", None).as_deref(), None);
        assert_eq!(num_sep("01a", None).as_deref(), None);
        assert_eq!(num_sep("01.a", None).as_deref(), None);
        assert_eq!(num_sep(".0.", None).as_deref(), None);

        //
        // With custom separator.
        //

        assert_eq!(
            num_sep("12345678900.1234", Some('_')).as_deref(),
            Some("12_345_678_900.1234")
        );
        assert_eq!(num_sep("123.45", Some('_')).as_deref(), Some("123.45"));
        assert_eq!(num_sep("1234.56", Some('_')).as_deref(), Some("1_234.56"));
        assert_eq!(num_sep(".56", Some('_')).as_deref(), Some(".56"));
        assert_eq!(num_sep("56", Some('_')).as_deref(), Some("56"));
        assert_eq!(num_sep("567", Some('_')).as_deref(), Some("567"));
        assert_eq!(num_sep("5678", Some('_')).as_deref(), Some("5_678"));
        assert_eq!(
            num_sep("12345678", Some('_')).as_deref(),
            Some("12_345_678")
        );
        assert_eq!(num_sep("5678.", Some('_')).as_deref(), Some("5_678."));
        assert_eq!(num_sep(".0123", Some('_')).as_deref(), Some(".0123"));

        assert_eq!(num_sep("-123.45", Some('_')).as_deref(), Some("-123.45"));
        assert_eq!(num_sep("-1234.56", Some('_')).as_deref(), Some("-1_234.56"));
        assert_eq!(num_sep("-.56", Some('_')).as_deref(), Some("-.56"));
        assert_eq!(num_sep("-56", Some('_')).as_deref(), Some("-56"));
        assert_eq!(num_sep("-567", Some('_')).as_deref(), Some("-567"));
        assert_eq!(num_sep("-5678", Some('_')).as_deref(), Some("-5_678"));
        assert_eq!(
            num_sep("-12345678", Some('_')).as_deref(),
            Some("-12_345_678")
        );
        assert_eq!(num_sep("-5678.", Some('_')).as_deref(), Some("-5_678."));
        assert_eq!(num_sep("-.0123", Some('_')).as_deref(), Some("-.0123"));

        assert_eq!(num_sep("", Some('_')).as_deref(), Some(""));
        assert_eq!(num_sep("-", Some('_')).as_deref(), Some("-"));

        assert_eq!(num_sep("a", Some('_')).as_deref(), None);
        assert_eq!(num_sep("0-", Some('_')).as_deref(), None);
        assert_eq!(num_sep("0..1", Some('_')).as_deref(), None);
        assert_eq!(num_sep("0..1", Some('_')).as_deref(), None);
        assert_eq!(num_sep("01a", Some('_')).as_deref(), None);
        assert_eq!(num_sep("01.a", Some('_')).as_deref(), None);
        assert_eq!(num_sep(".0.", Some('_')).as_deref(), None);
    }
}
