pub mod lexer;
pub mod token;

use std::env;
use std::fs;

use token::Token;
use lexer::Lexer;

fn main() {

  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Must provide .monk source filename");
  }
  let filename = &args[1];

  let contents = fs::read_to_string(filename)
    .expect("Reached end of file");

  for mut line in contents.lines() {
    print!(">> ");

    let mut lexer = Lexer::new(&mut line);

    loop {
      let tok = lexer.next_token();
      println!("{:?}", tok);
      if tok == Token::EndOfLine {
        break;
      }
    }
  }
}
