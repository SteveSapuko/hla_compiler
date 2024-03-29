use crate::definitions::*;
use crate::statement::*;

#[derive(Debug)]
pub struct Parser{
    pub tokens: Vec<Token>,
    pub ptr: usize,
}

impl Parser {
    pub fn new(tokens:  Vec<Token>) -> Self {
        return Parser {
            tokens: tokens,
            ptr: 0
        }
    }

    pub fn peek(&self, n: i64) -> Token {
        let peek_location: usize = (self.ptr as i64 + n) as usize;
        
        let temp = self.tokens[peek_location].clone();
        //println!("{:?}", temp);
        return temp
    }

    pub fn peek_forward(&self, n: i64) -> Option<Token> {
        let peek_location: usize = (self.ptr as i64 + n) as usize;
        
        if peek_location >= self.tokens.len() {
            return None
        }

        let temp = self.tokens[peek_location].clone();
        //println!("{:?}", temp);
        return Some(temp)
    }

    pub fn advance(&mut self) {
        //println!("{}", self.ptr);
        self.ptr += 1;
    }


    pub fn parse(&mut self) -> Result<Vec<Statement>, &'static str> {
        let mut program: Vec<Statement> = vec![];
        
        //self.make_ptr_types();

        while self.peek(0).ttype != TokenType::EOF {
            program.push( new_statement("Base").parse(self)?);
        }

        Ok(program)
    }

    /*fn make_ptr_types(&mut self) {
        while self.peek(0).ttype != TokenType::EOF {
            if self.peek(0).ttype == TokenType::Key("ptr@".to_string()) {
                if matches!(self.peek(1).ttype, TokenType::Id(_)) {
                    self.tokens[self.ptr] = Token {
                        ttype: TokenType::Id("ptr@".to_string() + &self.peek(1).data()),
                        pos: self.peek(0).pos
                    };

                    self.tokens.remove(self.ptr + 1);
                }
            }
            self.advance();
        }

        println!("{:#?}", self.tokens);
        self.ptr = 0;
    }*/
}