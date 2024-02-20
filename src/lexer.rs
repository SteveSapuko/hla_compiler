use fancy_regex::Regex; 
use crate::definitions::*;

pub struct Lexer {
    data: String,
    ptr: usize
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
        ^for |
        ^as |
        ^return |
        ^continue |
        ^break").unwrap();

        let reg_op = Regex::new(r"(?x)
        ^ \= (?!\=) |
        ^ \+ |
        ^ \- |
        ^ \& |
        ^ \* |
        ^ << |
        ^ >> |
        ^ \|(?!\|) | #single pipe
        ^ \~ |
        ^ \~\\").unwrap();

        let reg_cond = Regex::new(r"(?x)
        ^ \|\| |
        ^ \&\& |
        ^ <\= |
        ^ >= |
        ^ < (?!<) |
        ^ > (?!>) |
        ^ \=\=").unwrap();

        let reg_id = Regex::new(r"^[[:alpha:]][[:alnum:]]*").unwrap();

        let reg_lit = Regex::new(r"^\d+").unwrap();

        let mut token_list: Vec<Token> = vec![]; 

        while self.ptr < self.data.len() {
            self.ptr = self.skip_whitespace();            

            if let Some(m) = reg_key.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Key(m.as_str().to_string()));
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_op.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Op(m.as_str().to_string()));
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_cond.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Cond(m.as_str().to_string()));
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_id.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Id(m.as_str().to_string()));
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_lit.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Lit(m.as_str().to_string()));
                self.ptr += m.as_str().len();
                continue;
            }

            match self.data.as_bytes()[self.ptr] {
                b'(' => {token_list.push(Token::ParenOpen); self.ptr += 1; continue},
                b')' => {token_list.push(Token::ParenClose); self.ptr += 1; continue},
                b'[' => {token_list.push(Token::SquareOpen); self.ptr += 1; continue},
                b']' => {token_list.push(Token::SquareClose); self.ptr += 1; continue},
                b'{' => {token_list.push(Token::CurlyOpen); self.ptr += 1; continue},
                b'}' => {token_list.push(Token::CurlyClose); self.ptr += 1; continue},
                b';' => {token_list.push(Token::SemiCol); self.ptr += 1; continue},
                b':' => {token_list.push(Token::Col); self.ptr += 1; continue},
                _ => {}
            }

            return Err(self.ptr);
        }

        token_list.push(Token::EOF);
        return Ok(token_list);
    }

    fn skip_whitespace(&self) -> usize {
        let mut new_ptr = self.ptr;

        loop {
            let cur = self.data.as_bytes()[new_ptr];
            
            if cur == b' ' || cur == b'\n' || cur == b'\t' {
                new_ptr += 1;
                continue
            }

            break
        }

        return new_ptr
    }
}


