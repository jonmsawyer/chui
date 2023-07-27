//! Provides the `MoveGenerator` struct. `MoveGenerator`
//! generates all possible Algebraic and Coordinate Notation
//! moves via `MoveGenerator::generate_move_list()`.
//!
//! I realize there is a lot of duplicated code in this module,
//! but I've separated it out for easier inspection of the output
//! and for readabilty purposes.

use std::collections::HashSet;
use std::fmt;
use std::fmt::Write as _; // import without risk of name clashing

/// Contains all the needed information to generate all
/// possible Algebraic and Coordinate Notation moves.
///
/// Example:
///
/// ```
/// use chui::MoveGenerator;
///
/// let g = MoveGenerator::generate_move_list();
/// let (answer, _reason) = g.validate_moves();
/// assert!(answer);
///
/// println!("{:?}", g.move_list);
/// ```
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MoveGenerator<'a> {
    /// The 8 files (e.g., `a`, `b`, `c`, `d`, `e`, `f`, `g`, `h`).
    pub files: [&'a str; 8],

    /// The 8 ranks (e.g., `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`).
    pub ranks: [&'a str; 8],

    /// The 5 pieces (excluding pawn) (e.g., `K`, `Q`, `R`, `B`, `N`).
    pub pieces: [&'a str; 5],

    /// The various supported promotion notations
    /// (e.g., `/`, `=`, `\`, etc.).
    pub promotion_notation: [&'a str; 4],

    /// The 4 promotion pieces (e.g., `Q`, `R`, `B`, `N`).
    pub promotion_pieces: [&'a str; 4],

    /// The check notation (`+`).
    pub check: &'a str,

    /// The 2 check mate notations (`++`, `#`).
    pub check_mate: [&'a str; 4],

    /// The move notation (`-`).
    pub move_notation: &'a str,

    /// The capture notation (`x`).
    pub capture: &'a str,

    /// The 4 castling notations (2 valid, 2 invalid)
    /// (`O-O`, `0-0`, `O-O-O`, `0-0-0`).
    pub castle: [&'a str; 4],

    /// Raw castling notation (`0` or `O`).
    pub castle_notation: [&'a str; 3],

    /// A vector containing Strings of the resulting generated
    /// move list.
    pub move_list: Vec<String>,
}

/// Writes the display of the generated chess moves by a certain prefix.
///
/// TODO: Clean this up if possible.
impl fmt::Display for MoveGenerator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmt_str = String::new();
        let mut a_fmt_str = String::new();
        let mut a_count = 0;
        let mut b_fmt_str = String::new();
        let mut b_count = 0;
        let mut c_fmt_str = String::new();
        let mut c_count = 0;
        let mut d_fmt_str = String::new();
        let mut d_count = 0;
        let mut e_fmt_str = String::new();
        let mut e_count = 0;
        let mut f_fmt_str = String::new();
        let mut f_count = 0;
        let mut g_fmt_str = String::new();
        let mut g_count = 0;
        let mut h_fmt_str = String::new();
        let mut h_count = 0;
        let mut king_fmt_str = String::new();
        let mut king_count = 0;
        let mut queen_fmt_str = String::new();
        let mut queen_count = 0;
        let mut rook_fmt_str = String::new();
        let mut rook_count = 0;
        let mut bishop_fmt_str = String::new();
        let mut bishop_count = 0;
        let mut knight_fmt_str = String::new();
        let mut knight_count = 0;
        let mut castle_valid_fmt_str = String::new();
        let mut castle_valid_count = 0;
        let mut castle_invalid_fmt_str = String::new();
        let mut castle_invalid_count = 0;
        let tab = "    ";
        fmt_str += "[";
        for output in self.move_list.iter() {
            if output.starts_with('a') {
                let _ = write!(a_fmt_str, "{}, ", output);
                a_count += 1;
            }
            if output.starts_with('b') {
                let _ = write!(b_fmt_str, "{}, ", output);
                b_count += 1;
            }
            if output.starts_with('c') {
                let _ = write!(c_fmt_str, "{}, ", output);
                c_count += 1;
            }
            if output.starts_with('d') {
                let _ = write!(d_fmt_str, "{}, ", output);
                d_count += 1;
            }
            if output.starts_with('e') {
                let _ = write!(e_fmt_str, "{}, ", output);
                e_count += 1;
            }
            if output.starts_with('f') {
                let _ = write!(f_fmt_str, "{}, ", output);
                f_count += 1;
            }
            if output.starts_with('g') {
                let _ = write!(g_fmt_str, "{}, ", output);
                g_count += 1;
            }
            if output.starts_with('h') {
                let _ = write!(h_fmt_str, "{}, ", output);
                h_count += 1;
            }
            if output.starts_with('K') {
                let _ = write!(king_fmt_str, "{}, ", output);
                king_count += 1;
            }
            if output.starts_with('Q') {
                let _ = write!(queen_fmt_str, "{}, ", output);
                queen_count += 1;
            }
            if output.starts_with('R') {
                let _ = write!(rook_fmt_str, "{}, ", output);
                rook_count += 1;
            }
            if output.starts_with('B') {
                let _ = write!(bishop_fmt_str, "{}, ", output);
                bishop_count += 1;
            }
            if output.starts_with('N') {
                let _ = write!(knight_fmt_str, "{}, ", output);
                knight_count += 1;
            }
            if output.starts_with('O') {
                let _ = write!(castle_valid_fmt_str, "{}, ", output);
                castle_valid_count += 1;
            }
            if output.starts_with('0') {
                let _ = write!(castle_invalid_fmt_str, "{}, ", output);
                castle_invalid_count += 1;
            }
        }
        let _ = write!(
            fmt_str,
            "\n{}({}) a: [\n{}{}{}\n{}],",
            tab, a_count, tab, tab, a_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) b: [\n{}{}{}\n{}],",
            tab, b_count, tab, tab, b_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) c: [\n{}{}{}\n{}],",
            tab, c_count, tab, tab, c_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) d: [\n{}{}{}\n{}],",
            tab, d_count, tab, tab, d_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) e: [\n{}{}{}\n{}],",
            tab, e_count, tab, tab, e_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) f: [\n{}{}{}\n{}],",
            tab, f_count, tab, tab, f_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) g: [\n{}{}{}\n{}],",
            tab, g_count, tab, tab, g_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) h: [\n{}{}{}\n{}],",
            tab, h_count, tab, tab, h_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) K: [\n{}{}{}\n{}],",
            tab, king_count, tab, tab, king_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) Q: [\n{}{}{}\n{}],",
            tab, queen_count, tab, tab, queen_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) R: [\n{}{}{}\n{}],",
            tab, rook_count, tab, tab, rook_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) B: [\n{}{}{}\n{}],",
            tab, bishop_count, tab, tab, bishop_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) N: [\n{}{}{}\n{}],",
            tab, knight_count, tab, tab, knight_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) O: [\n{}{}{}\n{}],",
            tab, castle_valid_count, tab, tab, castle_valid_fmt_str, tab
        );
        let _ = write!(
            fmt_str,
            "\n{}({}) 0: [\n{}{}{}\n{}],",
            tab, castle_invalid_count, tab, tab, castle_invalid_fmt_str, tab
        );
        fmt_str += "\n]";
        write!(f, "{}", fmt_str)
    }
}

impl<'a> MoveGenerator<'a> {
    /// Returns a new `MoveGenerator<'a>` object. All the information
    /// needed to generate possible chess notation moves is contained.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let g = MoveGenerator::new();
    ///
    /// assert_eq!(
    ///     g,
    ///     MoveGenerator {
    ///         files: ["a", "b", "c", "d", "e", "f", "g", "h"],
    ///         ranks: ["1", "2", "3", "4", "5", "6", "7", "8"],
    ///         pieces: ["K", "Q", "R", "B", "N"],
    ///         promotion_notation: ["", "=", "/", "\\"],
    ///         promotion_pieces: ["Q", "R", "B", "N"],
    ///         check: "+",
    ///         check_mate: ["++", "#", "≠", "‡"],
    ///         move_notation: "-",
    ///         capture: "x",
    ///         castle: ["O-O", "0-0", "O-O-O", "0-0-0"],
    ///         castle_notation: ["0", "O", "o"],
    ///         move_list: Vec::<String>::new(),
    ///     }
    /// );
    /// ```
    pub const fn new() -> MoveGenerator<'a> {
        MoveGenerator {
            files: ["a", "b", "c", "d", "e", "f", "g", "h"],
            ranks: ["1", "2", "3", "4", "5", "6", "7", "8"],
            pieces: ["K", "Q", "R", "B", "N"],
            promotion_notation: ["", "=", "/", "\\"],
            promotion_pieces: ["Q", "R", "B", "N"],
            check: "+",
            check_mate: ["++", "#", "≠", "‡"],
            move_notation: "-",
            capture: "x",
            castle: [
                "O-O",   // King side (valid)
                "0-0",   // King side (invalid, but supported)
                "O-O-O", // Queen side (valid)
                "0-0-0", // Queen side (invalid, but supported)
            ],
            castle_notation: ["0", "O", "o"],
            move_list: Vec::<String>::new(),
        }
    }

    /// Validate the list of generated moves ensuring no move is
    /// duplicated in the list. I'm trying to avoid the use of
    /// `move_list.sort()` and `move_list.dedup()`, but the logic
    /// in `generate_square_to_square_moves()` and
    /// `generate_square_to_square_captures()` isn't quite right
    /// yet as they provide some duplicates. Still, it's working
    /// as tested and expected.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::generate_move_list();
    /// let (answer, _reason) = g.validate_moves();
    ///
    /// assert!(answer);
    /// ```
    pub fn validate_moves(&self) -> (bool, String) {
        let mut set = HashSet::<&String>::new();
        for item in self.move_list.iter() {
            if !set.insert(item) {
                return (false, format!("{} already exists.", item));
            }
        }

        (true, "Valid.".to_string())
    }

    #[allow(clippy::similar_names)]
    /// Checks to see if the move from the given
    /// `{file_a}{rank_b}{-,x}{file_b}{rank_b}` is valid.
    /// Return `true` on valid, `false` on invalid.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let file_a: usize = 0; let rank_a: usize = 6; // a7
    /// let file_b: usize = 0; let rank_b: usize = 7; // a8
    ///
    /// assert!(
    ///     MoveGenerator::move_is_valid(
    ///         file_a, rank_a, file_b, rank_b
    ///     )
    /// );
    /// ```
    pub const fn move_is_valid(
        file_a_idx: usize,
        rank_a_idx: usize,
        file_b_idx: usize,
        rank_b_idx: usize,
    ) -> bool {
        // We cast to `isize` for negative Coordinate checking.
        let file_a_idx = file_a_idx as isize;
        let file_b_idx = file_b_idx as isize;
        let rank_a_idx = rank_a_idx as isize;
        let rank_b_idx = rank_b_idx as isize;

        // If move-from and move-to is the same, move is invalid.
        if file_a_idx == file_b_idx && rank_a_idx == rank_b_idx {
            return false;
        }

        // If on same file or on same rank, move is valid.
        if file_a_idx == file_b_idx || rank_a_idx == rank_b_idx {
            return true;
        }

        // If on same diagonal, move is valid.
        if (file_b_idx - file_a_idx).abs() == (rank_a_idx - rank_b_idx).abs()
            && (file_b_idx - file_a_idx).abs() == (rank_b_idx - rank_a_idx).abs()
        {
            return true;
        }

        #[allow(clippy::nonminimal_bool)] // Because readability is better than optimization.
        // If within knight move, move is valid.
        if file_a_idx + 1 == file_b_idx && rank_a_idx + 2 == rank_b_idx
            || file_a_idx + 1 == file_b_idx && rank_a_idx - 2 == rank_b_idx
            || file_a_idx - 1 == file_b_idx && rank_a_idx + 2 == rank_b_idx
            || file_a_idx - 1 == file_b_idx && rank_a_idx - 2 == rank_b_idx
            || file_a_idx + 2 == file_b_idx && rank_a_idx + 1 == rank_b_idx
            || file_a_idx + 2 == file_b_idx && rank_a_idx - 1 == rank_b_idx
            || file_a_idx - 2 == file_b_idx && rank_a_idx + 1 == rank_b_idx
            || file_a_idx - 2 == file_b_idx && rank_a_idx - 1 == rank_b_idx
        {
            return true;
        }

        false
    }

    #[allow(clippy::similar_names)]
    /// Checks to see if the move from the given
    /// `{file_a}{rank_a}{-,x}{file_b}{rank_b}` is a valid
    /// pawn promotion move.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let file_a: usize = 0; let rank_a: usize = 6; // a7
    /// let file_b: usize = 0; let rank_b: usize = 7; // a8
    ///
    /// assert!(
    ///     MoveGenerator::move_is_valid_promotion(
    ///         file_a, rank_a, file_b, rank_b
    ///     )
    /// );
    /// ```
    pub const fn move_is_valid_promotion(
        file_a_idx: usize,
        rank_a_idx: usize,
        file_b_idx: usize,
        rank_b_idx: usize,
    ) -> bool {
        // Cast to `isize` due to negative bounds checks.
        let file_a_idx = file_a_idx as isize;
        let rank_a_idx = rank_a_idx as isize;
        let file_b_idx = file_b_idx as isize;
        let rank_b_idx = rank_b_idx as isize;

        // Pawns must move forward and be on proper starting ranks.
        if !(rank_a_idx == 6 && rank_b_idx == 7 || rank_a_idx == 1 && rank_b_idx == 0) {
            return false;
        }

        #[allow(clippy::nonminimal_bool)] // Because readability is better than optimization.
        // Ending file cannot be more than one apart from
        // starting file.
        //
        // TODO: Double check this.
        if !(file_a_idx + 1 == file_b_idx && file_a_idx < 7)
            && !(file_a_idx - 1 == file_b_idx && file_a_idx > 0)
            && file_a_idx != file_b_idx
        {
            return false;
        }

        true
    }

    /// Generate all pawn moves (e.g., a1, a2, ..., h7, h8).
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_pawn_moves();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_pawn_moves(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                // If the move ends on the first or eigth rank,
                // we process pawn promotion here.
                if *rank == "1" || *rank == "8" {
                    for piece in self.promotion_pieces.iter() {
                        for promotion in self.promotion_notation.iter() {
                            // a8Q, a8R, etc.
                            self.move_list
                                .push(format!("{}{}{}{}", file, rank, promotion, piece));

                            // a8Q+, a8R+, etc.
                            self.move_list.push(format!(
                                "{}{}{}{}{}",
                                file, rank, promotion, piece, self.check
                            ));

                            for mate in self.check_mate.iter() {
                                // a8Q++, a8/R++, a8=Q#, a8R#, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}",
                                    file, rank, promotion, piece, mate,
                                ));
                            }
                        }
                    }
                }
                // Process all other moves.
                else {
                    // a2, a3, ..., h6, h7
                    self.move_list.push(format!("{}{}", file, rank));

                    // a2+, a3+, ..., h6+, h7+
                    self.move_list
                        .push(format!("{}{}{}", file, rank, self.check));

                    for mate in self.check_mate.iter() {
                        // a2++, a2#, ..., g2++, g2#
                        self.move_list.push(format!("{}{}{}", file, rank, mate));
                    }
                }
            }
        }
    }

    /// Generate all pawn captures (e.g., axb8, cxd4+, fxg1Q#, etc.).
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_pawn_captures();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_pawn_captures(&mut self) {
        for (file_idx, file) in self.files.iter().enumerate() {
            for rank in self.ranks.iter() {
                // If the move ends on the first or eigth rank,
                // we process pawn promotion here.
                if *rank == "1" || *rank == "8" {
                    for piece in self.promotion_pieces.iter() {
                        for promotion in self.promotion_notation.iter() {
                            // Process all pawn captures from all files
                            // that are != "a".
                            if *file != "a" {
                                // bxa8Q, bxa8=R, bxg7/R, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}{}",
                                    file,
                                    self.capture,
                                    self.files[file_idx - 1],
                                    rank,
                                    promotion,
                                    piece
                                ));

                                // bxa8Q+, bxa8=R+, bxa8/R+, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}{}{}",
                                    file,
                                    self.capture,
                                    self.files[file_idx - 1],
                                    rank,
                                    promotion,
                                    piece,
                                    self.check
                                ));
                            }

                            // Process all pawn captures from all files
                            // that are != "h".
                            if *file != "h" {
                                // bxc8Q, bxc8R, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}{}",
                                    file,
                                    self.capture,
                                    self.files[file_idx + 1],
                                    rank,
                                    promotion,
                                    piece
                                ));

                                // bxc8Q+, bxc8=R+, bxd1/R+, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}{}{}",
                                    file,
                                    self.capture,
                                    self.files[file_idx + 1],
                                    rank,
                                    promotion,
                                    piece,
                                    self.check
                                ));
                            }

                            for mate in self.check_mate.iter() {
                                // Process all pawn captures from all files
                                // that are != "a".
                                if *file != "a" {
                                    // bxa8Q++, bxa8=R++, bxa8/Q#, etc.
                                    self.move_list.push(format!(
                                        "{}{}{}{}{}{}{}",
                                        file,
                                        self.capture,
                                        self.files[file_idx - 1],
                                        rank,
                                        promotion,
                                        piece,
                                        mate,
                                    ));
                                }

                                // Process all pawn captures from all files
                                // that are != "a".
                                if *file != "h" {
                                    // bxc8Q++, bxc8=R++, bxc8/Q#, etc.
                                    self.move_list.push(format!(
                                        "{}{}{}{}{}{}{}",
                                        file,
                                        self.capture,
                                        self.files[file_idx + 1],
                                        rank,
                                        promotion,
                                        piece,
                                        mate,
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    // Process all pawn captures from all files
                    // that are != "a".
                    if *file != "a" {
                        // bxa2, bxa3, ..., hxg6, hxg7
                        self.move_list.push(format!(
                            "{}{}{}{}",
                            file,
                            self.capture,
                            self.files[file_idx - 1],
                            rank,
                        ));

                        // bxa2+, bxa3+, ..., hxg6+, hxg7+
                        self.move_list.push(format!(
                            "{}{}{}{}{}",
                            file,
                            self.capture,
                            self.files[file_idx - 1],
                            rank,
                            self.check
                        ));
                    }

                    // Process all pawn captures from all files
                    // that are != "a".
                    if *file != "h" {
                        // bxa2, bxa3, ..., hxg6, hxg7
                        self.move_list.push(format!(
                            "{}{}{}{}",
                            file,
                            self.capture,
                            self.files[file_idx + 1],
                            rank,
                        ));

                        // bxa2+, bxa3+, ..., hxg6+, hxg7+
                        self.move_list.push(format!(
                            "{}{}{}{}{}",
                            file,
                            self.capture,
                            self.files[file_idx + 1],
                            rank,
                            self.check
                        ));
                    }

                    for mate in self.check_mate.iter() {
                        // Process all pawn captures from all files
                        // that are != "a".
                        if *file != "a" {
                            // bxa2++, bxa2#, bxc2++, bxc2#, ... hxg7++, hxg7#
                            self.move_list.push(format!(
                                "{}{}{}{}{}",
                                file,
                                self.capture,
                                self.files[file_idx - 1],
                                rank,
                                mate
                            ));
                        }

                        // Process all pawn captures from all files
                        // that are != "a".
                        if *file != "h" {
                            // axb2++, axb2#, axb2++, bxc2#, ... gxh7++, gxh7#
                            self.move_list.push(format!(
                                "{}{}{}{}{}",
                                file,
                                self.capture,
                                self.files[file_idx + 1],
                                rank,
                                mate
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Generate all king moves (e.g., Ka6, Kg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_king_moves();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_king_moves(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "K";

                // Ka1, Kh7, etc.
                self.move_list.push(format!("{}{}{}", piece, file, rank,));

                // Ka1+, Kh7+, etc.
                // Note: The king itself cannot check, but there
                //       can be a discovered check.
                self.move_list
                    .push(format!("{}{}{}{}", piece, file, rank, self.check,));

                for mate in self.check_mate.iter() {
                    // Ka1++, Kh7#, etc.
                    // Note: The king itself cannot check mate, but
                    //       there can be a discovered check mate.
                    self.move_list
                        .push(format!("{}{}{}{}", piece, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all king captures (e.g., Kxa6, Kxg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_king_captures();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_king_captures(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "K";

                // Kxa1, Kxh7, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, self.capture, file, rank,));

                // Kxa1+, Kxh7+, etc.
                // Note: The king itself cannot check, but there
                //       can be a discovered check after capture.
                self.move_list.push(format!(
                    "{}{}{}{}{}",
                    piece, self.capture, file, rank, self.check,
                ));

                for mate in self.check_mate.iter() {
                    // Kxa1++, Kxh7#, etc.
                    // Note: The king itself cannot check mate, but
                    //       there can be a discovered check mate
                    //       after capture.
                    self.move_list
                        .push(format!("{}{}{}{}{}", piece, self.capture, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all queen moves (e.g., Qa6, Qg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_queen_moves();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_queen_moves(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "Q";

                // Qa1, Qh7, etc.
                self.move_list.push(format!("{}{}{}", piece, file, rank,));

                // Qa1+, Qh7+, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, file, rank, self.check,));

                for mate in self.check_mate.iter() {
                    // Qa1++, Qh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}", piece, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all queen captures (e.g., Qxa6, Qxg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_queen_captures();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_queen_captures(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "Q";

                // Qxa1, Qxh7, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, self.capture, file, rank,));

                // Qxa1+, Qxh7+, etc.
                self.move_list.push(format!(
                    "{}{}{}{}{}",
                    piece, self.capture, file, rank, self.check,
                ));

                for mate in self.check_mate.iter() {
                    // Qxa1++, Qxh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}{}", piece, self.capture, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all rook moves (e.g., Ra6, Rg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_rook_moves();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_rook_moves(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "R";

                // Ra1, Rh7, etc.
                self.move_list.push(format!("{}{}{}", piece, file, rank,));

                // Ra1+, Rh7+, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, file, rank, self.check,));

                for mate in self.check_mate.iter() {
                    // Ra1++, Rh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}", piece, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all rook captures (e.g., Rxa6, Rxg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_rook_captures();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_rook_captures(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "R";

                // Rxa1, Rxh7, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, self.capture, file, rank,));

                // Rxa1+, Rxh7+, etc.
                self.move_list.push(format!(
                    "{}{}{}{}{}",
                    piece, self.capture, file, rank, self.check,
                ));

                for mate in self.check_mate.iter() {
                    // Rxa1++, Rxh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}{}", piece, self.capture, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all bishop moves (e.g., Ba6, Bg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_bishop_moves();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_bishop_moves(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "B";

                // Ba1, Bh7, etc.
                self.move_list.push(format!("{}{}{}", piece, file, rank,));

                // Ba1+, Bh7+, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, file, rank, self.check,));

                for mate in self.check_mate.iter() {
                    // Ba1++, Bh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}", piece, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all bishop captures (e.g., Bxa6, Bxg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_bishop_captures();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_bishop_captures(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "B";

                // Bxa1, Bxh7, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, self.capture, file, rank,));

                // Bxa1+, Bxh7+, etc.
                self.move_list.push(format!(
                    "{}{}{}{}{}",
                    piece, self.capture, file, rank, self.check,
                ));

                for mate in self.check_mate.iter() {
                    // Bxa1++, Bxh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}{}", piece, self.capture, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all knight moves (e.g., Na6, Ng1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_knight_moves();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_knight_moves(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "N";

                // Na1, Nh7, etc.
                self.move_list.push(format!("{}{}{}", piece, file, rank,));

                // Na1+, Nh7+, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, file, rank, self.check,));

                for mate in self.check_mate.iter() {
                    // Na1++, Nh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}", piece, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all knight captures (e.g., Nxa6, Nxg1, etc.).
    ///
    /// Note: this method contains a hardcoded piece, so not dynamic.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_knight_captures();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_knight_captures(&mut self) {
        for file in self.files.iter() {
            for rank in self.ranks.iter() {
                let piece = "N";

                // Nxa1, Nxh7, etc.
                self.move_list
                    .push(format!("{}{}{}{}", piece, self.capture, file, rank,));

                // Nxa1+, Nxh7+, etc.
                self.move_list.push(format!(
                    "{}{}{}{}{}",
                    piece, self.capture, file, rank, self.check,
                ));

                for mate in self.check_mate.iter() {
                    // Nxa1++, Nxh7#, etc.
                    self.move_list
                        .push(format!("{}{}{}{}{}", piece, self.capture, file, rank, mate,));
                }
            }
        }
    }

    /// Generate all castling moves (e.g., 0-0, 0-0-0,
    /// 0-0+, 0-0-0#, etc.).
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_castle_moves();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_castle_moves(&mut self) {
        for castle in self.castle.iter() {
            // 0-0, 0-0-0
            self.move_list.push(castle.to_string());

            // 0-0+, 0-0-0+
            self.move_list.push(format!("{}{}", castle, self.check,));

            for mate in self.check_mate.iter() {
                // 0-0++, 0-0#, 0-0-0++, 0-0-0#
                self.move_list.push(format!("{}{}", castle, mate,));
            }
        }
    }

    /// Generate all square to square moves
    /// (e.g., a1-a5, b4-a4, ..., h2-h4).
    ///
    /// TODO: this method produces duplicates, fix at some point.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_square_to_square_moves();
    /// g.move_list.sort(); // dedup will not remove dupes unless sorted
    /// g.move_list.dedup(); // assert! will panic! unless deduped
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_square_to_square_moves(&mut self) {
        for (file_a_idx, file_a) in self.files.iter().enumerate() {
            for (rank_a_idx, rank_a) in self.ranks.iter().enumerate() {
                for (file_b_idx, file_b) in self.files.iter().enumerate() {
                    for (rank_b_idx, rank_b) in self.ranks.iter().enumerate() {
                        // The square-to-square move must first be valid.
                        // E.g., not a1-g3.
                        if MoveGenerator::move_is_valid(
                            file_a_idx, rank_a_idx, file_b_idx, rank_b_idx,
                        ) {
                            // We check if the square-to-square move is
                            // a valid pawn promotion.
                            if MoveGenerator::move_is_valid_promotion(
                                file_a_idx, rank_a_idx, file_b_idx, rank_b_idx,
                            ) {
                                for piece in self.promotion_pieces.iter() {
                                    for promotion in self.promotion_notation.iter() {
                                        // a2-a1, a2-b1, a7-a8, a7-b8, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}",
                                            file_a, rank_a, self.move_notation, file_b, rank_b,
                                        ));

                                        // a2-a1Q, a2-a1=R, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}{}{}",
                                            file_a,
                                            rank_a,
                                            self.move_notation,
                                            file_b,
                                            rank_b,
                                            promotion,
                                            piece
                                        ));

                                        // a2-a1+, a2-a1+, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}{}",
                                            file_a,
                                            rank_a,
                                            self.move_notation,
                                            file_b,
                                            rank_b,
                                            self.check,
                                        ));

                                        // a2-a1Q+, a2-a1=R+, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}{}{}{}",
                                            file_a,
                                            rank_a,
                                            self.move_notation,
                                            file_b,
                                            rank_b,
                                            promotion,
                                            piece,
                                            self.check,
                                        ));

                                        for mate in self.check_mate.iter() {
                                            // a2-a1++, a2-b1#, etc.
                                            self.move_list.push(format!(
                                                "{}{}{}{}{}{}",
                                                file_a,
                                                rank_a,
                                                self.move_notation,
                                                file_b,
                                                rank_b,
                                                mate,
                                            ));

                                            // a2-a1Q++, a2-a1=R#, etc.
                                            self.move_list.push(format!(
                                                "{}{}{}{}{}{}{}{}",
                                                file_a,
                                                rank_a,
                                                self.move_notation,
                                                file_b,
                                                rank_b,
                                                promotion,
                                                piece,
                                                mate,
                                            ));
                                        }
                                    }
                                }
                            }
                            // Process all other moves.
                            else {
                                // a1-b1, a1-c1, ..., g8-h8, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}",
                                    file_a, rank_a, self.move_notation, file_b, rank_b,
                                ));

                                // a1-b1+, a1-c1+, ..., h7-h8+, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}{}",
                                    file_a, rank_a, self.move_notation, file_b, rank_b, self.check,
                                ));

                                for mate in self.check_mate.iter() {
                                    // a1-b1++, a1-c1#, ..., g7-h8++, g7-h8#
                                    self.move_list.push(format!(
                                        "{}{}{}{}{}{}",
                                        file_a, rank_a, self.move_notation, file_b, rank_b, mate,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Generate all square to square captures
    /// (e.g., a1xa5, b4xa4, ..., h2xh4).
    ///
    /// TODO: this method produces duplicates, fix at some point.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let mut g = MoveGenerator::new();
    /// g.generate_square_to_square_captures();
    /// g.move_list.sort(); // dedup will not remove dupes unless sorted
    /// g.move_list.dedup(); // assert! will panic! unless deduped
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_square_to_square_captures(&mut self) {
        for (file_a_idx, file_a) in self.files.iter().enumerate() {
            for (rank_a_idx, rank_a) in self.ranks.iter().enumerate() {
                for (file_b_idx, file_b) in self.files.iter().enumerate() {
                    for (rank_b_idx, rank_b) in self.ranks.iter().enumerate() {
                        // The square-to-square capture must first be valid.
                        // E.g., not a1xg3.
                        if MoveGenerator::move_is_valid(
                            file_a_idx, rank_a_idx, file_b_idx, rank_b_idx,
                        ) {
                            // We check if the square-to-square capture is
                            // a valid pawn promotion.
                            if MoveGenerator::move_is_valid_promotion(
                                file_a_idx, rank_a_idx, file_b_idx, rank_b_idx,
                            ) {
                                for piece in self.promotion_pieces.iter() {
                                    for promotion in self.promotion_notation.iter() {
                                        // a2xa1, a2xa1, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}",
                                            file_a, rank_a, self.capture, file_b, rank_b,
                                        ));

                                        // a2xa1Q, a2xa1=R, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}{}{}",
                                            file_a,
                                            rank_a,
                                            self.capture,
                                            file_b,
                                            rank_b,
                                            promotion,
                                            piece
                                        ));

                                        // a2xa1+, a2xa1+, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}{}{}{}",
                                            file_a,
                                            rank_a,
                                            self.capture,
                                            file_b,
                                            rank_b,
                                            promotion,
                                            piece,
                                            self.check,
                                        ));

                                        // a2xa1Q+, a2xa1=R+, etc.
                                        self.move_list.push(format!(
                                            "{}{}{}{}{}{}",
                                            file_a,
                                            rank_a,
                                            self.capture,
                                            file_b,
                                            rank_b,
                                            self.check,
                                        ));

                                        for mate in self.check_mate.iter() {
                                            // a2xa1++, a2xa1#, etc.
                                            self.move_list.push(format!(
                                                "{}{}{}{}{}{}{}{}",
                                                file_a,
                                                rank_a,
                                                self.capture,
                                                file_b,
                                                rank_b,
                                                promotion,
                                                piece,
                                                mate,
                                            ));

                                            // a2xa1Q++, a2xa1=R#, etc.
                                            self.move_list.push(format!(
                                                "{}{}{}{}{}{}",
                                                file_a, rank_a, self.capture, file_b, rank_b, mate,
                                            ));
                                        }
                                    }
                                }
                            }
                            // Process all other moves.
                            else {
                                // a1xb1, a1xc1, ..., g8xh8, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}",
                                    file_a, rank_a, self.capture, file_b, rank_b,
                                ));

                                // a1xb1+, a1xc1+, ..., h7xh8+, etc.
                                self.move_list.push(format!(
                                    "{}{}{}{}{}{}",
                                    file_a, rank_a, self.capture, file_b, rank_b, self.check,
                                ));

                                for mate in self.check_mate.iter() {
                                    // a1xb1++, a1xc1#, ..., g7xh8++, g7xh8#
                                    self.move_list.push(format!(
                                        "{}{}{}{}{}{}",
                                        file_a, rank_a, self.capture, file_b, rank_b, mate,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// The main driver method to produce all possible chess notations.
    /// If you want to generate a specific set of move lists, use the other
    /// `generate_{pawn,king,queen,rook,knight,square_to_square}_{moves,captures}()`
    /// methods after initializing `let g = MoveGenerator::new();`.
    ///
    /// Example:
    ///
    /// ```
    /// use chui::MoveGenerator;
    ///
    /// let g = MoveGenerator::generate_move_list();
    /// let (answer, _reason) = g.validate_moves();
    /// assert!(answer);
    ///
    /// println!("{:?}", g.move_list);
    /// ```
    pub fn generate_move_list() -> MoveGenerator<'a> {
        let mut g = MoveGenerator::new();

        g.generate_pawn_moves();
        g.generate_pawn_captures();
        g.generate_king_moves();
        g.generate_king_captures();
        g.generate_queen_moves();
        g.generate_queen_captures();
        g.generate_rook_moves();
        g.generate_rook_captures();
        g.generate_bishop_moves();
        g.generate_bishop_captures();
        g.generate_knight_moves();
        g.generate_knight_captures();
        g.generate_castle_moves();
        g.generate_square_to_square_moves();
        g.generate_square_to_square_captures();
        g.move_list.sort(); // sort for quicker lookups
        g.move_list.dedup(); // have to dedup() because of
                             // .generate_square_to_square_move() and
                             // .generate_square_to_square_captures().
                             // dedup() also requires sort() to be
                             // performed before dedup().
                             // TODO: fix.

        g
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup<'a>() -> MoveGenerator<'a> {
        MoveGenerator::generate_move_list()
    }

    #[test]
    fn moves_validate() {
        let g = setup();
        let (answer, reason) = g.validate_moves();
        if !answer {
            panic!("Error: {}", reason);
        }
    }

    #[test]
    fn check_list() {
        let g = setup();

        let check_list = vec![
            // pawn moves
            "g8=N++".to_string(),
            "a1Q".to_string(),
            "g1/R".to_string(),
            "b7+".to_string(),
            "b2++".to_string(),
            "c4#".to_string(),
            "a1=Q+".to_string(),
            "d8Q++".to_string(),
            "e8/Q#".to_string(),
            "b7".to_string(),
            "e1B".to_string(),
            // pawn captures
            "hxg8=N++".to_string(),
            "bxa1Q".to_string(),
            "fxg1/R".to_string(),
            "cxb7+".to_string(),
            "bxa2++".to_string(),
            "cxd4#".to_string(),
            "axb1=Q+".to_string(),
            "dxc8Q++".to_string(),
            "exd8/Q#".to_string(),
            "bxa7".to_string(),
            "dxe1B".to_string(),
            // king moves
            "Ka6".to_string(),
            "Kg7#".to_string(),
            "Kd5+".to_string(),
            "Ka6++".to_string(),
            // king captures
            "Kxc6#".to_string(),
            "Kxg7".to_string(),
            "Kxa2+".to_string(),
            "Kxe5++".to_string(),
            // queen moves
            "Qa6".to_string(),
            "Qb2+".to_string(),
            "Qg7#".to_string(),
            "Qd5++".to_string(),
            // queen captures
            "Qxc6".to_string(),
            "Qxd4+".to_string(),
            "Qxg7#".to_string(),
            "Qxe5++".to_string(),
            // rook moves
            "Ra6".to_string(),
            "Rh3+".to_string(),
            "Rg7#".to_string(),
            "Rd5++".to_string(),
            // rook captures
            "Rxc6".to_string(),
            "Rxb1+".to_string(),
            "Rxg7#".to_string(),
            "Rxe5++".to_string(),
            // bishop moves
            "Ba6".to_string(),
            "Bg3+".to_string(),
            "Bg7#".to_string(),
            "Bd5++".to_string(),
            // bishop captures
            "Bxc6".to_string(),
            "Bxe1+".to_string(),
            "Bxg7#".to_string(),
            "Bxe5++".to_string(),
            // knight moves
            "Na6".to_string(),
            "Nc2+".to_string(),
            "Ng7#".to_string(),
            "Nd5++".to_string(),
            // knight captures
            "Nxc6".to_string(),
            "Nxh8+".to_string(),
            "Nxg7#".to_string(),
            "Nxe5++".to_string(),
            // castle moves
            "0-0".to_string(),
            "0-0-0".to_string(),
            "0-0+".to_string(),
            "0-0-0+".to_string(),
            "0-0++".to_string(),
            "0-0-0++".to_string(),
            "0-0#".to_string(),
            "0-0-0#".to_string(),
            // square-to-square moves
            "a1-a2".to_string(),
            "a1-a2+".to_string(),
            "a1-a2++".to_string(),
            "a1-a2#".to_string(),
            "a7-a8".to_string(),
            "a7-a8+".to_string(),
            "a7-a8++".to_string(),
            "a7-a8#".to_string(),
            "g7-g8".to_string(),
            "g7-g8Q".to_string(),
            "g7-g8".to_string(),
            "g7-g8".to_string(),
            "g7-g8+".to_string(),
            "g7-g8++".to_string(),
            "g7-g8#".to_string(),
            "f7-g8".to_string(),
            "f7-g8+".to_string(),
            "f7-g8++".to_string(),
            "f7-g8#".to_string(),
            // square-to-square captures
            "a1xa2".to_string(),
            "a1xa2+".to_string(),
            "a1xa2++".to_string(),
            "a1xa2#".to_string(),
            "a7xa8".to_string(),
            "a7xa8+".to_string(),
            "a7xa8++".to_string(),
            "a7xa8#".to_string(),
            "g7xg8".to_string(),
            "g7xg8+".to_string(),
            "g7xg8++".to_string(),
            "g7xg8#".to_string(),
            "f7xg8".to_string(),
            "f7xg8+".to_string(),
            "f7xg8++".to_string(),
            "f7xg8#".to_string(),
        ];

        for check in check_list.iter() {
            if !g.move_list.contains(check) {
                panic!("doesn't contain {}", check);
            }
        }
    }

    //
    // Test MoveGenerator::move_is_valid()
    //

    #[test]
    pub fn move_invalid_for_same_move() {
        let file_a: usize = 0;
        let rank_a: usize = 0; // a1
        let file_b: usize = 0;
        let rank_b: usize = 0; // a1

        assert!(!MoveGenerator::move_is_valid(
            file_a, rank_a, file_b, rank_b
        ));
    }

    #[test]
    pub fn move_valid_for_same_file() {
        {
            let file_a: usize = 2;
            let rank_a: usize = 0; // c1
            let file_b: usize = 2;
            let rank_b: usize = 1; // c2

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }

        {
            let file_a: usize = 0;
            let rank_a: usize = 6; // a7
            let file_b: usize = 0;
            let rank_b: usize = 7; // a8

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
    }

    #[test]
    pub fn move_valid_for_same_rank() {
        {
            let file_a: usize = 1;
            let rank_a: usize = 0; // b1
            let file_b: usize = 2;
            let rank_b: usize = 0; // c1

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
        {
            let file_a: usize = 1;
            let rank_a: usize = 5; // b6
            let file_b: usize = 2;
            let rank_b: usize = 5; // c6

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
    }

    #[test]
    pub fn move_valid_for_same_diagonal() {
        {
            let file_a: usize = 0;
            let rank_a: usize = 0; // a1
            let file_b: usize = 7;
            let rank_b: usize = 7; // h8

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
        {
            let file_a: usize = 1;
            let rank_a: usize = 4; // b5
            let file_b: usize = 0;
            let rank_b: usize = 5; // a6

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
        {
            let file_a: usize = 1;
            let rank_a: usize = 4; // b5
            let file_b: usize = 2;
            let rank_b: usize = 6; // a6

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
    }

    #[test]
    pub fn move_valid_for_knight_move() {
        {
            let file_a: usize = 0;
            let rank_a: usize = 0; // a1
            let file_b: usize = 1;
            let rank_b: usize = 2; // b3

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
        {
            let file_a: usize = 7;
            let rank_a: usize = 4; // h5
            let file_b: usize = 5;
            let rank_b: usize = 5; // f6

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
        {
            let file_a: usize = 4;
            let rank_a: usize = 3; // e4
            let file_b: usize = 3;
            let rank_b: usize = 1; // d2

            assert!(MoveGenerator::move_is_valid(file_a, rank_a, file_b, rank_b));
        }
    }

    #[test]
    pub fn move_is_invalid() {
        {
            let file_a: usize = 0;
            let rank_a: usize = 0; // a1
            let file_b: usize = 3;
            let rank_b: usize = 6; // d7

            assert!(!MoveGenerator::move_is_valid(
                file_a, rank_a, file_b, rank_b
            ));
        }
        {
            let file_a: usize = 7;
            let rank_a: usize = 7; // h8
            let file_b: usize = 0;
            let rank_b: usize = 3; // a4

            assert!(!MoveGenerator::move_is_valid(
                file_a, rank_a, file_b, rank_b
            ));
        }
        {
            let file_a: usize = 0;
            let rank_a: usize = 7; // a8
            let file_b: usize = 7;
            let rank_b: usize = 3; // h4

            assert!(!MoveGenerator::move_is_valid(
                file_a, rank_a, file_b, rank_b
            ));
        }
    }

    //
    // Test MoveGenerator::move_is_valid_promotion()
    //

    #[test]
    pub fn move_is_valid_promotion() {
        {
            let file_a: usize = 0;
            let rank_a: usize = 6; // a7
            let file_b: usize = 0;
            let rank_b: usize = 7; // a8

            assert!(MoveGenerator::move_is_valid_promotion(
                file_a, rank_a, file_b, rank_b
            ));
        }
        {
            let file_a: usize = 3;
            let rank_a: usize = 6; // d7
            let file_b: usize = 2;
            let rank_b: usize = 7; // c8

            assert!(MoveGenerator::move_is_valid_promotion(
                file_a, rank_a, file_b, rank_b
            ));
        }
        {
            let file_a: usize = 0;
            let rank_a: usize = 1; // a2
            let file_b: usize = 0;
            let rank_b: usize = 0; // a1

            assert!(MoveGenerator::move_is_valid_promotion(
                file_a, rank_a, file_b, rank_b
            ));
        }
        {
            let file_a: usize = 3;
            let rank_a: usize = 1; // d2
            let file_b: usize = 2;
            let rank_b: usize = 0; // c1

            assert!(MoveGenerator::move_is_valid_promotion(
                file_a, rank_a, file_b, rank_b
            ));
        }
    }

    #[test]
    pub fn move_is_invalid_promotion_same_rank() {
        let file_a: usize = 0;
        let rank_a: usize = 6; // a7
        let file_b: usize = 1;
        let rank_b: usize = 6; // b7

        assert!(!MoveGenerator::move_is_valid_promotion(
            file_a, rank_a, file_b, rank_b
        ));
    }

    #[test]
    pub fn move_is_invalid_promotion_start_and_end_rank() {
        let file_a: usize = 6;
        let rank_a: usize = 6; // g7
        let file_b: usize = 6;
        let rank_b: usize = 5; // g5

        assert!(!MoveGenerator::move_is_valid_promotion(
            file_a, rank_a, file_b, rank_b
        ));
    }

    #[test]
    pub fn move_is_valid_promotion_start_and_end_file() {
        let file_a: usize = 6;
        let rank_a: usize = 6; // g7
        let file_b: usize = 5;
        let rank_b: usize = 7; // f8

        assert!(MoveGenerator::move_is_valid_promotion(
            file_a, rank_a, file_b, rank_b
        ));
    }

    #[test]
    pub fn move_is_invalid_promotion_start_and_end_file() {
        let file_a: usize = 6;
        let rank_a: usize = 6; // g7
        let file_b: usize = 2;
        let rank_b: usize = 7; // b8

        assert!(!MoveGenerator::move_is_valid_promotion(
            file_a, rank_a, file_b, rank_b
        ));
    }
}
