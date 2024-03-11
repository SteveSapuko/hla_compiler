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
    Ref(Ref),
    Primary(Box<PrimaryExpr>),
}

#[derive(Clone, Debug)]
pub enum PrimaryExpr {
    Grouping(Expr),
    Literal(Token),
    Id(Token),
}

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr
}
#[derive(Clone, Debug)]
pub struct Cast {
    pub value: Expr,
    pub to_type: Token
}

#[derive(Clone, Debug)]
pub struct Ref {
    pub operator: Token,
    pub right: Token,
}

pub fn new_expr(t: &'static str) -> Expr {
    match t {
        "Base" => Expr::Base,
        "Assign" => Expr::Assign(Box::new(BinaryExpr{
            left: new_expr("Equality"),
            operator: Token {ttype: TokenType::Op("=".to_string()), pos: 0},
            right: new_expr("Equality"),}
        )),
        "Equality" => Expr::Equality(Box::new(
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

        "Ref" => Expr::Ref(Ref {
            operator: Token { ttype: TokenType::Arrow, pos: 0 },
            right: Token { ttype: TokenType::Arrow, pos: 0 } }),

        "Primary" => Expr::Primary(Box::new(PrimaryExpr::Literal(Token { ttype: TokenType::Arrow, pos: 0 }))),
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
                let mut e = new_expr("Equality").parse(p)?;

                if TokenType::Op("=".to_string()) == p.peek(0).ttype {
                    p.advance();
                    let right = new_expr("Base").parse(p)?;

                    e = Expr::Assign(Box::new(BinaryExpr {
                        left: e,
                        operator: Token {ttype: TokenType::Op("=".to_string()), pos: p.peek(-1).pos},
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
                let v = new_expr("Ref").parse(p)?;

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

            Expr::Ref(_) => 'b: {
                if p.peek(0).ttype == TokenType::Op("*".to_string()) || 
                p.peek(0).ttype == TokenType::Op("&".to_string()){
                    p.advance();
                    if let TokenType::Id(_) = p.peek(0).ttype {
                        p.advance();
                        break 'b Expr::Ref(Ref {
                            operator: p.peek(-2),
                            right: p.peek(-1)
                        })
                    }
                    return Err("Expected Identifier to Reference")
                }

                new_expr("Primary").parse(p)?
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
                    TokenType::Lit(_) => {
                        Expr::Primary(
                            Box::new(PrimaryExpr::Literal(p.peek(0)))
                        )
                    }

                    TokenType::Id(_) => {
                        Expr::Primary(
                            Box::new(PrimaryExpr::Id(p.peek(0)))
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
            Self::Ref(d) => write!(f, "{} reference Op on {}", d.operator.data(), d.right.data()),
            Self::Primary(d) => {
                match *d.clone() {
                    PrimaryExpr::Grouping(v) => write!(f, "({})", v),
                    PrimaryExpr::Literal(v) => write!(f, "{}", v.ttype),
                    PrimaryExpr::Id(v) => write!(f, "{}", v.ttype), 
                }
            }
        }
    }
}