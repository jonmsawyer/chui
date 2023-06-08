//! Chui: Console Application

use chui_core::{ChuiResult, Color, Command, CommandContext, CommandKind, Game, WinCondition};

/// [`Console`] struct that drives the console application.
#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Console {}

impl Console {
    pub fn new() -> Console {
        Console {}
    }

    /// Run the engine.
    ///
    /// # Errors
    ///
    /// * Errors when...
    pub fn run(&mut self) -> ChuiResult<()> {
        let mut game = Game::default();
        let mut command = Command::new(&game);
        let context = CommandContext::Main;
        let mut break_loop = false;
        let mut display_board = true;

        loop {
            if display_board {
                println!("{}", game.to_move_to_string());
            } else {
                display_board = true;
            }
            println!();
            println!("Please input move(s) or command. (q to quit, h for help)");

            let move_input = Game::get_input();

            for move_str in move_input.split_whitespace() {
                let the_move = String::from(move_str);
                let command_move = the_move.clone();
                match command.process_command(context, command_move) {
                    Some(CommandKind::Quit) => {
                        break_loop = true;
                    }

                    Some(CommandKind::Help) => {
                        command.display_help(context);
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::SwitchParser) => {
                        game.switch_parser(&command);
                        command.rebuild_commands(&game);
                        continue;
                    }

                    Some(CommandKind::DisplayToMove) => {
                        println!();
                        println!("{}", game.to_move_to_string());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::DisplayForWhite) => {
                        println!();
                        println!("{}", game.white_to_string());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::DisplayForBlack) => {
                        println!();
                        println!("{}", game.black_to_string());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::DisplayFEN) => {
                        println!();
                        println!("{}", game.get_fen());
                        display_board = false;
                        continue;
                    }

                    Some(CommandKind::WhiteResigns) => {
                        println!();
                        println!("White resigns.");
                        game.win_condition = Some(WinCondition::WhiteResigns);
                        game.draw_condition = None;
                        continue;
                    }

                    Some(CommandKind::BlackResigns) => {
                        println!();
                        println!("Black resigns.");
                        game.win_condition = Some(WinCondition::BlackResigns);
                        game.draw_condition = None;
                        continue;
                    }

                    Some(CommandKind::DisplayForWhiteEachMove) => {
                        println!();
                        println!("Display for White after each move.");
                        game.display_for = Some(Color::White);
                        continue;
                    }

                    Some(CommandKind::DisplayForBlackEachMove) => {
                        println!();
                        println!("Display for Black after each move.");
                        game.display_for = Some(Color::Black);
                        continue;
                    }

                    Some(CommandKind::DisplayMoveList) => {
                        let mut output = String::new();

                        println!();

                        for (move_idx, move_obj) in game.move_list.iter().enumerate() {
                            let numeral = if move_idx % 2 == 0 {
                                format!("\n{}. ", (move_idx + 2) / 2)
                            } else {
                                String::new()
                            };

                            output = format!("{}{}{} ", output, numeral, move_obj);
                        }

                        if game.move_list.is_empty() {
                            output = "No moves have been made.".to_string();
                        }

                        display_board = false;

                        println!("Move List Notation:\n{}", output.trim());
                    }

                    _ => {
                        println!();
                        println!("Input move or command: {}", the_move);

                        // Ignore any moves or commands with a '.' in it.
                        // Eg., "1."
                        if the_move.contains('.') {
                            continue;
                        }

                        if the_move.eq("1-0") {
                            game.win_condition = Some(WinCondition::BlackResigns);
                            game.draw_condition = None;
                        } else if the_move.eq("0-1") {
                            game.win_condition = Some(WinCondition::WhiteResigns);
                            game.draw_condition = None;
                        } else if the_move.eq("1/2-1/2") || the_move.eq("½-½") {
                            game.win_condition = None;
                            game.draw_condition = None; // TODO: ?
                        }

                        match game.parse(the_move, game.to_move).as_ref() {
                            Ok(move_obj) => {
                                println!("Ok! The move: {:?}", move_obj);
                                game.current_move = Some(move_obj.clone());
                                if game.apply_move().is_ok() {
                                    println!("{}", move_obj.get_move_text());
                                    println!();

                                    game.move_list.push(move_obj.clone());

                                    game.half_move_counter += 1;
                                    if game.half_move_counter % 2 == 0 {
                                        game.move_counter += 1;
                                    }
                                } else {
                                    println!("Move not applied.");
                                    break;
                                }
                            }

                            Err(error) => println!("{}", error),
                        }
                    }
                }
            }

            if break_loop {
                break;
            }
        }

        Ok(())
    }
}
