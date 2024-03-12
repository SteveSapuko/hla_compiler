use fancy_regex::Regex; 
use crate::definitions::*;

pub struct Lexer {
    data: String,
    pub ptr: usize
}

impl Lexer {
    pub fn new(text: String) -> Result<Self, ()> {
        if !text.is_ascii() {
            return Err(());
        }
        
        Ok( Lexer {
            data: text,
            ptr: 0,
        })
    }

    
    pub fn lex(&mut self) -> Result<Vec<Token>, usize> {
        let reg_key = Regex::new(r"(?x)
        ^let |
        ^if |
        ^fn |
        ^else |
        ^while |
        ^loop |
        ^for |
        ^return |
        ^continue |
        ^struct |
        ^enum |
        ^break").unwrap();

        let reg_op = Regex::new(r"(?x)
        ^ as(?=\s) |
        ^ \= (?!\=) |
        ^ \+ |
        ^ \- |
        ^ \& |
        ^ \* |
        ^ << |
        ^ >> |
        ^ \|(?!\|) | #single pipe
        ^ \! |
        ^ \~\\").unwrap();

        let reg_cond = Regex::new(r"(?x)
        ^ \|\| |
        ^ \&\& |
        ^ <\= |
        ^ >= |
        ^ < (?!<) |
        ^ > (?!>) |
        ^ \=\=").unwrap();

        let reg_id = Regex::new(r"^[_[[:alpha:]]][_@[[:alnum:]]]*").unwrap();

        let reg_lit = Regex::new(r"^-?\d+(?![[:alpha:]])").unwrap();

        let mut token_list: Vec<Token> = vec![];

        while self.data.as_bytes()[self.ptr] != b'\0' {
            //self.ptr = self.skip_whitespace()?;
            self.skip_whitespace();

            //println!("ptr: {}   char: {}", self.ptr, self.data.as_bytes()[self.ptr] as char);

            if self.ptr < self.data.len() - 2 && &self.data.as_str()[self.ptr..self.ptr+2] == "->" {
                token_list.push(Token {
                    ttype: TokenType::Arrow,
                    pos: self.ptr
                });
                self.ptr += 2;
                continue;
            }

            if let Some(m) = reg_key.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token{ttype: TokenType::Key(m.as_str().to_string()), pos: self.ptr});
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_op.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token{ttype: TokenType::Op(m.as_str().to_string()), pos: self.ptr});
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_cond.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token{ttype: TokenType::Cond(m.as_str().to_string()), pos: self.ptr});
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_id.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token{ttype: TokenType::Id(m.as_str().to_string()), pos: self.ptr});
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_lit.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token{ttype: TokenType::Lit(m.as_str().to_string()), pos: self.ptr});
                self.ptr += m.as_str().len();
                continue;
            }


            match self.data.as_bytes()[self.ptr] {
                b'(' => {token_list.push(Token {ttype: TokenType::ParenOpen, pos: self.ptr}); self.ptr += 1; continue},
                b')' => {token_list.push(Token {ttype: TokenType::ParenClose, pos: self.ptr}); self.ptr += 1; continue},
                b'[' => {token_list.push(Token {ttype: TokenType::SquareOpen, pos: self.ptr}); self.ptr += 1; continue},
                b']' => {token_list.push(Token {ttype: TokenType::SquareClose, pos: self.ptr}); self.ptr += 1; continue},
                b'{' => {token_list.push(Token {ttype: TokenType::CurlyOpen, pos: self.ptr}); self.ptr += 1; continue},
                b'}' => {token_list.push(Token {ttype: TokenType::CurlyClose, pos: self.ptr}); self.ptr += 1; continue},
                b';' => {token_list.push(Token {ttype: TokenType::SemiCol, pos: self.ptr}); self.ptr += 1; continue},
                b':' => {token_list.push(Token {ttype: TokenType::Col, pos: self.ptr}); self.ptr += 1; continue},
                b',' => {token_list.push(Token {ttype: TokenType::Comma, pos: self.ptr}); self.ptr += 1; continue},
                b'.' => {token_list.push(Token {ttype: TokenType::Period, pos: self.ptr}); self.ptr += 1; continue},
                _ => {}
            }

            return Err(self.ptr);
        }

        token_list.push(Token {ttype: TokenType::EOF, pos: self.ptr});
        return Ok(token_list);
    }

    fn skip_whitespace(&mut self) {
        while (self.data.as_bytes()[self.ptr] == b' ' || self.data.as_bytes()[self.ptr] == b'\n' || self.data.as_bytes()[self.ptr] == b'\t') && self.data.as_bytes()[self.ptr] != 0 {
            self.ptr += 1;
        }
    }
}


