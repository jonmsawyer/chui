//! Chui: Coordinate Trainer

use std::io::{self, Write};
use std::time::{Duration, SystemTime};

mod color;
use color::ColorTrainer;

mod coord;
use coord::CoordTrainer;

mod grid;
use grid::GridTrainer;

/// An array of chessboard files as usize format 8 elements long.
const INT_FILES: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
/// An array of chessboard files as &str format 8 elements long, representing alpha.
const ALPHA_FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
/// An array of chessboard files as &str format 8 elements long, representing numeric.
const ALPHA_RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
/// Compile in the version of this crate.
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum CommandType {
    /// The default state of the application as a type of command.
    #[default]
    Help,
    /// Train using numeric coordinates.
    Numeric,
    /// Train using alpha coordinates.
    Alpha,
    /// Train both using alpha and numeric coordinates.
    Both,
    /// Train the colors of the coordinate squares.
    Color,
    /// Train the user in grid coordinates.
    Grid,
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
    /// A timer to hold the elapsed time since the session started.
    session_timer: SystemTime,
    /// A Duration to hold the elapsed time since the problem was displayed and input was received.
    input_duration: Duration,
    /// Coordinate Trainer
    coord_trainer: CoordTrainer,
    /// Color Trainer
    color_trainer: ColorTrainer,
    /// Grid Trainer
    grid_trainer: GridTrainer,
}

impl Default for CoordinateTrainer {
    fn default() -> Self {
        CoordinateTrainer {
            vec_correct: Vec::<(String, String, Duration)>::new(),
            vec_incorrect: Vec::<(String, String, Duration)>::new(),
            command_type: CommandType::Help,
            input: String::new(),
            session_timer: SystemTime::now(),
            input_duration: Duration::ZERO,
            coord_trainer: CoordTrainer::new(),
            color_trainer: ColorTrainer::new(),
            grid_trainer: GridTrainer::new(),
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
                // Train the user in the given session type.
                CommandType::Numeric | CommandType::Alpha | CommandType::Both => {
                    self.coord_trainer.train(self.command_type)
                }
                // Train the user in the colors of squares.
                CommandType::Color => self.color_trainer.train(self.command_type),
                // Train the user in grid coordinates.
                CommandType::Grid => self.grid_trainer.train(self.command_type),
                // Print help and continue.
                CommandType::Help => self.print_help(),
                // Quit the application. Don't print scores, don't collect $200.
                CommandType::Quit => {
                    break;
                }
                // Could not process further input.
                _ => {}
            }
        }

        Ok(())
    }

    // Print the help message when the CommandType is Help.
    fn print_help(&self) {
        println!("+===============================+");
        println!("| Chui: Coordinate Trainer Help |");
        println!("+===============================+");
        println!("");

        let numeric = if self.command_type == CommandType::Numeric {
            "* numeric".to_string()
        } else {
            "  numeric".to_string()
        };

        let alpha = if self.command_type == CommandType::Alpha {
            "* alpha".to_string()
        } else {
            "  alpha".to_string()
        };

        let both = if self.command_type == CommandType::Both {
            "* both".to_string()
        } else {
            "  both".to_string()
        };

        let color = if self.command_type == CommandType::Color {
            "* color".to_string()
        } else {
            "  color".to_string()
        };

        let grid = if self.command_type == CommandType::Grid {
            "* grid".to_string()
        } else {
            "  grid".to_string()
        };

        let help = if self.command_type == CommandType::Help {
            "* ?, or help".to_string()
        } else {
            "  ?, or help".to_string()
        };

        println!("Commands:");
        println!("(* Selected)");
        println!(
            "{:24}---------------------------------------------------",
            "------------------"
        );
        match self.command_type {
            CommandType::Numeric
            | CommandType::Alpha
            | CommandType::Both
            | CommandType::Color
            | CommandType::Grid
            | CommandType::Input => {
                println!("{:24}Your answer to the generated problem.", "  ANSWER");
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
        println!(
            "{:24}Start a Coordinate Square Color training session.",
            color
        );
        println!("{:24}Start a Grid Coordinate training session.", grid);
        println!("{:24}This help message.", help);
        println!("{:24}Quit this training session.", "  q, quit, or exit");
        println!("");
    }

    /// Print the output correlating to a comprehensive score sheet.
    pub fn print_scores(&self) {
        let session = match self.command_type {
            CommandType::Numeric => "Numeric Coordinate Type".to_string(),
            CommandType::Alpha => "Alpha Coordinate Type".to_string(),
            CommandType::Both => "Alphanumeric Coordinate Types".to_string(),
            CommandType::Color => "Coordinate Square Color Type".to_string(),
            _ => "Unknown Type".to_string(),
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

    /// Get user input from `stdin` and store it in `self.input`.
    fn get_input(&mut self) {
        let input_timer = SystemTime::now();

        match self.command_type {
            _ => {
                println!(" === Please input command. '?' or 'help' for help. 'q' to quit.");
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
        let command_type = if self.input.eq("numeric") {
            CommandType::Numeric
        } else if self.input.eq("alpha") {
            CommandType::Alpha
        } else if self.input.eq("both") {
            CommandType::Both
        } else if self.input.eq("color") {
            CommandType::Color
        } else if self.input.eq("grid") {
            CommandType::Grid
        } else if self.input.eq("?") || self.input.eq("help") {
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
}
