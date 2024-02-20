use crate::definitions::*;
use crate::parser::*;


#[derive(Clone, Debug)]
pub enum Expr {
    Base,
    Equality(Box<BinaryExpr>),
    Comparison(Box<BinaryExpr>),
    Term(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Primary(Box<PrimaryExpr>),
}

#[derive(Clone, Debug)]
pub enum PrimaryExpr {
    Grouping(Expr),
    Literal(String),
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
                left: new_expr('U'),
                operator: Token::Arrow,
                right: new_expr('U'),
            }
        )),
        'U' => Expr::Unary(Box::new(
            UnaryExpr {
                operator: Token::Arrow,
                right: new_expr('P'),
            }
        )),
        'P' => Expr::Primary(Box::new(PrimaryExpr::Literal(String::new()))),
        _ => panic!()
    }
}

impl Expr {    
    pub fn eval(&mut self, p: &mut Parser) -> Expr {
        *self = match self {
            Expr::Base => {
                let mut e = new_expr('E');
                e.eval(p)
            }
            
            Expr::Equality(_) => {
                let mut e = new_expr('C');
                e.eval(p);
                

                while if let Token::Cond(d) = p.peek(0) {
                    d.as_str() == "==" || d.as_str() == "!="
                } else {false} {
                    println!("hmm");
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr('C');
                    r.eval(p);

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
                e.eval(p);

                while if let Token::Cond(d) = p.peek(0) {
                    d.as_str() == "<" || d.as_str() == ">" ||
                    d.as_str() == "<=" || d.as_str() == ">="
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr('T');
                    r.eval(p);

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
                let mut e = new_expr('U');
                e.eval(p);

                while if let Token::Op(d) = p.peek(0) {
                    d.as_str() == "-" || d.as_str() == "+"
                } else {false} {
                    
                    let new_operator = p.peek(0);
                    p.advance();

                    let mut r = new_expr('U');
                    r.eval(p);

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
                        r.eval(p);
                        
                        break 'b Expr::Unary(Box::new(
                            UnaryExpr {
                                operator: new_operator,
                                right: r,
                            }
                        ))
                    } 
                }

                let mut e = new_expr('P');
                e.eval(p)
            }
        
            Expr::Primary(_) => 'b: {
                if let Token::ParenOpen = p.peek(0) {
                    let mut e = new_expr('B');
                    p.advance();
                    e.eval(p);

                    assert_eq!(p.peek(0), Token::ParenClose); //TODO return error
                    p.advance();

                    break 'b Expr::Primary(Box::new(
                        PrimaryExpr::Grouping(e)
                    ))

                }

                let e = Expr::Primary(
                    Box::new(
                        PrimaryExpr::Literal(match p.peek(0) {
                            Token::Id(d) => d,
                            Token::Lit(d) => d,
                            _ => panic!("{}", p.peek(0)),
                        })
                    )
                );
                p.advance();
                e
            }
        };
        //println!("\n{}\n", self);
        return self.clone()
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "EXPR"),
            Self::Equality(d) => write!(f, "({} {} {})", d.left, d.operator, d.right),
            Self::Comparison(d) => write!(f, "({} {} {})", d.left, d.operator, d.right),
            Self::Term(d) => write!(f, "({} {} {})", d.left, d.operator, d.right),
            Self::Unary(d) => write!(f, "({} {})", d.operator, d.right),
            Self::Primary(d) => {
                match *d.clone() {
                    PrimaryExpr::Grouping(v) => write!(f, "({})", v),
                    PrimaryExpr::Literal(v) => write!(f, "{}", v)
                }
            }
        }
    }
}