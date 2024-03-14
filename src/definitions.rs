use super::statement::*;
use super::parser::Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub pos: usize,
}

impl Token {
    pub fn data(&self) -> String {
        self.ttype.data()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Key(String),
    Op(String),
    Cond(String),
    Id(String),
    Lit(String),
    ParenOpen,
    ParenClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    SemiCol,
    Col,
    Comma,
    Period,
    Arrow,
    EOF,
}

impl TokenType {
    pub fn data(&self) -> String{
        match self {
            TokenType::Key(d) => d.clone(),
            TokenType::Op(d) => d.clone(),
            TokenType::Cond(d) => d.clone(),
            TokenType::Id(d) => d.clone(),
            TokenType::Lit(d) => d.clone(),
            _ => panic!("Token didn't have data")
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(d) => write!(f, "{}", d),
            Self::Op(d) => write!(f, "{}", d),
            Self::Cond(d) => write!(f, "{}", d),
            Self::Id(d) => write!(f, "{}", d),
            Self::Lit(d) => write!(f, "{}", d),
            Self::ParenOpen => write!(f, "("),
            Self::ParenClose => write!(f, ")"),
            Self::SquareOpen => write!(f, "["),
            Self::SquareClose => write!(f, "]"),
            Self::CurlyOpen => write!(f, "{{"),
            Self::CurlyClose => write!(f, "}}"),
            Self::SemiCol => write!(f, ";"),
            Self::Col => write!(f, ":"),
            Self::Arrow => write!(f, "->"),
            Self::EOF => write!(f, "EOF"),
            Self::Comma => write!(f, ","),
            Self::Period => write!(f, "."),
        }
    }
}

impl Statement {
    pub fn get_param_vec(&self) -> Vec<(Token, DeclrType)> {
        match self {
            Self::Parameters(d) => d.clone(),
            
            _ => panic!("cannot get params from this statement")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclrType {
    BasicType(Token),
    Array(Box<DeclrType>, Token),
    Pointer(Box<DeclrType>),
}

impl DeclrType {
    pub fn get_token(&self) -> Token {
        match self.clone() {
            Self::BasicType(t) => t,
            Self::Array(t, s) => t.get_token(),
            Self::Pointer(t) => t.get_token(),
        }
    }
}

pub fn parse_type(p: &mut Parser) -> Result<DeclrType, &'static str> {
    let vtype;
    
    if matches!(p.peek(0).ttype, TokenType::Id(_)) {
        vtype = DeclrType::BasicType(p.peek(0));
        p.advance();

    } else if p.peek(0).ttype == TokenType::SquareOpen {
        p.advance();
        
        let array_type = parse_type(p)?;

        if p.peek(0).ttype != TokenType::SemiCol {
            return Err("Expected Semicolon after Array Type")
        }
        p.advance();

        if !matches!(p.peek(0).ttype, TokenType::Lit(_)) {
            return Err("Expected Literal for Array Size")
        }

        let array_size = p.peek(0);
        p.advance();
        
        if p.peek(0).ttype != TokenType::SquareClose {
            return Err("Expected Closing Square Bracket after Array Size")
        }
        p.advance();

        vtype = DeclrType::Array(Box::new(array_type), array_size)

    } else if p.peek(0).ttype == TokenType::Key("@".to_string()){
        p.advance();
        let points_to_type = parse_type(p)?;
        return Ok(DeclrType::Pointer(Box::new(points_to_type)))

    } else {
        return Err("Cannot Parse Type")
    }
    
    
    Ok(vtype)
}


pub static RESERVED_IDS: [&str; 1] = [
    "void",
];

pub const BLANK_TOKEN: Token = Token { ttype: TokenType::Arrow, pos: 0 };