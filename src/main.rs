mod calculator;
mod error;
mod parser;
mod tokenizer;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if !args.is_empty() {
        let expression = args.join(" ");
        match parser::evaluate(&expression) {
            Ok(result) => println!("{result}"),
            Err(err) => {
                eprintln!("error: {err}");
                std::process::exit(1);
            }
        }
        return;
    }

    run_repl();
}

fn run_repl() {
    println!("Cli Calculator");
    println!("Enter an expression like `3 + 4` (operators: + - * /). Type q to quit.");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");

        if stdout.flush().is_err() {
            break;
        }

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(err) => {
                eprintln!("error reading input: {err}");
                break;
            }
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.eq_ignore_ascii_case("q") || line.eq_ignore_ascii_case("quit") {
            break;
        }

        match parser::evaluate(line) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("error: {err}"),
        }
    }
}
