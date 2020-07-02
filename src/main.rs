mod cli;
mod cli_commands;
use std::env;

fn main() {
    cli::run_command(env::args());
}
