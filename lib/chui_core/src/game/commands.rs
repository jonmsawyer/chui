//! Commands module.

use crate::prelude::*;

/// Switch the current move parser based on a `CommandKind`.
pub fn switch_parser(game: &mut Game, command: &Command) {
    let context: CommandContext = CommandContext::SwitchParser;

    loop {
        println!();
        println!("Current parser: {}", game.parser.name());
        command.display_help(context);
        println!();
        println!("Select option. (1-8, b to go back, h for help)");

        let input: String = Game::get_input();

        match command.process_command(context, input) {
            Some(CommandKind::SwitchToAlgebraicParser) => {
                let parser_engine: ParserEngine = ParserEngine::Algebraic;
                println!("Switching parser to {:?}.", parser_engine);
                game.set_parser(parser_engine);
                break;
            }

            // Some(CommandKind::SwitchToConciseReversibleParser) => {
            //     let parser_engine = ParserEngine::ConciseReversible;
            //     println!("Switching parser to {:?}.", parser_engine);
            //     game.set_parser(parser_engine);
            //     break;
            // }

            // Some(CommandKind::SwitchToCoordinateParser) => {
            //     let parser_engine = ParserEngine::Coordinate;
            //     println!("Switching parser to {:?}.", parser_engine);
            //     game.set_parser(parser_engine);
            //     break;
            // }

            // Some(CommandKind::SwitchToDescriptiveParser) => {
            //     let parser_engine = ParserEngine::Descriptive;
            //     println!("Switching parser to {:?}.", parser_engine);
            //     game.set_parser(parser_engine);
            //     break;
            // }
            Some(CommandKind::SwitchToICCFParser) => {
                let parser_engine = ParserEngine::ICCF;
                println!("Switching parser to {:?}.", parser_engine);
                game.set_parser(parser_engine);
                break;
            }

            // Some(CommandKind::SwitchToLongAlgebraicParser) => {
            //     let parser_engine = ParserEngine::LongAlgebraic;
            //     println!("Switching parser to {:?}.", parser_engine);
            //     game.set_parser(parser_engine);
            //     break;
            // }

            // Some(CommandKind::SwitchToReversibleAlgebraicParser) => {
            //     let parser_engine = ParserEngine::ReversibleAlgebraic;
            //     println!("Switching parser to {:?}.", parser_engine);
            //     game.set_parser(parser_engine);
            //     break;
            // }

            // Some(CommandKind::SwitchToSmithParser) => {
            //     let parser_engine = ParserEngine::Smith;
            //     println!("Switching parser to {:?}.", parser_engine);
            //     game.set_parser(parser_engine);
            //     break;
            // }
            Some(CommandKind::Help) => {
                continue;
            }

            Some(CommandKind::Back) => {
                println!("Not switching parser.");
                break;
            }

            _ => println!("Invalid option."),
        }
    }
}
