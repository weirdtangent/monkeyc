pub mod lexer;
pub mod token;

use std::env;
use std::fs;
use std::io;

use lexer::Lexer;
use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        process_file(&args[1]);
    } else {
        process_stdin();
    }
    println!("End.");
}

fn process_stdin() {
    loop {
        print!(">> ");

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(size) if size > 1 => tokenize(&line),
            Ok(size) if size == 1 => println!("CTRL-D to end."),
            Ok(size) if size == 0 => break,
            _ => panic!("Error reading from stdin"),
        }
    }
}

fn process_file(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Reached end of file");
    for line in contents.lines() {
        print!(">> ");
        tokenize(line);
    }
}

fn tokenize(mut line: &str) {
    let mut lexer = Lexer::new(&mut line);

    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        if tok == Token::EndOfLine {
            break;
        }
    }
}
