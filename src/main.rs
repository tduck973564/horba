use std::env::args;
use std::error::Error;
use std::fs;
use std::process::exit;

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
}

// That () in the return type should be ! but its experimental so /shrug
// This doesn't return anything other than an Err because it exits out if it worked
fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(path)?;
    match run(file) {
        true => exit(1),
        false => exit(0),
    }
}

fn run_prompt() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) if line == *".exit" => exit(0),
            Ok(line) => { run(line); },
            Err(_) => println!(),
        }
    }
}

fn run(source: String) -> bool {
    let mut scanner = scanner::scanner::Scanner::new(source.clone());
    let tokens = scanner.scan_tokens();

    let mut exit_err = false;

    /*println!();
    for token in &tokens {
        println!("{}", token);
    }
    println!();*/

    let mut parser = parser::Parser::new(source.clone(), tokens);
    let interpreter = parser::interpreter::Interpreter {};

    while let Some(expression) = parser.parse() {
        /*println!("expr: {:?}", expression);
        println!(
            "astprinter: {}\n",
            parser::ast_printer::AstPrinter {}.print(expression)
        )*/
        interpreter.interpret(expression, &source).unwrap_or_else(|_| { exit_err = false; () });
    }

    exit_err
}
