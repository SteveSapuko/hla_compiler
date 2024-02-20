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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Op(d) => write!(f, "{}", d),
            Self::Cond(d) => write!(f, "{}", d),
            _ => panic!()
        }
    }
}

/* 
#[derive(Debug)]
enum BoolTerminal {
    IsTerminal(String),
    NonTerminal(Box<dyn Expression>)
}

trait Expression : std::fmt::Debug {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression>;
}



#[derive(Debug)]
struct Expr {}

impl Expr {
    pub fn new() -> Self {
        Expr {}
    }
}

impl Expression for Expr {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression> {
        let mut e = ExprEquality::new();
        return e.eval(p)
    }
}

#[derive(Debug)]
struct ExprBinary {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>
}

impl ExprBinary {
    pub fn new() -> Self {
        ExprBinary {
            left: Box::new(Expr::new()),
            operator: Token::Arrow,
            right: Box::new(Expr::new()),
        }
    }
}

impl Expression for ExprBinary {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression> {
        panic!(); //should never be reached
    }
}




#[derive(Debug)]
struct ExprEquality {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

impl ExprEquality {
    pub fn new() -> Self {
        ExprEquality {
            left: Box::new(Expr::new()),
            operator: Token::Arrow,
            right: Box::new(Expr::new()),
        }
    }
}

impl Expression for ExprEquality {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression> {
        let mut e: Box<dyn Expression> = Box::new(ExprComparison::new());
        e.eval(p);

        while if let Token::Cond(d) = p.peek(0) {
            d.as_str() == "!=" || d.as_str() == "=="
        } else {false} {
            
            self.operator = p.peek(0);
            p.advance();

            let mut r = ExprComparison::new();
            r.eval(p);

            e = Box::new(ExprBinary {left: e, operator: self.operator, right: Box::new(r)});
        }

        return e
    }
}

#[derive(Debug)]
struct ExprComparison {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

impl ExprComparison {
    pub fn new() -> Self {
        ExprComparison {
            left: Box::new(Expr::new()),
            operator: Token::Arrow,
            right: Box::new(Expr::new()) }
    }
}

impl Expression for ExprComparison {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression> {
        let mut e: Box<dyn Expression> = Box::new(ExprTerm::new());
        e.eval(p);

        while if let Token::Cond(d) = p.peek(0) {
            d.as_str() == "<" || d.as_str() == ">" || d.as_str() == "<=" || d.as_str() == ">="
            || d.as_str() == "||" || d.as_str() == "&&"
        } else {false} {
            
            self.operator = p.peek(0);
            p.advance();

            let mut r = ExprTerm::new();
            r.eval(p);

            e = Box::new(ExprBinary {left: e, operator: self.operator, right: Box::new(r)});
        }

        return e
    }
}

#[derive(Debug)]
struct ExprTerm {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

impl ExprTerm {
    pub fn new() -> Self {
        ExprTerm {
            left: Box::new(Expr::new()),
            operator: Token::Arrow,
            right: Box::new(Expr::new()) }
    }
}

impl Expression for ExprTerm {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression> {
        todo!();
    }
}

#[derive(Debug)]
struct ExprUnary {
    operator: Token,
    right: Box<dyn Expression>,
}

impl ExprUnary {
    pub fn new() -> Self {
        ExprUnary {
            operator: Token::Arrow,
            right: Box::new(Expr::new()) }
    }
}

impl Expression for ExprUnary {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression> {
        if let Token::Op(d) = p.peek(0) {
            if d.as_str() == "!" || d.as_str() == "-" {
                self.operator = p.peek(0);
                p.advance();
                self.right = Box::new(ExprPrimary{});
                self.right.eval(p);
                return Box::new(*self);
            }
        }

        let mut e = ExprPrimary{};
        e.eval(p);
        return Box::new(e)
    }
}

/*
#[derive(Debug)]
pub struct ExprGrouping {
    value: Box<dyn Expression>
}

impl Expression for ExprGrouping {
    fn eval(&mut self, p: &mut Parser) -> BoolTerminal {
        self.value.eval(p)
    }
}

*/

#[derive(Debug)]
struct ExprLiteral {
    value: String
}

impl ExprLiteral {
    pub fn new(s: String) -> Self {
        ExprLiteral {
            value: s
        }
    }
}



#[derive(Debug)]
struct ExprPrimary {}

impl Expression for ExprPrimary {
    fn eval(&mut self, p: &mut Parser) -> Box<dyn Expression> {
        
        if let Token::Lit(d) = p.peek(0) {
            
        }
        
        if let Token::ParenOpen = p.peek(0) {
            p.consume();
            
            let mut e = Expr {};
            e.eval(p);

            p.consume(); //TODO: return error if there is no closing paren
            return BoolTerminal::NonTerminal(Box::new(ExprGrouping{value: Box::new(e)}))
            //return ExprGrouping
        }

        todo!(); //if this point is reached, then the parsed code is wrong, error needs to be returned
    }
}*/