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

    
    pub fn lex(&mut self) -> Result<(Vec<Token>, Vec<usize>), usize> {
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
        ^break").unwrap();

        let reg_op = Regex::new(r"(?x)
        ^ as |
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

        let reg_id = Regex::new(r"^[[:alpha:]][[:alnum:]]*").unwrap();

        let reg_lit = Regex::new(r"^\d+(?![[:alpha:]])").unwrap();

        let mut token_list: Vec<Token> = vec![];
        let mut token_pos: Vec<usize> = vec![]; 

        while self.data.as_bytes()[self.ptr] != b'\0' {
            //self.ptr = self.skip_whitespace()?;
            self.skip_whitespace();

            if let Some(m) = reg_key.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Key(m.as_str().to_string()));
                token_pos.push(self.ptr);
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_op.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Op(m.as_str().to_string()));
                token_pos.push(self.ptr);
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_cond.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Cond(m.as_str().to_string()));
                token_pos.push(self.ptr);
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_id.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Id(m.as_str().to_string()));
                token_pos.push(self.ptr);
                self.ptr += m.as_str().len();
                continue;
            }

            if let Some(m) = reg_lit.find(&self.data.as_str()[self.ptr..]).unwrap() {
                token_list.push(Token::Lit(m.as_str().to_string()));
                token_pos.push(self.ptr);
                self.ptr += m.as_str().len();
                continue;
            }

            match self.data.as_bytes()[self.ptr] {
                b'(' => {token_list.push(Token::ParenOpen); token_pos.push(self.ptr); self.ptr += 1; continue},
                b')' => {token_list.push(Token::ParenClose); token_pos.push(self.ptr); self.ptr += 1; continue},
                b'[' => {token_list.push(Token::SquareOpen); token_pos.push(self.ptr); self.ptr += 1; continue},
                b']' => {token_list.push(Token::SquareClose); token_pos.push(self.ptr); self.ptr += 1; continue},
                b'{' => {token_list.push(Token::CurlyOpen); token_pos.push(self.ptr); self.ptr += 1; continue},
                b'}' => {token_list.push(Token::CurlyClose); token_pos.push(self.ptr); self.ptr += 1; continue},
                b';' => {token_list.push(Token::SemiCol); token_pos.push(self.ptr); self.ptr += 1; continue},
                b':' => {token_list.push(Token::Col); token_pos.push(self.ptr); self.ptr += 1; continue},
                _ => {}
            }

            return Err(self.ptr);
        }

        token_list.push(Token::EOF);
        token_pos.push(self.ptr);
        return Ok((token_list, token_pos));
    }

    fn skip_whitespace(&mut self) {
        while (self.data.as_bytes()[self.ptr] == b' ' || self.data.as_bytes()[self.ptr] == b'\n' || self.data.as_bytes()[self.ptr] == b'\t') && self.data.as_bytes()[self.ptr] != 0 {
            self.ptr += 1;
        }
    }
}


