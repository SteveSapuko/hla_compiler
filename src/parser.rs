use crate::definitions::*;
use crate::expression::*;

#[derive(Debug)]
pub struct Parser{
    tokens: Vec<Token>,
    ptr: usize,
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


    pub fn parse(&mut self) -> Expr {
        let mut e = new_expr('B');
        e = e.eval(self);
        e
    }
}