pub mod help;
pub mod lib_compiler;

pub trait CLICommandInterface {
    fn run();
}

pub use crate::{cli_commands::help::HelpCommand, cli_commands::lib_compiler::Compiler};
