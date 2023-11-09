//! Chui: Console Application

use chui_core::prelude::*;

/// Log a blank line to the console.
pub fn log() {
    println!();
}

/// Log output to the console.
pub fn log_ln(text: String) {
    println!("{}", text);
}

/// Log output to the console.
pub fn log_str(text: &str) {
    println!("{}", text);
}

/// Log output to the console with a given line ending.
pub fn _log_ending(text: String, ending: &str) {
    print!("{}{}", text, ending);
}

/// Run the engine.
///
/// # Errors
///
/// * Errors when...
pub fn run() -> ChuiResult<()> {
    let mut game: Game = Game::default();
    let mut command: Command = Command::new(&game);
    let context: CommandContext = CommandContext::Main;
    let mut break_loop: bool = false;
    let mut display_board: bool = true;

    loop {
        if display_board {
            log_ln(game.to_move_to_string());
        } else {
            display_board = true;
        }
        log();
        log_str("Please input move(s) or command. (q to quit, h for help)");

        let move_input: String = Game::get_input();

        for move_str in move_input.split_whitespace() {
            let mut move_string: String = String::from(move_str);
            let input_command: String = move_string.clone();
            match command.process_command(context, input_command) {
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
                    log();
                    log_ln(game.to_move_to_string());
                    display_board = false;
                    continue;
                }
                Some(CommandKind::DisplayForWhite) => {
                    log();
                    log_ln(game.white_to_string());
                    display_board = false;
                    continue;
                }
                Some(CommandKind::DisplayForBlack) => {
                    log();
                    log_ln(game.black_to_string());
                    display_board = false;
                    continue;
                }
                Some(CommandKind::DisplayFEN) => {
                    log();
                    log_ln(game.get_fen());
                    display_board = false;
                    continue;
                }
                Some(CommandKind::WhiteResigns) => {
                    log();
                    log_str("White resigns.");
                    game.win_condition = Some(WinCondition::WhiteResigns);
                    game.draw_condition = None;
                    continue;
                }
                Some(CommandKind::BlackResigns) => {
                    log();
                    log_str("Black resigns.");
                    game.win_condition = Some(WinCondition::BlackResigns);
                    game.draw_condition = None;
                    continue;
                }
                Some(CommandKind::DisplayForWhiteEachMove) => {
                    log();
                    log_str("Display for White after each move.");
                    game.display_for = Some(Color::White);
                    continue;
                }
                Some(CommandKind::DisplayForBlackEachMove) => {
                    log();
                    log_str("Display for Black after each move.");
                    game.display_for = Some(Color::Black);
                    continue;
                }
                Some(CommandKind::DisplayMoveList) => {
                    let mut output = String::new();

                    log();

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

                    log_ln(format!("Move List Notation:\n{}", output.trim()));
                }
                Some(CommandKind::DisplayCaptures) => {
                    let mut white_output = String::new();
                    let mut black_output = String::new();

                    log();

                    for piece in game.captured_pieces.iter() {
                        match piece.get_color() {
                            Color::White => {
                                black_output = format!("{}{} ", black_output, piece);
                            }

                            Color::Black => {
                                white_output = format!("{}{} ", white_output, piece);
                            }
                        }
                    }

                    display_board = false;

                    log_str("Captures:");
                    log_ln(format!("White: {}", white_output));
                    log_ln(format!("Black: {}", black_output));
                }
                _ => {
                    log();
                    log_ln(format!("Input move or command: {}", move_string));

                    let move_string_vec: Vec<&str> = move_str.split('.').collect::<Vec<_>>();
                    log_ln(format!("Move string parts: {:?}", move_string_vec));
                    if move_string_vec.len() > 1 {
                        move_string = move_string_vec[1].to_string();
                        log_ln(format!("Move string is now: {}", move_string));
                    }

                    if move_string.eq("1-0") {
                        game.win_condition = Some(WinCondition::BlackResigns);
                        game.draw_condition = None;
                        break;
                    } else if move_string.eq("0-1") {
                        game.win_condition = Some(WinCondition::WhiteResigns);
                        game.draw_condition = None;
                        break;
                    } else if move_string.eq("1/2-1/2") || move_string.eq("½-½") {
                        game.win_condition = None;
                        game.draw_condition = None; // TODO: ?
                        break;
                    }

                    match game.parse(move_string, game.to_move).as_mut() {
                        Ok(chess_move) => {
                            if let Err(result) = chess_move.validate_move_for_board(&game.board) {
                                log_ln(result.to_string());
                                log_str("Move was found to be invalid by the validation function. Move not applied.");
                                continue;
                            }
                            game.set_current_move(Some(chess_move.clone()));
                            if game.apply_move().is_ok() {
                                game.move_list.push(chess_move.clone());

                                game.half_move_counter += 1;
                                if game.half_move_counter % 2 == 0 {
                                    game.move_counter += 1;
                                }
                            } else {
                                log_str("Move not applied.");
                                continue;
                            }
                        }

                        Err(error) => log_ln(format!("{}", error)),
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
