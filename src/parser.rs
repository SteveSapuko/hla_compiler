use crate::definitions::*;
use crate::statement::*;

#[derive(Debug)]
pub struct Parser{
    tokens: Vec<Token>,
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
        let temp = self.tokens[(self.ptr as i64 + n) as usize].clone();
        //println!("{:?}", temp);
        return temp
    }

    pub fn advance(&mut self) {
        //println!("{}", self.ptr);
        self.ptr += 1;
    }


    pub fn parse(&mut self) -> Result<Vec<Statement>, &'static str> {
        let mut program: Vec<Statement> = vec![];
        
        while self.peek(0) != Token::EOF {
            program.push( new_statement("Base").parse(self)?);
        }

        Ok(program)
    }
}