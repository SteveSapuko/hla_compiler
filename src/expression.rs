use crate::definitions::*;
use crate::parser::*;


#[derive(Clone, Debug)]
pub enum Expr {
    Base,
    Assign(Box<BinaryExpr>),
    Equality(Box<BinaryExpr>),
    Comparison(Box<BinaryExpr>),
    Term(Box<BinaryExpr>),
    Shift(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Primary(Box<PrimaryExpr>),
}

#[derive(Clone, Debug)]
pub enum PrimaryExpr {
    Grouping(Expr),
    Literal(String),
    Id(String),
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    left: Expr,
    operator: Token,
    right: Expr,
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    operator: Token,
    right: Expr
}

pub fn new_expr(t: char) -> Expr {
    match t {
        'B' => Expr::Base,
        'A' => Expr::Assign(Box::new(BinaryExpr{
            left: new_expr('E'),
            operator: Token::Op("=".to_string()),
            right: new_expr('E'),}
        )),
        'E' => Expr::Equality(Box::new(
            BinaryExpr {
                left: new_expr('C'),
                operator: Token::Arrow,
                right: new_expr('C'),
            }
        )),
        'C' => Expr::Comparison(Box::new(
            BinaryExpr {
                left: new_expr('T'),
                operator: Token::Arrow,
                right: new_expr('T'),
            }
        )),
        'T' => Expr::Term(Box::new(
            BinaryExpr {
                left: new_expr('S'),
                operator: Token::Arrow,
                right: new_expr('S'),
            }
        )),
        'S' => Expr::Shift(Box::new(
            BinaryExpr {
                left: new_expr('U'),
                operator: Token::Arrow,
                right: new_expr('P')
            }
        )),
        'U' => Expr::Unary(Box::new(
            UnaryExpr {
                operator: Token::Arrow,
                right: new_expr('P'),
            }
        )),
        'P' => Expr::Primary(Box::new(PrimaryExpr::Literal(String::new()))),
        _ => panic!("new_expr invalid syntax")
    }
}

impl Expr {    
    pub fn parse(&mut self, p: &mut Parser) -> Result<Expr, &'static str> {
        *self = match self {
            Expr::Base => {
                let mut e = new_expr('A');
                e.parse(p)?
            }

            Expr::Assign(_) => {
                let mut e = new_expr('E').parse(p)?;

                if Token::Op("=".to_string()) == p.peek(0) {
                    p.advance();
                    let right = new_expr('B').parse(p)?;

                    e = Expr::Assign(Box::new(BinaryExpr {
                        left: e,
                        operator: Token::Op("=".to_string()),
                        right: right }))
                }
                
                e
            }

            Expr::Equality(_) => {
                let mut e = new_expr('C');
                e.parse(p)?;
                

                while if let Token::Cond(d) = p.peek(0) {
                    d.as_str() == "==" || d.as_str() == "!="
                } else {false} {
                    println!("hmm");
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr('C');
                    r.parse(p)?;

                    e = Expr::Equality(Box::new(
                        BinaryExpr {
                            left: e,
                            operator: new_operator,
                            right: r,
                        }
                    ));
                }
                e
            }

            Expr::Comparison(_) => {
                let mut e = new_expr('T');
                e.parse(p)?;

                while if let Token::Cond(d) = p.peek(0) {
                    d.as_str() == "<" || d.as_str() == ">" ||
                    d.as_str() == "<=" || d.as_str() == ">="
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr('T');
                    r.parse(p)?;

                    e = Expr::Comparison(Box::new(
                        BinaryExpr {
                            left: e,
                            operator: new_operator,
                            right: r,
                        }
                    ));
                }
                e
            }
        
            Expr::Term(_) => {
                let mut e = new_expr('S');
                e.parse(p)?;

                while if let Token::Op(d) = p.peek(0) {
                    d.as_str() == "-" || d.as_str() == "+"
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr('S');
                    r.parse(p)?;

                    e = Expr::Term(Box::new(
                        BinaryExpr {
                            left: e,
                            operator: new_operator,
                            right: r,
                        }
                    ));
                }
                e
            }
        
            Expr::Shift(_) => {
                let mut e = new_expr('U');
                e.parse(p)?;

                while if let Token::Op(d) = p.peek(0) {
                    d.as_str() == "<<" || d.as_str() == ">>"
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr('P');
                    r.parse(p)?;

                    e = Expr::Term(Box::new(
                        BinaryExpr {
                            left: e,
                            operator: new_operator,
                            right: r,
                        }
                    ));
                }
                e
            }

            Expr::Unary(_) => 'b: {
                let new_operator: Token;
                if let Token::Op(d) = p.peek(0) {
                    if d.as_str() == "-" || d.as_str() == "!" {
                        new_operator = p.peek(0);
                        p.advance();
                        
                        let mut r = new_expr('P');
                        r.parse(p)?;
                        
                        break 'b Expr::Unary(Box::new(
                            UnaryExpr {
                                operator: new_operator,
                                right: r,
                            }
                        ))
                    } 
                }

                let mut e = new_expr('P');
                e.parse(p)?
            }
        
            Expr::Primary(_) => 'b: {
                if let Token::ParenOpen = p.peek(0) {
                    let mut e = new_expr('B');
                    p.advance();
                    e.parse(p)?;

                    match p.peek(0) {
                        Token::ParenClose => {}
                        _ => return Err("Expected Closing Parentheses")

                    }

                    p.advance();

                    break 'b Expr::Primary(Box::new(
                        PrimaryExpr::Grouping(e)
                    ))

                }
                
                //not a grouping if this point is reached

                let e = match p.peek(0) {
                    Token::Lit(d) => {
                        Expr::Primary(
                            Box::new(PrimaryExpr::Literal(d))
                        )
                    }

                    Token::Id(d) => {
                        Expr::Primary(
                            Box::new(PrimaryExpr::Id(d))
                        )
                    }
                    _ => return Err("Expected Expression")
                };
                p.advance();
                e
            }
        };
        //println!("\n{}\n", self);
        return Ok(self.clone())
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, ""),
            Self::Assign(d) => write!(f, "({} = {})", d.left, d.right),
            Self::Equality(d) => write!(f, "({} {} {})", d.left, d.operator, d.right),
            Self::Comparison(d) => write!(f, "({} {} {})", d.left, d.operator, d.right),
            Self::Term(d) => write!(f, "({} {} {})", d.left, d.operator, d.right),
            Self::Shift(d) => write!(f, "({} {} {})", d.left, d.operator, d.right),
            Self::Unary(d) => write!(f, "({} {})", d.operator, d.right),
            Self::Primary(d) => {
                match *d.clone() {
                    PrimaryExpr::Grouping(v) => write!(f, "({})", v),
                    PrimaryExpr::Literal(v) => write!(f, "{}", v),
                    PrimaryExpr::Id(v) => write!(f, "{}", v), 
                }
            }
        }
    }
}