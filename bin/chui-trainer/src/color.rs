//! Chui: Coordinate Trainer

use std::io::{self, Write};
use std::time::{Duration, SystemTime};

use rand::Rng;

use super::{trait_defs::Trainer, CommandType};

use chui_core::{STR_FILES, STR_RANKS};

#[derive(Debug, Trainer, Clone)]
#[trainer(base = true)]
/// `ColorTrainer` struct. Train square colors of board coordinates.
pub struct ColorTrainer {
    /// Verbose name of this trainer.
    name_verbose: String,
    /// A Vector of Strings representing the verbose names of sub-modules inside the trainer.
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
    /// A timer to hold the elapsed time since the session started.
    session_timer: SystemTime,
    /// A Duration to hold the elapsed time since the problem was displayed and input was received.
    input_duration: Duration,
    /// Color coordinate
    color_coordinate: (usize, usize),
    /// The answer to the color problem.
    answer_color: bool,
}

impl Default for ColorTrainer {
    fn default() -> Self {
        ColorTrainer {
            name_verbose: "Coordinate Square Colors".to_string(),
            session_timer: SystemTime::now(),
            // Default
            names_verbose: Default::default(),
            vec_correct: Default::default(),
            vec_incorrect: Default::default(),
            command_type: Default::default(),
            input: Default::default(),
            input_duration: Default::default(),
            color_coordinate: Default::default(),
            answer_color: Default::default(),
        }
    }
}

impl ColorTrainer {
    /// Print the help message when the [`CommandType`] is Help.
    fn print_help(&self) {
        println!("+=======================================+");
        println!("| {} Trainer Help |", self.name_verbose);
        println!("+=======================================+");
        println!();
        println!("Commands:");
        println!("(* Selected)");
        println!(
            "{:24}---------------------------------------------------",
            "------------------"
        );

        match self.command_type {
            CommandType::Color | CommandType::Input => {
                println!("{:24}Your answer to the generated problem.", "  ANSWER");
            }
            _ => {}
        }

        println!(
            "{:24}You are in the {} training session.",
            "* color", self.name_verbose
        );
        println!("{:24}This help message.", self.get_help_msg_string());
        println!(
            "{:24}Quit this training session (but not this application).",
            "  q, quit, or exit"
        );
        println!();
    }

    /// Print the output correlating to an incorrect answer.
    fn print_incorrect(&self) {
        let color = if self.answer_color {
            "Dark".to_string()
        } else {
            "Light".to_string()
        };

        println!(
            " --- Incorrect! Answer is '{}'. ({} correct, {} incorrect)",
            color,
            self.vec_correct.len(),
            self.vec_incorrect.len()
        );
    }

    /// Print the output correlating to a comprehensive score sheet.
    pub fn print_scores(&self) {
        let header = format!("For this {} session, your stats are:", self.name_verbose);
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

        self.session_timer.elapsed().map_or_else(
            |_| {
                println!(
                    "Could not determine the elapsed time since you started this training session."
                );
            },
            |elapsed| {
                println!(
                    "Elapsed Time for Training Session:{:>18}",
                    format!("[{:0.2} secs]", elapsed.as_secs_f32())
                );
            },
        );

        println!();
    }

    /// Get user input from `stdin` and store it in `self.input`.
    fn get_input(&mut self) {
        let input_timer = SystemTime::now();

        match self.command_type {
            CommandType::Color => {
                println!(
                    " === What is {}? '1' for Light, '2' for Dark",
                    self.get_algebraic_coordinate()
                );
            }
            CommandType::Help => {
                println!(" === Please input command. '?' or 'help' for help. 'q' to quit.");
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

        println!();
    }

    /// Generate the color coordinate.
    fn generate_problem(&mut self) {
        self.clear_saved_color_coordinate();

        let mut rng = rand::thread_rng();

        self.set_color_coordinate(rng.gen_range(1..9), rng.gen_range(1..9));
    }

    /// Set the board coordinate to the passed in rank and file (1-indexed).
    fn set_color_coordinate(&mut self, file: usize, rank: usize) {
        self.color_coordinate = (file, rank);
    }

    /// Evaluate the answer based on the operation.
    fn evaluate_answer(&mut self) {
        let (file, rank) = self.color_coordinate;

        if file % 2 == 0 {
            // Even file.
            if rank % 2 == 0 {
                // Even rank. Dark color.
                self.answer_color = true;
            } else {
                // Odd rank. Light color.
                self.answer_color = false;
            }
        } else {
            // Odd file.
            if rank % 2 == 0 {
                // Even rank. Light color.
                self.answer_color = false;
            } else {
                // Odd rank. Dark color.
                self.answer_color = true;
            }
        }
    }

    /// Solve the answer based on the user's input.
    fn solve_answer(&mut self) {
        self.evaluate_answer();

        if (!self.answer_color
            && (self.input.eq("1") || self.input.to_ascii_lowercase().eq("light")))
            || (self.answer_color
                && (self.input.eq("2") || self.input.to_ascii_lowercase().eq("dark")))
        {
            self.add_correct();
        } else {
            self.add_incorrect();
        }

        println!();

        self.clear_saved_color_coordinate();
    }

    /// Return a String representing the Algebraic coordinate chosen.
    fn get_algebraic_coordinate(&self) -> String {
        if self.color_coordinate.0 != 0 && self.color_coordinate.1 != 0 {
            format!(
                "{}{}",
                STR_FILES[self.color_coordinate.0 - 1],
                STR_RANKS[self.color_coordinate.1 - 1]
            )
        } else {
            "(0, 0)".to_string()
        }
    }

    /// Clear out the saved, rendered, expression.
    fn clear_saved_color_coordinate(&mut self) {
        self.color_coordinate = (0, 0);
        self.answer_color = false;
    }

    /// Render the last saved expression and copy the user input for entry into the vector
    /// of correct answers.
    fn add_correct(&mut self) {
        let color = if self.input.eq("1") || self.input.to_ascii_lowercase().eq("light") {
            "Light".to_string()
        } else if self.input.eq("2") || self.input.to_ascii_lowercase().eq("dark") {
            "Dark".to_string()
        } else {
            self.input.clone()
        };

        let element = (self.get_algebraic_coordinate(), color, self.input_duration);
        self.vec_correct.push(element);
        self.print_correct();
    }

    /// Render the last saved expression and copy the user input for entry into the vector
    /// of incorrect answers.
    fn add_incorrect(&mut self) {
        let color = if self.input.eq("1") || self.input.to_ascii_lowercase().eq("light") {
            "Light".to_string()
        } else if self.input.eq("2") || self.input.to_ascii_lowercase().eq("dark") {
            "Dark".to_string()
        } else {
            self.input.clone()
        };

        let element = (self.get_algebraic_coordinate(), color, self.input_duration);
        self.vec_incorrect.push(element);
        self.print_incorrect();
    }
}
