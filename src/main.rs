mod lexer;
mod parser;
mod definitions;
mod expression;
mod statement;
mod syntax;
mod cgen;

use crate::{lexer::*, parser::*, syntax::*};
use std::{fs::File, io::Read};

fn main() {     
     let mut f = File::open("text.txt").unwrap();
     let mut text = String::new();
     f.read_to_string(&mut text).unwrap();
     text = text.trim().to_string();
     text.push('\0');


     let mut l = Lexer::new(text.clone()).unwrap();
     let tokens;
     match l.lex() {
          Ok(d) => tokens = d,
          Err(p) => {
               let (line, col) = find_relative_pos(p, text);
               println!("Lexing Error at Line: {} Col: {}", line, col);
               std::process::exit(-1);
          }
     }

     let mut parser = Parser::new(tokens.clone());
     println!("{:?}\n", parser);
     let ast = parser.parse();

     if let Err(e) = ast.clone() {
          let (line, col) = find_relative_pos(parser.tokens[parser.ptr - 1].pos, text);
          println!("PARSING ERROR - {} - at Ln: {} Col: {} - Token: {}\n", e, line, col, parser.tokens[parser.ptr - 1].ttype);
          std::process::exit(-1);
     }

     let ast = ast.unwrap();

     for s in ast.clone() {
          println!("{}", s);
     }

     if let Err(e) = check_ast_syntax(ast.clone()) {
          println!("\n{:#?}", e);
     }

}


//(Line, Col)
fn find_relative_pos(target: usize, f: String) -> (usize, usize) {
     let mut absolute_pos: usize = 0;
     //println!("Target - {}", target);

     for line in f.lines().into_iter().enumerate() {
          let line_n = line.0; 
          let mut line = line.1.to_string();
          line.push('\0'); //to accomodate the fact that \n is not included
          //println!("{}", line);

          for c in line.as_bytes().into_iter().enumerate() {
               let pos_in_line = c.0;
               //println!("Absolute - {}", absolute_pos);
               if absolute_pos == target {
                    return (line_n + 1, pos_in_line + 2)
               }

               absolute_pos += 1;
          }
     }
     
     panic!("relative pos ain't working");
}