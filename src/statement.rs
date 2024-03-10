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
    IfStmt(Box<CondStmt>),
    WhileStmt(Box<CondStmt>),
    BreakStmt,
    ExprStmt(Expr),
    Block(Vec<Statement>),
}

#[derive(Clone, Debug)]
pub struct VarDeclr {
    pub name: String,
    pub var_type: String,
    pub value: Option<Expr>
}

#[derive(Clone, Debug)]
pub struct CondStmt {
    pub cond: Expr,
    pub true_branch: Statement,
    pub false_branch: Option<Statement>,
}

pub fn new_statement(t: &'static str) -> Statement{
    match t {
        "Base" => Statement::Base,
        "Declr" => Statement::Declr,
        "VarDeclr" => {
            Statement::VarDeclr(VarDeclr {
                name: String::new(),
                var_type: String::new(),
                value: None })
        },
        "Stmt" => Statement::Stmt,
        "ExprStmt" => {
            Statement::ExprStmt(new_expr("Base"))
        },
        "Block"=> Statement::Block(vec![]),
        "LoopStmt" => Statement::LoopStmt(Box::new(new_statement("Block"))),
        "IfStmt" => Statement::IfStmt(Box::new(CondStmt {
            cond: new_expr("Base"),
            true_branch: new_statement("Block"),
            false_branch: None,
        })),
        "WhileStmt" => Statement::WhileStmt(Box::new(CondStmt {
            cond: new_expr("Base"),
            true_branch: new_statement("Block"),
            false_branch: None })),
        "BreakStmt" => {Statement::BreakStmt}
        _ => panic!("Need to implement new_statement")
    }
}

impl Statement {
    pub fn parse(&mut self, p: &mut Parser) -> Result<Statement, &'static str> {
        *self = match self {
            Statement::Base => new_statement("Declr").parse(p)?,

            Statement::Declr => 'b: {
                if p.peek(0) == Token::Key("let".to_string())  {
                    p.advance();
                    break 'b new_statement("VarDeclr").parse(p)?
                }

                new_statement("Stmt").parse(p)?
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
                    let temp = new_expr("Base").parse(p)?;
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
                    break 'b new_statement("Block").parse(p)?
                }

                if p.peek(0) == Token::Key("loop".to_string()) {
                    p.advance();
                    break 'b new_statement("LoopStmt").parse(p)?
                }

                if p.peek(0) == Token::Key("if".to_string()) {
                    p.advance();
                    break 'b new_statement("IfStmt").parse(p)?
                }

                if p.peek(0) == Token::Key("while".to_string()) {
                    p.advance();
                    break 'b new_statement("WhileStmt").parse(p)?
                }

                if p.peek(0) == Token::Key("break".to_string()) {
                    p.advance();
                    break 'b new_statement("BreakStmt")
                }

                //if this point is reached, statement is ExprStmt
                let e = new_expr("Base").parse(p)?;
                if !matches!(p.peek(0), Token::SemiCol) {
                    return Err("Expected Semicolon after Expression Statement")
                }
                
                p.advance();
                Statement::ExprStmt(e)
            }

            Statement::LoopStmt(_) => {
                if p.peek(0) != Token::CurlyOpen {
                    return Err("Expected Block after Loop Statement")
                }

                p.advance();
                let s = new_statement("Block").parse(p)?;

                Statement::LoopStmt(Box::new(s))
            }

            Statement::IfStmt(_) => 'b: {
                let cond = new_expr("Base").parse(p)?;

                if p.peek(0) != Token::CurlyOpen {
                    return Err("Expected Block after If Statement")
                }
                
                p.advance();
                let true_b = new_statement("Block").parse(p)?;

                if p.peek(0) == Token::Key("else".to_string()) {
                    p.advance();

                    if p.peek(0) != Token::CurlyOpen {return Err("Expected Block After Else Statement")}

                    p.advance();
                    let false_b = new_statement("Block").parse(p)?;
                    break 'b Statement::IfStmt(Box::new( CondStmt {
                        cond: cond,
                        true_branch: true_b,
                        false_branch: Some(false_b) }
                    ))
                }


                Statement::IfStmt(Box::new(CondStmt {
                    cond: cond,
                    true_branch: true_b,
                    false_branch: None }))
            }

            Statement::WhileStmt(_) => {
                let cond = new_expr("Base").parse(p)?;
                
                if p.peek(0) != Token::CurlyOpen {
                    return Err("Expected Curly Brace after While Statement")
                }

                p.advance();

                let body = new_statement("Block").parse(p)?;

                Statement::WhileStmt(Box::new(CondStmt {
                    cond: cond,
                    true_branch: body,
                    false_branch: None }))
            }

            Statement::Block(v) => {
                while p.peek(0) != Token::CurlyClose {
                    if p.peek(0) == Token::EOF {return Err("Expected Closing Curly Bracket")}
                    
                    let s = new_statement("Base").parse(p)?;
                    v.push(s);
                }
                
                p.advance();
                self.clone()
            }

            _ => panic!("Statement not implemented")

        };

        return Ok(self.clone())
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone() {
            Statement::Base => Ok(()),
            Statement::Declr => Ok(()),
            Statement::Stmt => Ok(()),
            Statement::ExprStmt(s) => write!(f, "Expression Stmt: {};", s),
            Statement::Block(b) => {
                write!(f, "Block:")?;
                for s in b {
                    write!(f, "\n{}", s)?;
                }
                //write!(f, "\n")?;
                Ok(())
            },
            Statement::VarDeclr(d) => write!(f, "declare {} type: {} value: {}", d.name, d.var_type, d.value.unwrap_or(new_expr("Base"))),
            Statement::LoopStmt(d) => write!(f, "Loop {}", *d),
            Statement::IfStmt(d) => write!(f, "If {} then {}\nelse {}", d.cond, d.true_branch, d.false_branch.unwrap_or(new_statement("Base"))),
            Statement::WhileStmt(d) => write!(f, "While {} do {}", d.cond, d.true_branch),
            Statement::BreakStmt => write!(f, "Break"),
            _ => panic!("implement stmt display")
        }
    }
}