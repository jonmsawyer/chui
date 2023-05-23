//! Chui: Coordinate Trainer

use std::io::{self, Write};
use std::time::{Duration, SystemTime};

use rand::Rng;

/// An array of chessboard files as usize format 8 elements long.
const INT_FILES: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
/// An array of chessboard files as &str format 8 elements long, representing alpha.
const ALPHA_FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
/// An array of chessboard files as &str format 8 elements long, representing numeric.
const ALPHA_RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
/// Compile in the version of this crate.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default, Copy, Clone, PartialEq)]
enum CommandType {
    /// The default state of the application as a type of command.
    #[default]
    Help,
    /// Train using numeric coordinates.
    Numeric,
    /// Train using alpha coordinates.
    Alpha,
    /// Train both using alpha and numeric coordinates.
    Both,
    /// Quit the training session or quit the application.
    Quit,
    /// Process available input.
    Input,
}

#[derive(Debug)]
pub struct CoordinateTrainer {
    /// A vector of 2-tuples, containing Strings, representing (<expression>, <input>).
    /// This vector stores the correct answers in the order they were given.
    vec_correct: Vec<(String, String, Duration)>,
    /// A vector of 2-tuples, containing Strings, representing (<expression>, <input>).
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

impl Default for CoordinateTrainer {
    fn default() -> Self {
        CoordinateTrainer {
            vec_correct: Vec::<(String, String, Duration)>::new(),
            vec_incorrect: Vec::<(String, String, Duration)>::new(),
            command_type: CommandType::Help,
            input: String::new(),
            answer: 0,
            lhs: 0,
            rhs: 0,
            operation: false,
            saved_lhs: String::new(),
            saved_rhs: String::new(),
            saved_operation: String::new(),
            session_timer: SystemTime::now(),
            input_duration: Duration::ZERO,
        }
    }
}

impl CoordinateTrainer {
    /// Create a new CoordinateTrainer object.
    pub fn new() -> Self {
        CoordinateTrainer {
            ..Default::default()
        }
    }

    /// Run the Coordinate Trainer application. Runs inside of two main loops. The outer loop
    /// controls the outer game loop logic, the inner loop, .e.g, when `self.train()` is called,
    /// runs the game mode logic.
    pub fn run(&mut self) -> Result<(), String> {
        println!(r#"Chui: Coordinate Trainer v{}"#, VERSION);
        println!("");

        loop {
            self.get_input();
            self.process_command(true);
            match self.command_type {
                // Print help and continue.
                CommandType::Help => self.print_help(),
                // Train the user in the given session type.
                CommandType::Numeric | CommandType::Alpha | CommandType::Both => self.train(),
                // Could not process further input.
                CommandType::Input => {}
                // Quit the application. Don't print scores, don't collect $200.
                CommandType::Quit => {
                    break;
                }
            }
        }

        Ok(())
    }

    /// Run the main training simulator. It doesn't matter if we're only training Numeric,
    /// Alpha, or Both, we use this loop for all 3 session types.
    ///
    /// Loop until the user quits the session. Until quit, this loop generates a problem,
    /// a user input is expected, and the answer checked for correctness. When the user
    /// quits, print a score sheet and go back to the main state of the application
    /// (`CommandType::Help`).
    pub fn train(&mut self) {
        self.session_timer = SystemTime::now();

        let training_header = match self.command_type {
            CommandType::Numeric => " = Train the Numeric Coordinate Type =".to_string(),
            CommandType::Alpha => " = Train the Alpha Coordinate Type =".to_string(),
            CommandType::Both => " = Train Both Coordinate Types =".to_string(),
            _ => " ??? Unknown Training Session ???".to_string(),
        };

        println!("{}", training_header);
        println!("");

        loop {
            self.generate_problem();
            self.get_input();
            // We do not store the command type when calling `self.process_command()` because
            // we don't want to change the state of the application right before checking
            // user input.
            match self.process_command(false) {
                CommandType::Numeric
                | CommandType::Alpha
                | CommandType::Both
                | CommandType::Input => self.solve_answer(),
                CommandType::Help => self.print_help(),
                CommandType::Quit => {
                    self.quit();
                    break;
                }
            }
        }
    }

    // Print the help message when the CommandType is Help.
    fn print_help(&self) {
        let numeric = if self.command_type == CommandType::Numeric {
            "*numeric".to_string()
        } else {
            "numeric".to_string()
        };

        let alpha = if self.command_type == CommandType::Alpha {
            "*alpha".to_string()
        } else {
            "alpha".to_string()
        };

        let both = if self.command_type == CommandType::Both {
            "*both".to_string()
        } else {
            "both".to_string()
        };

        let help = if self.command_type == CommandType::Help {
            "*?, or help".to_string()
        } else {
            "?, or help".to_string()
        };

        println!("Commands:");
        println!(
            "{:24}---------------------------------------------------",
            "----------------"
        );
        match self.command_type {
            CommandType::Numeric | CommandType::Alpha | CommandType::Both | CommandType::Input => {
                println!("{:24}Your answer to the generated problem.", "ANSWER");
            }
            _ => {}
        }
        println!(
            "{:24}Start a Numeric Coordinates training session.",
            numeric
        );
        println!("{:24}Start an Alpha Coordinates training session.", alpha);
        println!(
            "{:24}Start an Alphanumeric Coordinates training session.",
            both
        );
        println!("{:24}This help message.", help);
        println!("{:24}Quit this training session.", "q, quit, or exit");
        println!("");
    }

    /// Print the output correlating to a correct answer.
    fn print_correct(&self) {
        println!(
            " +++ Correct! ({} correct, {} incorrect)",
            self.vec_correct.len(),
            self.vec_incorrect.len()
        );
    }

    /// Print the output correlating to an incorrect answer.
    fn print_incorrect(&self) {
        println!(
            " --- Incorrect! Answer is '{}' or '{}'. ({} correct, {} incorrect)",
            INT_FILES[self.answer - 1],
            ALPHA_FILES[self.answer - 1],
            self.vec_correct.len(),
            self.vec_incorrect.len()
        )
    }

    /// Print the output correlating to a comprehensive score sheet.
    pub fn print_scores(&self) {
        let session = match self.command_type {
            CommandType::Numeric => "Numerical Coordinate Type".to_string(),
            CommandType::Alpha => "Alpha Coordinate Type".to_string(),
            CommandType::Both => "Alphanumerical Coordinate Types".to_string(),
            _ => "Unknown Coordinate Type".to_string(),
        };

        let header = format!("For the {}, your stats are:", session);
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

    /// Print the final scores and reset the Coordinate Trainer to the default run state.
    ///
    /// Note: This doesn't actually quit the application.
    ///
    /// TODO: Maybe it should?
    fn quit(&mut self) {
        self.print_scores();
        *self = CoordinateTrainer::new();
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
            CommandType::Input => {
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

    /// Given user input on `self.input`, process the command that follows that input. If
    /// `set` is true, set `self.command_type` as the processed command, otherwise just
    /// return that variant.
    ///
    /// Note: `CommandType` is Copy and Clone.
    fn process_command(&mut self, set: bool) -> CommandType {
        let command_type = if self.input.eq("?") || self.input.eq("help") {
            CommandType::Help
        } else if self.input.eq("numeric") {
            CommandType::Numeric
        } else if self.input.eq("alpha") {
            CommandType::Alpha
        } else if self.input.eq("both") {
            CommandType::Both
        } else if self.input.eq("q") || self.input.eq("quit") || self.input.eq("exit") {
            CommandType::Quit
        } else {
            CommandType::Input
        };

        if set {
            self.command_type = command_type;
        }

        command_type
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
            for (idx, alpha) in ALPHA_FILES.iter().enumerate() {
                if self.input.eq(alpha) {
                    if self.answer == INT_FILES[idx] {
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
            self.print_incorrect();
        }

        println!("");

        self.clear_saved_expression();
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
            CommandType::Alpha => ALPHA_FILES[self.lhs - 1].to_string(),
            CommandType::Both => {
                if rand::random() {
                    ALPHA_FILES[self.lhs - 1].to_string()
                } else {
                    ALPHA_RANKS[self.lhs - 1].to_string()
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
            CommandType::Alpha => ALPHA_FILES[self.rhs - 1].to_string(),
            CommandType::Both => {
                if rand::random() {
                    ALPHA_FILES[self.rhs - 1].to_string()
                } else {
                    ALPHA_RANKS[self.rhs - 1].to_string()
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
