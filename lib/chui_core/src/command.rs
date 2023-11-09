//! Provides a struct for `Command` instances. Combined with
//! `CommandKind` and `CommandContext`, the command represents the
//! available commands per context, all built into `Command.commands`.

use std::collections::HashMap;

use crate::Game;

/// The kind of command.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum CommandKind {
    /// A move. Passed into the current `Parser` for the `Engine`.
    Move,

    /// Switch the current move parser.
    SwitchParser,

    /// Display the board for the `Color` that is to move.
    DisplayToMove,

    /// Display the board for `White`.
    DisplayForWhite,

    /// Display the board for `Black`.
    DisplayForBlack,

    /// Display help.
    Help,

    /// Back out of the current command scope, if any.
    Back,

    /// Quit the application.
    Quit,

    /// Switch to Algebraic Parser.
    SwitchToAlgebraicParser,

    /// Switch to Concise Reversible Parser.
    SwitchToConciseReversibleParser,

    /// Switch to Coordinate Parser.
    SwitchToCoordinateParser,

    /// Switch to Descriptive Parser.
    SwitchToDescriptiveParser,

    /// Switch to ICCF Parser.
    SwitchToICCFParser,

    /// Switch to Long Algebraic Parser.
    SwitchToLongAlgebraicParser,

    /// Switch to Reversible Algebraic Parser.
    SwitchToReversibleAlgebraicParser,

    /// Switch to Smith Parser.
    SwitchToSmithParser,

    /// Display the FEN layout of the board.
    DisplayFEN,

    /// Display the list of moves.
    DisplayMoveList,

    /// Display captures.
    DisplayCaptures,

    /// White resigns.
    WhiteResigns,

    /// Black resigns.
    BlackResigns,

    /// Display the board for `White` after each move.
    DisplayForWhiteEachMove,

    /// Display the board for `Black` after each move.
    DisplayForBlackEachMove,
}

/// The context of the command.
#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CommandContext {
    /// Main context. Top level command processor.
    #[default]
    Main,

    /// Switch parser command. Apply the command within the context
    /// of switching the parser.
    SwitchParser,
}

/// The command part. Used when building a vec of command parts
/// in the commands list.
#[derive(Debug, Clone)]
pub struct CommandPart {
    /// A vec of strings containing the commands to apply.
    commands: Vec<String>,

    /// The description.
    description: String,

    /// The kind of command to execute.
    command_kind: CommandKind,
}

/// Main command struct. Contains a hashmap of commands based on
/// context.
#[derive(Debug)]
pub struct Command {
    /// The available commands per context.
    commands: HashMap<CommandContext, Vec<CommandPart>>,
}

impl Command {
    /// New command.
    pub fn new(game: &Game) -> Command {
        Command {
            commands: Command::build_commands(game),
        }
    }

    /// Process the input command based on the given context.
    pub fn process_command(&self, context: CommandContext, command: String) -> Option<CommandKind> {
        // Get the list of commands for the context.
        if let Some(list) = self.commands.get(&context) {
            // For each command part, compare the input command
            // to its input list. If the input command matches,
            // return Some(CommandKind).
            for command_part in list.iter() {
                for cmd in command_part.commands.iter() {
                    if cmd.eq(&command) {
                        return Some(command_part.command_kind);
                    }
                }
            }
        }

        None
    }

    /// Build a new commands list. This is useful when the context
    /// of the `Engine` has changed, such as switching to a new
    /// parser engine.
    pub fn rebuild_commands(&mut self, game: &Game) {
        self.commands = Command::build_commands(game);
    }

    /// Print the help message for a given context. Goes through
    /// the command list and determins the length of each command
    /// list and description, then pretty prints the list of
    /// commands with their description.
    ///
    /// Example output:
    ///
    /// <pre>
    /// Input                   Description
    /// -------                 ------------------------------------------------
    /// MOVE                    E.g., e4, Bxc6, Qb4, exf8=Q++ (Algebraic Parser)
    /// sw                      Switch the current parser engine
    /// dw                      Display board for White
    /// db                      Display board for Black
    /// h, help                 Display this help message
    /// q, quit                 Quit the application
    /// </pre>
    pub fn display_help(&self, context: CommandContext) {
        if let Some(list) = self.commands.get(&context) {
            // Calculate the longest input string.
            let mut longest_input = 0;

            // Calculate the longest description string.
            let mut longest_description = 0;

            // The separation distance between lhs and rhs.
            let sep = 32;

            // Calculate the longest input command and description
            // for each command.
            for command_part in list.iter() {
                let input = command_part.commands.join(", ");
                if input.len() > longest_input {
                    longest_input = input.len();
                }
                let desc = &command_part.description;
                if desc.len() > longest_description {
                    longest_description = desc.len();
                }
            }

            println!();

            // Prints something similar to:
            // Input                   Description
            println!("{0:<1$}Description", "Input", sep);

            // Prints something similar to:
            // -------                 -----------------------------
            println!(
                "{0:-<2$}{1:4$}{0:-<3$}",
                "-",
                " ",
                longest_input,
                longest_description,
                sep - longest_input,
            );

            // Print the input command with its description, aligned
            // with the headers.
            //
            // Prints something similar to:
            // MOVE                    E.g., Qb4 (Algebraic Parser)
            for command_part in list.iter() {
                println!(
                    "{0:<2$}{1}",
                    command_part.commands.join(", "),
                    command_part.description,
                    sep,
                );
            }
        }
    }

    /// Build (or rebuild) the commands list based on the
    /// (changed) context of `Engine`.
    pub fn build_commands(game: &Game) -> HashMap<CommandContext, Vec<CommandPart>> {
        let mut map = HashMap::new();

        // Insert commands for context `Main`.
        map.insert(
            CommandContext::Main,
            vec![
                CommandPart {
                    commands: vec!["MOVE".to_string()],
                    description: format!("E.g., {}", game.parser.eg()),
                    command_kind: CommandKind::Move,
                },
                CommandPart {
                    commands: vec!["wr".to_string(), "white resigns".to_string()],
                    description: "White resigns".to_string(),
                    command_kind: CommandKind::WhiteResigns,
                },
                CommandPart {
                    commands: vec!["br".to_string(), "black resigns".to_string()],
                    description: "Black resigns".to_string(),
                    command_kind: CommandKind::BlackResigns,
                },
                CommandPart {
                    commands: vec!["sw".to_string(), "switch parser".to_string()],
                    description: "Switch the current parser engine".to_string(),
                    command_kind: CommandKind::SwitchParser,
                },
                CommandPart {
                    commands: vec![
                        "d".to_string(),
                        "dtm".to_string(),
                        "display to move".to_string(),
                    ],
                    description: "Display board for the color that is to move".to_string(),
                    command_kind: CommandKind::DisplayToMove,
                },
                CommandPart {
                    commands: vec!["dw".to_string(), "display white".to_string()],
                    description: "Display board for White".to_string(),
                    command_kind: CommandKind::DisplayForWhite,
                },
                CommandPart {
                    commands: vec!["db".to_string(), "display black".to_string()],
                    description: "Display board for Black".to_string(),
                    command_kind: CommandKind::DisplayForBlack,
                },
                CommandPart {
                    commands: vec!["dfw".to_string(), "display for white".to_string()],
                    description: "Display board for White after each move".to_string(),
                    command_kind: CommandKind::DisplayForWhiteEachMove,
                },
                CommandPart {
                    commands: vec!["dfb".to_string(), "display for black".to_string()],
                    description: "Display board for Black after each move".to_string(),
                    command_kind: CommandKind::DisplayForBlackEachMove,
                },
                CommandPart {
                    commands: vec!["ml".to_string(), "move list".to_string()],
                    description: "Display the move list notation".to_string(),
                    command_kind: CommandKind::DisplayMoveList,
                },
                CommandPart {
                    commands: vec!["c".to_string(), "captures".to_string()],
                    description: "Display captures for both players.".to_string(),
                    command_kind: CommandKind::DisplayCaptures,
                },
                CommandPart {
                    commands: vec!["fen".to_string()],
                    description: "Display the FEN layout of the board".to_string(),
                    command_kind: CommandKind::DisplayFEN,
                },
                CommandPart {
                    commands: vec![
                        "h".to_string(),
                        "dh".to_string(),
                        "help".to_string(),
                        "display help".to_string(),
                    ],
                    description: "Display this help message".to_string(),
                    command_kind: CommandKind::Help,
                },
                CommandPart {
                    commands: vec!["q".to_string(), "quit".to_string()],
                    description: "Quit the application".to_string(),
                    command_kind: CommandKind::Quit,
                },
            ],
        );

        // Insert commands for context `SwitchParser`.
        map.insert(
            CommandContext::SwitchParser,
            vec![
                CommandPart {
                    commands: vec!["1".to_string(), "algebraic".to_string()],
                    description: "Algebraic Parser (Not Implemented)".to_string(),
                    command_kind: CommandKind::SwitchToAlgebraicParser,
                },
                CommandPart {
                    commands: vec!["2".to_string(), "concise reversible".to_string()],
                    description: "Concise Reversible Parser (Not Implemented)".to_string(),
                    command_kind: CommandKind::SwitchToConciseReversibleParser,
                },
                CommandPart {
                    commands: vec!["3".to_string(), "Coordinate".to_string()],
                    description: "Coordinate Parser (Not Implemented)".to_string(),
                    command_kind: CommandKind::SwitchToCoordinateParser,
                },
                CommandPart {
                    commands: vec!["4".to_string(), "descriptive".to_string()],
                    description: "Descriptive Parser (Not Implemented)".to_string(),
                    command_kind: CommandKind::SwitchToDescriptiveParser,
                },
                CommandPart {
                    commands: vec!["5".to_string(), "iccf".to_string()],
                    description: "ICCF Parser".to_string(),
                    command_kind: CommandKind::SwitchToICCFParser,
                },
                CommandPart {
                    commands: vec!["6".to_string(), "long algebraic".to_string()],
                    description: "Long Algebraic Parser (Not Implemented)".to_string(),
                    command_kind: CommandKind::SwitchToLongAlgebraicParser,
                },
                CommandPart {
                    commands: vec!["7".to_string(), "reversible algebraic".to_string()],
                    description: "Reversible Algebraic Parser (Not Implemented)".to_string(),
                    command_kind: CommandKind::SwitchToReversibleAlgebraicParser,
                },
                CommandPart {
                    commands: vec!["8".to_string(), "smith".to_string()],
                    description: "Smith Parser (Not Implemented)".to_string(),
                    command_kind: CommandKind::SwitchToSmithParser,
                },
                CommandPart {
                    commands: vec![
                        "h".to_string(),
                        "dh".to_string(),
                        "help".to_string(),
                        "display help".to_string(),
                    ],
                    description: "Display this help message".to_string(),
                    command_kind: CommandKind::Help,
                },
                CommandPart {
                    commands: vec![
                        "b".to_string(),
                        "gb".to_string(),
                        "back".to_string(),
                        "go back".to_string(),
                    ],
                    description: "Go back".to_string(),
                    command_kind: CommandKind::Back,
                },
            ],
        );

        map
    }
}
