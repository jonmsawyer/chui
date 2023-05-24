//! Chui: Coordinate Trainer

use std::io::{self, Write};
use std::time::{Duration, SystemTime};

use rand::Rng;

use super::CommandType;
use super::{ALPHA_FILES, ALPHA_RANKS};

#[derive(Debug)]
pub struct ColorTrainer {
    name_verbose: String,
    // names_verbose: Vec<String>,
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
        let name = "Coordinate Square Colors".to_string();

        ColorTrainer {
            name_verbose: name.clone(),
            // names_verbose: vec![name],
            vec_correct: Vec::<(String, String, Duration)>::new(),
            vec_incorrect: Vec::<(String, String, Duration)>::new(),
            command_type: CommandType::Help,
            input: String::new(),
            session_timer: SystemTime::now(),
            input_duration: Duration::ZERO,
            color_coordinate: (0, 0),
            answer_color: false,
        }
    }
}

impl ColorTrainer {
    /// Create a new `ColorTrainer` object.
    pub fn new() -> Self {
        ColorTrainer {
            ..Default::default()
        }
    }

    /// Train board coordinate colors.
    pub fn train(&mut self, command_type: CommandType) {
        self.command_type = command_type;
        self.session_timer = SystemTime::now();

        println!(" = Train Yourself in {} =", self.get_name_verbose());
        println!("");

        loop {
            self.generate_problem();
            self.get_input();
            // We do not store the command type when calling `self.process_command()` because
            // we don't want to change the state of the application right before checking
            // user input.
            match self.process_command(false) {
                CommandType::Input => self.solve_answer(),
                CommandType::Help => self.print_help(),
                CommandType::Quit => {
                    self.quit();
                    break;
                }
                _ => continue,
            }
        }
    }

    /// Get the verbose name of the session.
    fn get_name_verbose(&self) -> String {
        self.name_verbose.clone()
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
        )
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
        *self = ColorTrainer::new();
    }

    /// Get user input from `stdin` and store it in `self.input`.
    fn get_input(&mut self) {
        let input_timer = SystemTime::now();

        match self.command_type {
            CommandType::Color => {
                println!(
                    " === What is {}? '1' for Light, '2' for Dark",
                    self.get_algebraic_coordinate()
                )
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

        if self.answer_color == false
            && (self.input.eq("1") || self.input.to_ascii_lowercase().eq("light"))
        {
            self.add_correct();
        } else if self.answer_color == true
            && (self.input.eq("2") || self.input.to_ascii_lowercase().eq("dark"))
        {
            self.add_correct();
        } else {
            self.add_incorrect();
        }

        println!("");

        self.clear_saved_color_coordinate();
    }

    /// Return a String representing the Algebraic coordinate chosen.
    fn get_algebraic_coordinate(&self) -> String {
        if self.color_coordinate.0 != 0 && self.color_coordinate.1 != 0 {
            format!(
                "{}{}",
                ALPHA_FILES[self.color_coordinate.0 - 1],
                ALPHA_RANKS[self.color_coordinate.1 - 1]
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

        let element = (
            self.get_algebraic_coordinate(),
            color,
            self.input_duration.clone(),
        );
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

        let element = (
            self.get_algebraic_coordinate(),
            color,
            self.input_duration.clone(),
        );
        self.vec_incorrect.push(element);
        self.print_incorrect();
    }
}
