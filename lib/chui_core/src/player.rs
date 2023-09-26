//! Provides the struct and implementation of `Player`. Each
//! player requires a `last_name`, but optionally a `first_name`,
//! `name_prefix`, `name_suffix`, `age`, and `rating`.

use std::fmt;

use super::piece::Color;

/// Contains the information related to a player, such as piece
/// `color`, `name`, `age`, and Elo `rating`.
///
/// Initialize a new player like so:
///
/// Example:
///
/// ```
/// use chui_core::{Color, Player};
///
/// let player = Player::new(
///     Color::White,
///     Some("Fred Johnson"),
///     Some(48),
///     None,
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    /// The piece color of the player. One of `Color::White`
    /// or `Color::Black`.
    pub color: Color,

    /// The name of the player. All UTF-8 input is valid.
    pub name: Option<String>,

    /// The optional age of the player. Useful in certain export
    /// formats.
    pub age: Option<u8>,

    /// The optional ELO or national rating of the player.
    pub rating: Option<u32>,
}

impl Player {
    /// Creates a new `Player` instance when provided with `color`.
    /// Most fields are `Option`al.
    pub fn new(color: Color, name: Option<&str>, age: Option<u8>, rating: Option<u32>) -> Player {
        Player {
            color,
            name: name.map(|the_name| the_name.to_string()),
            age,
            rating,
        }
    }
}

/// Writes the full computed name of the player.
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = format!("{:?}", self.color);

        let name = &self
            .name
            .as_ref()
            .map_or_else(|| "(no name)".to_string(), |name| name.to_string());

        let age = self
            .age
            .map_or_else(String::new, |age| format!(" (Age {})", age));

        let rating = self.rating.map_or_else(
            || "(no Elo rating)".to_string(),
            |rating| format!("({} Elo)", rating),
        );

        write!(f, "{}: {}{} {}", color, name, age, rating)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn full_name_is_computed_name_age_rating() {
        let player = Player::new(
            Color::White,
            Some("Dr. John Smith III"),
            Some(47),
            Some(1500),
        );

        assert_eq!(
            format!("{}", player),
            String::from("White: Dr. John Smith III (Age 47) (1500 Elo)")
        );
    }

    #[test]
    fn full_name_is_computed_name_age_no_rating() {
        let player = Player::new(Color::Black, Some("John Smith IV"), Some(12), None);

        assert_eq!(
            format!("{}", player),
            String::from("Black: John Smith IV (Age 12) (no Elo rating)")
        );
    }

    #[test]
    fn full_name_is_computed_name_no_age_rating() {
        let player = Player::new(
            Color::Black,
            Some("Billy Bob Joe Bob Jr."),
            None,
            Some(2639),
        );

        assert_eq!(
            format!("{}", player),
            String::from("Black: Billy Bob Joe Bob Jr. (2639 Elo)")
        );
    }

    #[test]
    fn full_name_is_computed_name_no_age_no_rating() {
        let player = Player::new(Color::Black, Some("Smith"), None, None);

        assert_eq!(
            format!("{}", player),
            String::from("Black: Smith (no Elo rating)")
        );
    }

    #[test]
    fn full_name_is_computed_no_name_age_rating() {
        let player = Player::new(Color::White, None, Some(47), Some(1500));

        assert_eq!(
            format!("{}", player),
            String::from("White: (no name) (Age 47) (1500 Elo)")
        );
    }

    #[test]
    fn full_name_is_computed_no_name_age_no_rating() {
        let player = Player::new(Color::Black, None, Some(12), None);

        assert_eq!(
            format!("{}", player),
            String::from("Black: (no name) (Age 12) (no Elo rating)")
        );
    }

    #[test]
    fn full_name_is_computed_no_name_no_age_rating() {
        let player = Player::new(Color::Black, None, None, Some(2639));

        assert_eq!(
            format!("{}", player),
            String::from("Black: (no name) (2639 Elo)")
        );
    }

    #[test]
    fn full_name_is_computed_no_name_no_age_no_rating() {
        let player = Player::new(Color::Black, None, None, None);

        assert_eq!(
            format!("{}", player),
            String::from("Black: (no name) (no Elo rating)")
        );
    }
}
