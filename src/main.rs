use std::env::args;
use std::error::Error;
use std::fs;

pub mod error;
pub mod parser;
pub mod scanner;

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        x if x > 2 => println!("Usage: horba [script]"),
        x if x == 2 => run_file(&args[1]).expect("Could not run file"),
        _ => run_prompt(),
    };
    parser::ast_printer::ast_test();
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(path)?;
    run(file);
    Ok(())
}

fn run_prompt() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => run(line),
            Err(_) => println!(),
        }
    }
}

fn run(source: String) {
    let mut scanner = scanner::scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
