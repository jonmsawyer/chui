//! Chui: Coordinate Trainer

use std::fmt::Debug;
use std::io::{self, Write};
use std::time::{Duration, SystemTime};

use rand::Rng;

use super::{trait_defs::*, CommandType};
use super::{INT_FILES, STR_FILES, STR_RANKS};

#[derive(Debug, Trainer)]
#[trainer(base = true)]
pub struct AlphaNumericTrainer {
    /// Verbose name.
    name_verbose: String,
    /// Vec of Strings of internal names dependeing on context.
    names_verbose: Vec<String>,
    /// A vector of 3-tuples, containing Strings, representing (<expression>, <input>).
    /// This vector stores the correct answers in the order they were given.
    vec_correct: Vec<(String, String, Duration)>,
    /// A vector of 3-tuples, containing Strings, representing (<expression>, <input>).
    /// This vector stores the incorrect answers in the order they were given.
    vec_incorrect: Vec<(String, String, Duration)>,
    /// The state of the application as a type of command.
    command_type: CommandType,
    /// The user input string, when prompted.
    input: String,
    /// The answer to the generated problem.
    answer: usize,
    /// The left-hand-side of the generated problem (expression).
    lhs: usize,
    /// The right-hand-side of the generated problem (expression).
    rhs: usize,
    /// A random operation. False means subtract the `lhs` from the `rhs`. True means add the
    /// two sides together.
    operation: bool,
    /// A String representing the last rendering of the left-hand-side of the expression.
    saved_lhs: String,
    /// A String representing the last rendering of the right-hand-side of the expression.
    saved_rhs: String,
    /// A String representing the last rendering of the operation of the expression.
    saved_operation: String,
    /// A timer to hold the elapsed time since the session started.
    session_timer: SystemTime,
    /// A Duration to hold the elapsed time since the problem was displayed and input was received.
    input_duration: Duration,
}

impl Default for AlphaNumericTrainer {
    fn default() -> Self {
        AlphaNumericTrainer {
            name_verbose: "AlphaNumeric Coordinates".to_string(),
            names_verbose: vec![
                "Numeric Coordinates".to_string(),
                "Alpha Coordinates".to_string(),
                "AlphaNumeric Coordinates".to_string(),
            ],
            session_timer: SystemTime::now(),
            // Default
            vec_correct: Default::default(),
            vec_incorrect: Default::default(),
            command_type: Default::default(),
            input: Default::default(),
            answer: Default::default(),
            lhs: Default::default(),
            rhs: Default::default(),
            operation: Default::default(),
            saved_lhs: Default::default(),
            saved_rhs: Default::default(),
            saved_operation: Default::default(),
            input_duration: Default::default(),
        }
    }
}

impl AlphaNumericTrainer {
    /// Get the verbose name of the session based on the command type.
    pub fn get_name_verbose(&self) -> String {
        match self.command_type {
            CommandType::Numeric => self.names_verbose[0].clone(),
            CommandType::Alpha => self.names_verbose[1].clone(),
            CommandType::Both => self.names_verbose[2].clone(),
            _ => " ??? Unknown Training Session ???".to_string(),
        }
    }

    // Print the help message when the CommandType is Help.
    fn print_help(&self) {
        println!("+=======================================+");
        println!("| {} Trainer Help |", self.name_verbose);
        println!("+=======================================+");
        println!("");

        println!("Commands:");
        println!("(* Selected)");
        println!(
            "{:24}---------------------------------------------------",
            "------------------"
        );

        match self.command_type {
            CommandType::Numeric | CommandType::Alpha | CommandType::Both | CommandType::Input => {
                println!("{:24}Your answer to the generated problem.", "  ANSWER");
            }
            _ => {}
        }

        if self.command_type == CommandType::Numeric {
            println!(
                "{:24}You are in the {} training session.",
                "* numeric", self.names_verbose[0]
            );
        } else {
            println!(
                "{:24}To select this training session, first quit this one and then select it.",
                "  numeric",
            );
        };

        if self.command_type == CommandType::Alpha {
            println!(
                "{:24}You are in the {} training session.",
                "* alpha", self.names_verbose[1],
            );
        } else {
            println!(
                "{:24}To select this training session, first quit this one and then select it.",
                "  alpha",
            );
        };

        if self.command_type == CommandType::Both {
            println!(
                "{:24}You are in the {} training session.",
                "* both", self.names_verbose[2]
            );
        } else {
            println!(
                "{:24}To select this training session, first quit this one and then select it.",
                "  both",
            );
        };

        println!("{:24}This help message.", self.get_help_msg_string());
        println!(
            "{:24}Quit this training session (but not this application).",
            "  q, quit, or exit"
        );
        println!("");
    }

    /// Return a String representing the `self.get_help()` `?` and `help` text.
    fn get_help_msg_string(&self) -> String {
        if self.command_type == CommandType::Help {
            "* ?, or help".to_string()
        } else {
            "  ?, or help".to_string()
        }
    }

    /// Print the output correlating to an incorrect answer.
    fn print_incorrect(&self) {
        println!(
            " --- Incorrect! Answer is '{}' or '{}'. ({} correct, {} incorrect)",
            STR_FILES[self.answer - 1],
            STR_FILES[self.answer - 1],
            self.vec_correct.len(),
            self.vec_incorrect.len()
        )
    }

    /// Print the output correlating to a comprehensive score sheet.
    pub fn print_scores(&self) {
        let session = match self.command_type {
            CommandType::Numeric => self.names_verbose[0].clone(),
            CommandType::Alpha => self.names_verbose[1].clone(),
            CommandType::Both => self.names_verbose[2].clone(),
            _ => "Unknown".to_string(),
        };

        let header = format!("For this {} session, your stats are:", session);
        let header_len = header.len();

        println!("{}", header);
        println!("{:->header_len$}", "-");

        let mut avg_duration = 0_f32;

        println!("Number Correct: {}", self.vec_correct.len());
        for (idx, (expression, input, duration)) in self.vec_correct.iter().enumerate() {
            avg_duration += duration.as_secs_f32();
            println!(
                "{:>4}:  {}  {:<22}{:>16}",
                idx + 1,
                expression,
                format!("(with answer '{}')", input),
                format!("[{:>4.2} secs]", duration.as_secs_f32())
            );
        }
        println!(
            "{:>27}Avg Duration: [{:0.2} secs]",
            "",
            avg_duration / self.vec_correct.len() as f32
        );

        avg_duration = 0_f32;

        println!("Number Incorrect: {}", self.vec_incorrect.len());
        for (idx, (expression, input, duration)) in self.vec_incorrect.iter().enumerate() {
            avg_duration += duration.as_secs_f32();
            println!(
                "{:>4}:  {}  {:<22}{:>16}",
                idx + 1,
                expression,
                format!("(with answer '{}')", input),
                format!("[{:>4.2} secs]", duration.as_secs_f32())
            );
        }
        println!(
            "{:>27}Avg Duration: [{:0.2} secs]",
            "",
            avg_duration / self.vec_incorrect.len() as f32
        );

        if let Ok(elapsed) = self.session_timer.elapsed() {
            println!(
                "Elapsed Time for Training Session:{:>18}",
                format!("[{:0.2} secs]", elapsed.as_secs_f32())
            );
        } else {
            println!(
                "Could not determine the elapsed time since you started this training session."
            );
        }

        println!("");
    }

    /// Get user input from `stdin` and store it in `self.input`.
    fn get_input(&mut self) {
        let input_timer = SystemTime::now();

        match self.command_type {
            CommandType::Help => {
                println!(" === Please input command. '?' or 'help' for help. 'q' to quit.");
            }
            CommandType::Numeric | CommandType::Alpha | CommandType::Both => {
                println!(" === What is {}?", self.get_expression());
            }
            CommandType::Quit => {
                println!(" vvv Quitting training session...");
            }
            _ => {
                println!(" !!! Invalid command. '?' or 'help' for help. 'q' to quit.");
            }
        }

        // Do some fancy "prompt" work.
        print!(" >>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input result.");

        if let Ok(duration) = input_timer.elapsed() {
            self.input_duration = duration;
        } else {
            self.input_duration = Duration::ZERO;
        }

        self.input = input.trim().to_string();

        println!("");
    }

    /// Generate the problem. Don't let any answers fall outside of the range 1..=8 for any
    /// given operation.
    ///
    /// If the operation is to add, if their sums is greater than 8, then regenerate the
    /// expression unless this check passes.
    ///
    /// If the operation is to subtract, and their difference is less than 1, then regenerate
    /// the expression unless this check passes.
    ///
    /// Note: we want to orient `self.lhs` to be greater than or equal to `self.rhs`.
    fn generate_problem(&mut self) {
        self.clear_saved_expression();

        let mut rng = rand::thread_rng();

        loop {
            self.lhs = rng.gen_range(1..9);
            self.rhs = rng.gen_range(1..9);
            self.operation = rand::random();

            let (lesser, greater) = if self.lhs < self.rhs {
                (self.lhs, self.rhs)
            } else {
                (self.rhs, self.lhs)
            };

            if self.operation {
                // + operation
                if lesser + greater < 9 {
                    if rand::random() {
                        self.lhs = lesser;
                        self.rhs = greater;
                    } else {
                        self.rhs = lesser;
                        self.lhs = greater;
                    }
                    return;
                }

                continue;
            }

            // - operation
            self.lhs = greater;
            self.rhs = lesser;

            // The only way these two attributes would evaluate to 0 is if they were equal,
            // so this is invalid.
            if self.lhs == self.rhs {
                continue;
            }

            return;
        }
    }

    /// Evaluate the answer based on the operation. Note that by this point the `lhs` and `rhs`
    /// positional arguments are oriented correctly. We don't want to calculate negative numbers.
    ///
    /// Note: `self.lhs` and `self.rhs` have values between 1..=8 and `self.lhs` is guaranteed
    /// to be greater than or equal to `self.rhs` when the operation is subtraction.
    fn evaluate_answer(&mut self) {
        if self.operation {
            self.answer = self.lhs + self.rhs;
        } else {
            self.answer = self.lhs - self.rhs;
        }
    }

    /// Solve the answer based on the user's input. If user's input matches the internal
    /// representation of the answer (literally just `1..=8`), then they get a correct answer
    /// and their answer is saved. If not, an incorrect answer is recorded.
    fn solve_answer(&mut self) {
        self.evaluate_answer();

        let mut is_evaluated = false;

        if let Ok(answer) = self.input.parse::<usize>() {
            if self.answer == answer {
                self.add_correct();
            } else {
                self.add_incorrect();
            }

            is_evaluated = true;
        } else {
            for (idx, alpha) in STR_FILES.iter().enumerate() {
                if self.input.eq(alpha) {
                    if self.answer == INT_FILES[idx] as usize {
                        self.add_correct();
                    } else {
                        self.add_incorrect();
                    }

                    is_evaluated = true;
                }
            }
        }

        if !is_evaluated {
            self.add_incorrect();
        }

        println!("");
    }

    /// Return a String representing the left-hand-side of the equation.
    ///
    /// If the training session is in Numeric mode, return the raw value.
    ///
    /// If the training session is in Alpha mode, return the alpha representation
    /// of the expression.
    ///
    /// If the training session is in Alphanumeric mode, return a randomized
    /// numeric of alpha display of the expression.
    fn get_lhs(&self) -> String {
        match self.command_type {
            CommandType::Alpha => STR_FILES[self.lhs - 1].to_string(),
            CommandType::Both => {
                if rand::random() {
                    STR_FILES[self.lhs - 1].to_string()
                } else {
                    STR_RANKS[self.lhs - 1].to_string()
                }
            }
            _ => format!("{}", self.lhs),
        }
    }

    /// Return a String representing the right-hand-side of the equation.
    ///
    /// If the training session is in Numeric mode, return the raw value.
    ///
    /// If the training session is in Alpha mode, return the alpha representation
    /// of the expression.
    ///
    /// If the training session is in Alphanumeric mode, return a randomized
    /// numeric of alpha display of the expression.
    fn get_rhs(&self) -> String {
        match self.command_type {
            CommandType::Alpha => STR_FILES[self.rhs - 1].to_string(),
            CommandType::Both => {
                if rand::random() {
                    STR_FILES[self.rhs - 1].to_string()
                } else {
                    STR_RANKS[self.rhs - 1].to_string()
                }
            }
            _ => format!("{}", self.rhs),
        }
    }

    /// Return a String representing the operation of this expression. Either "+" or "-".
    fn get_operation(&self) -> String {
        if self.operation {
            "+".to_string()
        } else {
            "-".to_string()
        }
    }

    /// Return a String representing the formatted expression
    fn get_expression(&mut self) -> String {
        if self.saved_lhs.is_empty() || self.saved_operation.is_empty() || self.saved_rhs.is_empty()
        {
            self.saved_lhs = self.get_lhs();
            self.saved_operation = self.get_operation();
            self.saved_rhs = self.get_rhs();
        }

        format!(
            "{} {} {}",
            self.saved_lhs, self.saved_operation, self.saved_rhs
        )
    }

    /// Clear out the saved, rendered, expression.
    fn clear_saved_expression(&mut self) {
        self.saved_lhs.clear();
        self.saved_operation.clear();
        self.saved_rhs.clear();
    }

    /// Render the last saved expression and copy the user input for entry into the vector
    /// of correct answers.
    fn add_correct(&mut self) {
        let element = (
            self.get_expression(),
            self.input.clone(),
            self.input_duration.clone(),
        );
        self.vec_correct.push(element);
        self.print_correct();
    }

    /// Render the last saved expression and copy the user input for entry into the vector
    /// of incorrect answers.
    fn add_incorrect(&mut self) {
        let element = (
            self.get_expression(),
            self.input.clone(),
            self.input_duration.clone(),
        );
        self.vec_incorrect.push(element);
        self.print_incorrect();
    }
}
