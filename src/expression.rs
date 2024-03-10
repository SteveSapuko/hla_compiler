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
    Cast(Box<Cast>),
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
#[derive(Clone, Debug)]
pub struct Cast {
    value: Expr,
    to_type: Token
}

pub fn new_expr(t: &'static str) -> Expr {
    match t {
        "Base" => Expr::Base,
        "Assign" => Expr::Assign(Box::new(BinaryExpr{
            left: new_expr("Expr"),
            operator: Token {ttype: TokenType::Op("=".to_string()), pos: 0},
            right: new_expr("Expr"),}
        )),
        "Expr" => Expr::Equality(Box::new(
            BinaryExpr {
                left: new_expr("Comp"),
                operator: Token {ttype: TokenType::Arrow, pos: 0},
                right: new_expr("Comp"),
            }
        )),
        "Comp" => Expr::Comparison(Box::new(
            BinaryExpr {
                left: new_expr("Term"),
                operator: Token {ttype: TokenType::Arrow, pos: 0},
                right: new_expr("Term"),
            }
        )),
        "Term" => Expr::Term(Box::new(
            BinaryExpr {
                left: new_expr("Shift"),
                operator: Token {ttype: TokenType::Arrow, pos: 0},
                right: new_expr("Shift"),
            }
        )),
        "Shift" => Expr::Shift(Box::new(
            BinaryExpr {
                left: new_expr("Unary"),
                operator: Token {ttype: TokenType::Arrow, pos: 0},
                right: new_expr("Primary")
            }
        )),
        "Unary" => Expr::Unary(Box::new(
            UnaryExpr {
                operator: Token {ttype: TokenType::Arrow, pos: 0},
                right: new_expr("Primary"),
            }
        )),

        "Cast" => Expr::Cast(Box::new(Cast {
            value: new_expr("Base"),
            to_type: Token {ttype: TokenType::Arrow, pos: 0},
        })),

        "Primary" => Expr::Primary(Box::new(PrimaryExpr::Literal(String::new()))),
        _ => panic!("new_expr invalid syntax -- {}", t)
    }
}

impl Expr {    
    pub fn parse(&mut self, p: &mut Parser) -> Result<Expr, &'static str> {
        *self = match self {
            Expr::Base => {
                let mut e = new_expr("Assign");
                e.parse(p)?
            }

            Expr::Assign(_) => {
                let mut e = new_expr("Expr").parse(p)?;

                if TokenType::Op("=".to_string()) == p.peek(0).ttype {
                    p.advance();
                    let right = new_expr("Base").parse(p)?;

                    e = Expr::Assign(Box::new(BinaryExpr {
                        left: e,
                        operator: Token {ttype: TokenType::Op("=".to_string()), pos: 0},
                        right: right }))
                }
                
                e
            }

            Expr::Equality(_) => {
                let mut e = new_expr("Comp");
                e.parse(p)?;
                

                while if let TokenType::Cond(d) = p.peek(0).ttype {
                    d.as_str() == "==" || d.as_str() == "!="
                } else {false} {
                    println!("hmm");
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr("Comp");
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
                let mut e = new_expr("Term");
                e.parse(p)?;

                while if let TokenType::Cond(d) = p.peek(0).ttype {
                    d.as_str() == "<" || d.as_str() == ">" ||
                    d.as_str() == "<=" || d.as_str() == ">="
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr("Term");
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
                let mut e = new_expr("Shift");
                e.parse(p)?;

                while if let TokenType::Op(d) = p.peek(0).ttype {
                    d.as_str() == "-" || d.as_str() == "+"
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr("Shift");
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
                let mut e = new_expr("Unary");
                e.parse(p)?;

                while if let TokenType::Op(d) = p.peek(0).ttype {
                    d.as_str() == "<<" || d.as_str() == ">>"
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr("Primary");
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
                if let TokenType::Op(d) = p.peek(0).ttype {
                    if d.as_str() == "-" || d.as_str() == "!" {
                        new_operator = p.peek(0);
                        p.advance();
                        
                        let mut r = new_expr("Cast");
                        r.parse(p)?;
                        
                        break 'b Expr::Unary(Box::new(
                            UnaryExpr {
                                operator: new_operator,
                                right: r,
                            }
                        ))
                    } 
                }

                let mut e = new_expr("Cast");
                e.parse(p)?
            }
        
            Expr::Cast(_) => 'b: {
                let v = new_expr("Primary").parse(p)?;

                if p.peek(0).ttype == TokenType::Op("as".to_string()) {
                    p.advance();

                    if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                        return Err("Expected Identifier for Cast Type")
                    }

                    p.advance();

                    break 'b Expr::Cast(Box::new(
                        Cast {
                            value: v,
                            to_type: p.peek(-1)
                        }
                    ))
                }

                v
            }

            Expr::Primary(_) => 'b: {
                if let TokenType::ParenOpen = p.peek(0).ttype {
                    let mut e = new_expr("Base");
                    p.advance();
                    e.parse(p)?;

                    match p.peek(0).ttype {
                        TokenType::ParenClose => {}
                        _ => return Err("Expected Closing Parentheses")

                    }

                    p.advance();

                    break 'b Expr::Primary(Box::new(
                        PrimaryExpr::Grouping(e)
                    ))

                }
                
                //not a grouping if this point is reached

                let e = match p.peek(0).ttype {
                    TokenType::Lit(d) => {
                        Expr::Primary(
                            Box::new(PrimaryExpr::Literal(d))
                        )
                    }

                    TokenType::Id(d) => {
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
            Self::Equality(d) => write!(f, "({} {} {})", d.left, d.operator.ttype, d.right),
            Self::Comparison(d) => write!(f, "({} {} {})", d.left, d.operator.ttype, d.right),
            Self::Term(d) => write!(f, "({} {} {})", d.left, d.operator.ttype, d.right),
            Self::Shift(d) => write!(f, "({} {} {})", d.left, d.operator.ttype, d.right),
            Self::Unary(d) => write!(f, "({} {})", d.operator.ttype, d.right),
            Self::Cast(d) => write!(f, "({} cast to {})", d.value, d.to_type.ttype),
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