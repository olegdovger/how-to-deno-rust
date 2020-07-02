use crate::cli_commands::*;
use std::env::Args;

pub fn run_command(args: Args) {
    let vec_args = &args.collect::<Vec<_>>()[1..];

    match vec_args.len() {
        s if s > 0 => {
            let command: &str = &vec_args[0];

            match command {
                "serve" => Compiler::run(),
                "help" => HelpCommand::run(),
                _ => println!(),
            }
        }
        _ => println!("\n\tError. Run command as \"cargo run command_name\""),
    };
}
