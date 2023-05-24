//! Chui: Coordinate Trainer trait definitions.

pub use chui_macros::Trainer;

use super::CommandType;

pub trait Trainer {
    fn new() -> Self;
    fn train(&mut self, command_type: CommandType);
    fn get_name_verbose(&self) -> String;
    fn get_names_verbose(&self, idx: usize, debug: bool) -> String;
    fn get_help_msg_string(&self) -> String;
    fn print_correct(&self);
    fn print_incorrect(&self);
    fn quit(&mut self);
    fn process_command(&mut self, set: bool) -> CommandType;
    fn generate_problem(&mut self);
    fn evaluate_answer(&mut self);
    fn solve_answer(&mut self);
}
