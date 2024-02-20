mod lexer;
mod parser;
mod definitions;
mod expression;

use crate::{lexer::*, parser::Parser};
use std::{fs::File, io::Read};

fn main() {


     //let test = Regex::new(r"^[[:alpha:]][[:alnum:]]*").unwrap();
     //let test = Regex::new(r"[[:alpha:]]").unwrap();
     //println!("{:?}", test.find("xasd123 asdfh").unwrap().unwrap().as_str());
     
     let mut f = File::open("text.txt").unwrap();
     let mut text = String::new();
     f.read_to_string(&mut text).unwrap();

     

     let mut l = Lexer::new(text).unwrap();
     let tokens = l.lex().unwrap();

     let mut parser = Parser::new(tokens);
     println!("{:?}", parser);
     let ast = parser.parse();

     println!("{}", ast);

}
