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

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Pointer(Box<VarType>),
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    UserStruct(UserStruct),
    Array(u16, Box<VarType>)
}

impl VarType {
    pub fn size(&self) -> u16 {
        match self {
            Self::U8 => 1,
            Self::I8 => 1,
            Self::U16 => 2,
            Self::I16 => 2,
            Self::U32 => 4,
            Self::I32 => 4,
            Self::U64 => 8,
            Self::I64 => 8,
            Self::Pointer(_) => 2,
            Self::UserStruct(s) => {
                let mut sum: u16 = 0;
                for t in &s.fields {
                    sum += t.1.size();
                }
                sum
            }
            Self::Array(s, _) => *s

        }
    }

    pub fn from(t: &str, defined_types: &Vec<UserStruct>) -> Result<Self, &'static str> {
        match t.to_lowercase().as_str() {
            "u8" => Ok(Self::U8),
            "i8" => Ok(Self::I8),

            "u16" => Ok(Self::U16),
            "i16" => Ok(Self::I16),

            "u32" => Ok(Self::U32),
            "i32" => Ok(Self::I32),

            "u64" => Ok(Self::U64),
            "i64" => Ok(Self::I64),

            _ => {
                for user_type in defined_types {
                    if t == user_type.name {
                        return Ok(VarType::UserStruct(user_type.clone()))
                    }
                }

                return Err("Undefined Type")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserStruct {
    pub name: String,
    pub fields: Vec<(String, VarType)>
}