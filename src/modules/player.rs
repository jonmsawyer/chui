//! File: `player.rs`
//!
//! Module: `player`
//!
//! Provides the struct and implementation of `Player`. Each
//! player requires a `last_name`, but optionally a `first_name`,
//! `name_prefix`, `name_suffix`, `age`, and `rating`.

use std::fmt;

use super::color::PieceColor;

/// Contains the information related to a player, such as piece
/// `color`, `last_name`, `first_name`, `name_prefix`,
/// `name_suffix`, `full_name`, `age`, and `rating`.
///
/// Initialize a new player like so:
///
/// Example:
///
/// ```
/// use chui::{PieceColor, Player};
/// let player = Player::new(
///     PieceColor::White,
///     "Johnson",
///     Some("Fred"),
///     None,
///     None,
///     Some(48),
///     None,
/// );
/// ```
#[derive(Debug)]
pub struct Player {
    /// The piece color of the player. One of `PieceColor::White`
    /// or `PieceColor::Black`.
    pub color: PieceColor,

    /// The last name of the player.
    pub last_name: String,

    /// The optional first name of the player.
    pub first_name: Option<String>,

    /// The optional name prefix of the player (e.g., "Dr.").
    pub name_prefix: Option<String>,

    /// The optional name suffix of the player (e.g., "Jr.").
    pub name_suffix: Option<String>,

    /// The full name of the player. Will contain all parts of the
    /// name if they are available (e.g., "Dr. Smitch, John Jr.").
    pub full_name: String,

    /// The optional age of the player. Useful in certain export
    /// formats.
    pub age: Option<u8>,

    /// The optional ELO or national rating of the player.
    pub rating: Option<u32>,
}

impl Player {
    /// Creates a new `Player` instance when provided with `color`,
    /// `last_name`, `first_name`, `name_prefix`, `name_suffix`,
    /// `age`, and `rating` information. Some fields are `Option`al.
    pub fn new(
        color: PieceColor,
        last_name: &str,
        first_name: Option<&str>,
        name_prefix: Option<&str>,
        name_suffix: Option<&str>,
        age: Option<u8>,
        rating: Option<u32>,
    ) -> Player {
        let mut full_name = String::from(last_name);

        let first_name = match first_name {
            Some(first_name) => {
                full_name += &format!(", {}", first_name);
                Some(first_name.to_string())
            }
            None => None,
        };

        let name_prefix = match name_prefix {
            Some(name_prefix) => {
                full_name = format!("{} {}", name_prefix, full_name);
                Some(name_prefix.to_string())
            }
            None => None,
        };

        let name_suffix = match name_suffix {
            Some(name_suffix) => {
                full_name = format!("{} {}", full_name, name_suffix);
                Some(name_suffix.to_string())
            }
            None => None,
        };

        Player {
            color,
            last_name: last_name.to_string(),
            first_name,
            name_prefix,
            name_suffix,
            full_name,
            rating,
            age,
        }
    }
}

/// Writes the full computed name of the player.
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = format!("{:?}: {}", self.color, self.full_name);
        if let Some(rating) = self.rating {
            name += &format!(" ({})", rating);
        } else {
            name += " (no rating)";
        }
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn full_name_is_computed_1() {
        let player = Player::new(
            PieceColor::White,
            "Smith",
            Some("John"),
            Some("Dr."),
            Some("III"),
            Some(47),
            Some(1500)
        );

        assert_eq!(
            format!("{}", player),
            String::from("White: Dr. Smith, John III (1500)")
        )
    }

    #[test]
    fn full_name_is_computed_2() {
        let player = Player::new(
            PieceColor::Black,
            "Smith",
            Some("John"),
            None,
            Some("IV"),
            Some(12),
            None,
        );

        assert_eq!(
            format!("{}", player),
            String::from("Black: Smith, John IV (no rating)")
        )
    }

    #[test]
    fn full_name_is_computed_3() {
        let player = Player::new(
            PieceColor::Black,
            "Smith",
            None,
            None,
            None,
            Some(27),
            Some(2639),
        );

        assert_eq!(
            format!("{}", player),
            String::from("Black: Smith (2639)")
        )
    }
}
