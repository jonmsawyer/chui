//! Chui: Coordinate Trainer

use std::io::{self, Write};
use std::time::{Duration, SystemTime};

use chui_core::{INT_FILES, STR_FILES, STR_RANKS, VERSION};

pub mod trait_defs;
use trait_defs::Trainer;

mod color;
use color::ColorTrainer;

mod coord;
use coord::AlphaNumericTrainer;

mod grid;
use grid::GridTrainer;

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

#[derive(Debug, Trainer)]
#[trainer(base = true)]
pub struct CoordinateTrainer {
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
    /// A timer to hold the elapsed time since the session started.
    session_timer: SystemTime,
    /// A Duration to hold the elapsed time since the problem was displayed and input was received.
    input_duration: Duration,
    /// Coordinate Trainer
    coord_trainer: AlphaNumericTrainer,
    /// Color Trainer
    color_trainer: ColorTrainer,
    /// Grid Trainer
    grid_trainer: GridTrainer,
}

impl Default for CoordinateTrainer {
    fn default() -> Self {
        CoordinateTrainer {
            name_verbose: "CoordinateTrainer".to_string(),
            names_verbose: Default::default(),
            session_timer: SystemTime::now(),
            coord_trainer: AlphaNumericTrainer::new(),
            color_trainer: ColorTrainer::new(),
            grid_trainer: GridTrainer::new(),

            vec_correct: Default::default(),
            vec_incorrect: Default::default(),
            command_type: Default::default(),
            input: Default::default(),
            input_duration: Default::default(),
        }
    }
}

impl CoordinateTrainer {
    /// Run the Coordinate Trainer application. Runs inside of two main loops. The outer loop
    /// controls the outer game loop logic, the inner loop, .e.g, when `self.train()` is called,
    /// runs the game mode logic.
    pub fn run(&mut self) -> Result<(), String> {
        println!(r#" = Chui: Coordinate Trainer v{} ="#, VERSION);
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

    // Print the help message when the `CommandType` is Help.
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
            "{:24}Start a {} training session.",
            numeric,
            self.coord_trainer.get_names_verbose(0, false)
        );
        println!(
            "{:24}Start an {} training session.",
            alpha,
            self.coord_trainer.get_names_verbose(1, false)
        );
        println!(
            "{:24}Start an {} training session.",
            both,
            self.coord_trainer.get_names_verbose(2, false)
        );
        println!(
            "{:24}Start a {} training session.",
            color,
            self.color_trainer.get_name_verbose()
        );
        println!(
            "{:24}Start a {} training session.",
            grid,
            self.grid_trainer.get_name_verbose()
        );
        println!("{:24}This help message.", help);
        println!("{:24}Exit out of this application.", "  q, quit, or exit");
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
