use crate::parser::*;
use crate::expression::*;
use crate::definitions::*;

#[derive(Clone, Debug)]
pub enum Statement {
    Base,
    Declr,
    FnDeclr(Box<FnDeclr>),
    Parameters(Vec<(Token, Token)>), //(name, type)
    VarDeclr(VarDeclr),
    Stmt,
    LoopStmt(Box<Statement>), //must contain Statement::Block
    IfStmt(Box<CondStmt>),
    WhileStmt(Box<CondStmt>),
    BreakStmt(Token),
    ReturnStmt(Token, Option<Expr>),
    ExprStmt(Expr),
    Block(Vec<Statement>),
}

#[derive(Clone, Debug)]
pub struct VarDeclr {
    pub name: Token,
    pub var_type: Token,
    pub value: Option<Expr>
}

#[derive(Clone, Debug)]
pub struct FnDeclr {
    pub name: Token,
    pub params: Statement, //Statement::parameters
    pub ret_type: Token,
    pub body: Statement,
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
        "FnDeclr" => {
            Statement::FnDeclr(Box::new(FnDeclr {
                name: Token { ttype: TokenType::Arrow, pos: 0 },
                params: new_statement("Base"),
                ret_type: Token { ttype: TokenType::Arrow, pos: 0 },
                body: new_statement("Base") }))
        }
        "Params" => Statement::Parameters(vec![]),
        
        "VarDeclr" => {
            Statement::VarDeclr(VarDeclr {
                name: Token { ttype: TokenType::Arrow, pos: 0 },
                var_type: Token { ttype: TokenType::Arrow, pos: 0 },
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
        
        "BreakStmt" => {Statement::BreakStmt(Token { ttype: TokenType::Arrow, pos: 0 },)},
        
        "ReturnStmt" => {Statement::ReturnStmt(Token { ttype: TokenType::Arrow, pos: 0 }, None)}
        _ => panic!("Need to implement new_statement")
    }
}

impl Statement {
    pub fn parse(&mut self, p: &mut Parser) -> Result<Statement, &'static str> {
        *self = match self {
            Statement::Base => new_statement("Declr").parse(p)?,

            Statement::Declr => 'b: {
                if p.peek(0).ttype == TokenType::Key("let".to_string())  {
                    p.advance();
                    break 'b new_statement("VarDeclr").parse(p)?
                }

                if p.peek(0).ttype == TokenType::Key("fn".to_string()) {
                    p.advance();
                    break 'b new_statement("FnDeclr").parse(p)?
                }

                new_statement("Stmt").parse(p)?
            }

            Statement::FnDeclr(_) => {
                if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                    return Err("Expected Identifier for Function Name")
                }
                let fn_name = p.peek(0);
                p.advance();

                if !matches!(p.peek(0).ttype, TokenType::ParenOpen) {
                    return Err("Expected Opening Parentheses after Function Name")
                }
                p.advance();

                let mut params = new_statement("Params");

                if p.peek(0).ttype != TokenType::ParenClose {
                    params.parse(p)?;
                }

                if p.peek(0).ttype != TokenType::ParenClose {
                    return Err("Expected Closing Parentheses after Function Declaration")
                }
                p.advance();

                if p.peek(0).ttype != TokenType::Arrow {
                    return Err("Expected Arrow to Denote Function Type")
                }
                p.advance();

                if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                    return Err("Expected Identifier for Return Type")
                }
                let ret_type = p.peek(0);
                p.advance();

                if p.peek(0).ttype != TokenType::CurlyOpen {
                    return Err("Expected Opening Curly Brace for Function Body")
                }
                p.advance();

                let body = new_statement("Block").parse(p)?;

                Statement::FnDeclr(Box::new(FnDeclr {
                    name: fn_name,
                    params: params,
                    ret_type: ret_type,
                    body: body }
                ))
            }

            Statement::Parameters(_) => {
                let mut param_vec: Vec<(Token, Token)> = vec![];

                if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                    return Err("Expected Identifier for Parameter Name")
                }
                let param_name = p.peek(0);
                p.advance();

                if !matches!(p.peek(0).ttype, TokenType::Col) {
                    return Err("Expected Colon after Parameter Name")
                }
                p.advance();

                if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                    return Err("Expected Identifier for Parameter Type")
                }
                let param_type = p.peek(0);
                param_vec.push((param_name, param_type));
                
                p.advance();

                while p.peek(0).ttype == TokenType::Comma {
                    p.advance();

                    if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                        return Err("Expected Identifier for Parameter Name")
                    }
                    let param_name = p.peek(0);
                    p.advance();
    
                    if !matches!(p.peek(0).ttype, TokenType::Col) {
                        return Err("Expected Colon after Parameter Name")
                    }
                    p.advance();
    
                    if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                        return Err("Expected Identifier for Parameter Type")
                    }
                    let param_type = p.peek(0);
                    param_vec.push((param_name, param_type));
                    
                    p.advance();   
                }

                Statement::Parameters(param_vec)
            }

            Statement::VarDeclr(_) => {
                if !matches!(p.peek(0).ttype, TokenType::Id(_)) {
                    return Err("Expected Identifier for Variable Name")
                }

                let name = p.peek(0);
                p.advance();

                if !matches!(p.peek(0).ttype, TokenType::Col) {
                    return Err("Expected Colon After Variable Name")
                }
                p.advance();

                let vtype;

                if matches!(p.peek(0).ttype, TokenType::Id(_)) {
                    vtype = p.peek(0);
                } else {
                    return Err("Expected Identifier as Type for Declaration")
                }

                p.advance();

                let mut value = None;
                if p.peek(0).ttype == TokenType::Op("=".to_string()) {
                    p.advance();
                    let temp = new_expr("Base").parse(p)?;
                    value = Some(temp);
                }

                if !matches!(p.peek(0).ttype, TokenType::SemiCol) {
                    return Err("Expected Semicolon after Declaration")
                }
                
                p.advance();

                Statement::VarDeclr(VarDeclr {
                    name: name,
                    var_type: vtype,
                    value: value })
                }

            Statement::Stmt => 'b: {
                if matches!(p.peek(0).ttype, TokenType::CurlyOpen) {
                    p.advance();
                    break 'b new_statement("Block").parse(p)?
                }

                if p.peek(0).ttype == TokenType::Key("loop".to_string()) {
                    p.advance();
                    break 'b new_statement("LoopStmt").parse(p)?
                }

                if p.peek(0).ttype == TokenType::Key("if".to_string()) {
                    p.advance();
                    break 'b new_statement("IfStmt").parse(p)?
                }

                if p.peek(0).ttype == TokenType::Key("while".to_string()) {
                    p.advance();
                    break 'b new_statement("WhileStmt").parse(p)?
                }

                if p.peek(0).ttype == TokenType::Key("break".to_string()) {
                    p.advance();

                    if p.peek(0).ttype == TokenType::CurlyClose {
                        break 'b new_statement("BreakStmt")
                    } else {
                        return Err("Expected Closing Brace after Break")
                    }
                    
                }

                if p.peek(0).ttype == TokenType::Key("return".to_string()) {
                    p.advance();

                    break 'b new_statement("ReturnStmt").parse(p)?
                }

                //if this point is reached, statement is ExprStmt
                let e = new_expr("Base").parse(p)?;
                if !matches!(p.peek(0).ttype, TokenType::SemiCol) {
                    return Err("Expected Semicolon after Expression Statement")
                }
                
                p.advance();
                Statement::ExprStmt(e)
            }

            Statement::ReturnStmt(r, _) => {
                let mut return_value = None;
                
                if p.peek(0).ttype != TokenType::SemiCol {
                    let value = new_expr("Base").parse(p)?;
                    return_value = Some(value);
                } else {
                    p.advance();
                }

                Statement::ReturnStmt(r.clone(), return_value)
            }

            Statement::LoopStmt(_) => {
                if p.peek(0).ttype != TokenType::CurlyOpen {
                    return Err("Expected Block after Loop Statement")
                }

                p.advance();
                let s = new_statement("Block").parse(p)?;

                Statement::LoopStmt(Box::new(s))
            }

            Statement::IfStmt(_) => 'b: {
                let cond = new_expr("Base").parse(p)?;

                if p.peek(0).ttype != TokenType::CurlyOpen {
                    return Err("Expected Block after If Statement")
                }
                
                p.advance();
                let true_b = new_statement("Block").parse(p)?;

                if p.peek(0).ttype == TokenType::Key("else".to_string()) {
                    p.advance();

                    if p.peek(0).ttype != TokenType::CurlyOpen {return Err("Expected Block After Else Statement")}

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
                
                if p.peek(0).ttype != TokenType::CurlyOpen {
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
                while p.peek(0).ttype != TokenType::CurlyClose {
                    if p.peek(0).ttype == TokenType::EOF {return Err("Expected Closing Curly Bracket")}
                    
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
            Statement::VarDeclr(d) => write!(f, "declare {} type: {} value: {}", d.name.data(), d.var_type.data(), d.value.unwrap_or(new_expr("Base"))),
            Statement::LoopStmt(d) => write!(f, "Loop {}", *d),
            Statement::IfStmt(d) => write!(f, "If {} then {}\nelse {}", d.cond, d.true_branch, d.false_branch.unwrap_or(new_statement("Base"))),
            Statement::WhileStmt(d) => write!(f, "While {} do {}", d.cond, d.true_branch),
            Statement::BreakStmt(d) => write!(f, "Break"),
            Statement::FnDeclr(d) => write!(f, "define function: {}   params: {}  \nret type: {}   body: {}", d.name.ttype, d.params, d.ret_type.ttype, d.body),
            Statement::ReturnStmt(t, d) => {
                match d {
                    Some(t) => write!(f, "return {}", t),
                    None => write!(f,"return")
                }
            }
            Statement::Parameters(d) => {
                for p in d {
                    write!(f, "\nparam name: {}   param type: {}", p.0.ttype, p.1.ttype)?;
                }
                Ok(())
            }
            _ => panic!("implement stmt display")
        }
    }
}