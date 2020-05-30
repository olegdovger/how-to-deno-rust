use crate::cli_commands::CLICommandInterface;

#[derive(Debug, Default)]
pub struct HelpCommand {}

impl CLICommandInterface for HelpCommand {
    fn run() {
        println!("\n\n  There are available commands:\n");
        println!("    serve - Build rust modules in project");
        println!("    help  - Help");
        print!("\n");
    }
}
