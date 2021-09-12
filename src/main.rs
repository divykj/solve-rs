mod cli;
mod runners;

use clap::Clap;
use cli::Cli;
use runners::{run_expression, run_from_file, run_repl};

fn main() {
    let cli = Cli::parse();

    if let Some(file) = cli.file {
        run_from_file(file);
    } else if !cli.expressions.is_empty() {
        println!("{}", run_expression(cli.expressions.join(" ").as_str()));
    } else {
        run_repl();
    }
}
