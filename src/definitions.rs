use crate::expression::*;

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
        }
    }
}

/*impl Expr {
    pub fn get_id_value(&self) -> String {
        match self.clone() {
            Self::Primary(p) => {
                match *p {
                    PrimaryExpr::Id(s) => s.ttype.data(),
                    _ => panic!("Use this only on ID Expressions")
                }
            }
            _ => panic!("use this only on ID Expressions")
        }
    }
}*/