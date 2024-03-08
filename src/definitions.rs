#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

impl Token {
    pub fn data(&self) -> Option<String>{
        match self {
            Token::Key(d) => Some(d.clone()),
            Token::Op(d) => Some(d.clone()),
            Token::Cond(d) => Some(d.clone()),
            Token::Id(d) => Some(d.clone()),
            Token::Lit(d) => Some(d.clone()),
            _ => None
        }
    }
}

impl std::fmt::Display for Token {
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