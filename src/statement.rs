use crate::parser::*;
use crate::expression::*;
use crate::definitions::*;

#[derive(Clone, Debug)]
pub enum Statement {
    Base,
    Declr,
    VarDeclr(VarDeclr),
    Stmt,
    LoopStmt(Box<Statement>), //must contain Statement::Block
    ExprStmt(Expr),
    Block(Vec<Statement>),
}

#[derive(Clone, Debug)]
pub struct VarDeclr {
    pub name: String,
    pub var_type: String,
    pub value: Option<Expr>
}

pub fn new_statement(t: char) -> Statement{
    match t {
        '0' => Statement::Base,
        'D' => Statement::Declr,
        'V' => {
            Statement::VarDeclr(VarDeclr {
                name: String::new(),
                var_type: String::new(),
                value: None })
        },
        'S' => Statement::Stmt,
        'E' => {
            Statement::ExprStmt(new_expr('B'))
        },
        'B'=> Statement::Block(vec![]),
        _ => panic!("You got this wrong")
    }
}

impl Statement {
    pub fn parse(&mut self, p: &mut Parser) -> Result<Statement, &'static str> {
        *self = match self {
            Statement::Base => new_statement('D').parse(p)?,

            Statement::Declr => 'b: {
                if p.peek(0) == Token::Key("let".to_string())  {
                    p.advance();
                    break 'b new_statement('V').parse(p)?
                }

                new_statement('S').parse(p)?
            }

            Statement::VarDeclr(_) => {
                if !matches!(p.peek(0), Token::Id(_)) {
                    return Err("Expected Identifier for Variable Name")
                }

                let name = p.peek(0).data().expect("id needs string");
                p.advance();

                if !matches!(p.peek(0), Token::Col) {
                    return Err("Expected Colon After Variable Name")
                }
                p.advance();

                if !matches!(p.peek(0), Token::Id(_)) {
                    return Err("Expected Identifier for Variable Type")
                }

                let vtype = p.peek(0).data().unwrap();
                p.advance();

                let mut value = None;
                if p.peek(0) == Token::Op("=".to_string()) {
                    p.advance();
                    let temp = new_expr('B').parse(p)?;
                    value = Some(temp);
                }

                if !matches!(p.peek(0), Token::SemiCol) {
                    return Err("Expected Semicolon after Declaration")
                }
                
                p.advance();

                Statement::VarDeclr(VarDeclr {
                    name: name,
                    var_type: vtype,
                    value: value })
                }

            Statement::Stmt => 'b: {
                if matches!(p.peek(0), Token::CurlyOpen) {
                    p.advance();
                    break 'b new_statement('B').parse(p)?
                }

                //if this point is reached, statement is ExprStmt

                let e = new_expr('B').parse(p)?;
                if !matches!(p.peek(0), Token::SemiCol) {
                    return Err("Expected Semicolon after Expression Statement")
                }
                
                p.advance();
                Statement::ExprStmt(e)
            }

            Statement::Block(v) => {
                while p.peek(0) != Token::CurlyClose {
                    if p.peek(0) == Token::EOF {return Err("Expected Closing Curly Bracket")}
                    
                    let s = new_statement('0').parse(p)?;
                    v.push(s);
                }
                
                p.advance();
                self.clone()
            }

            _ => panic!("you messed up")

        };

        return Ok(self.clone())
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Statement::Base => write!(f, ""),
            Statement::Declr => write!(f, ""),
            Statement::Stmt => write!(f, ""),
            Statement::ExprStmt(s) => write!(f, "Expression Stmt: {};", s),
            Statement::Block(b) => {
                write!(f, "Block:\n")?;
                for s in b {
                    write!(f, "{}\n", s)?;
                }
                write!(f, "\n")?;
                Ok(())
            },
            Statement::VarDeclr(d) => write!(f, "declare {} type: {} value: {}", d.name, d.var_type, d.value.unwrap_or(new_expr('B'))),
            _ => panic!("implement stmt display")
        }
    }
}