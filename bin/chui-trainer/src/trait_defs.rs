//! Chui: Coordinate Trainer trait definitions.

pub use chui_macros::Trainer;

use super::CommandType;

/// Trainer trait.
pub trait Trainer {
    /// New
    fn new() -> Self;
    /// Train
    fn train(&mut self, command_type: CommandType);
    /// Get name verbose.
    fn get_name_verbose(&self) -> String;
    /// Get names verbose.
    fn get_names_verbose(&self, idx: usize, debug: bool) -> String;
    /// Get help message string.
    fn get_help_msg_string(&self) -> String;
    /// Print correct.
    fn print_correct(&self);
    /// Print incorrect.
    fn print_incorrect(&self);
    /// Quit
    fn quit(&mut self);
    /// Process command.
    fn process_command(&mut self, set: bool) -> CommandType;
    /// Generate problem.
    fn generate_problem(&mut self);
    /// Evaluate answer.
    fn evaluate_answer(&mut self);
    /// Solve answer.
    fn solve_answer(&mut self);
}
