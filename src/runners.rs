use crate::cli::Cli;
use clap::IntoApp;
use solve_lib::run;
use std::{fs::File, io::Read, path::PathBuf};

pub(crate) fn run_from_file(file: PathBuf) {
    let mut file = File::open(file).expect("Failed to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file.");

    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .for_each(|expression| println!("{} = {}", expression, run_expression(expression)));
}

pub(crate) fn run_expression(expression: &str) -> String {
    match run(expression) {
        Ok(result) => format!("{}", result),
        Err(error) => format!("{:#}", error),
    }
}

pub(crate) fn run_repl() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut app = Cli::into_app().help_template("{bin} {version}\n{about}");

    app.print_help().unwrap();
    loop {
        match rl.readline(">> ").as_deref().map(|line| line.trim()) {
            Ok("") => {
                continue;
            }
            Ok("exit") => {
                println!("Bye!");
            }
            Ok(entry) => {
                rl.add_history_entry(entry);
                println!("{}", run_expression(entry));
                continue;
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
            }
            Err(rustyline::error::ReadlineError::Eof) => {}
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
        break;
    }
}
