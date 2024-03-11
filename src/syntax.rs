use super::definitions::*;
use super::statement::*;
use super::expression::*;




pub fn check_ast_syntax(ast: Vec<Statement>) -> Result<(), SyntaxErr> {
    let mut ss = ScopeStack {stack: vec![]};

    for statement in ast {
        statement.check_syntax(&mut ss)?;
    }

    Ok(())
}

//check_syntax() makes sure variables are declared before used, and that types are correct
impl Statement {
    fn check_syntax(&self, ss: &mut ScopeStack) -> Result<(), SyntaxErr> {
        match self.clone() {
            Self::Block(body) => {
                ss.enter_scope();

                for stmt in body {
                    stmt.check_syntax(ss)?;
                }

                ss.leave_scope();
            }
            
            Self::VarDeclr(declr) => {
                if let Some(value) = declr.value {
                    let value_type = value.check_syntax(ss)?;
                    ss.declr(declr.name.clone(), declr.var_type.clone());

                    if value_type != declr.var_type {
                        return Err(SyntaxErr::WrongType(declr.var_type, Token {
                            ttype: TokenType::Id(value_type),
                            pos: declr.name.pos + 3 })) //+3 so that the token reported is "="
                    }
                }

                ss.declr(declr.name, declr.var_type);
            }

            Self::LoopStmt(body) => {
                body.check_syntax(ss)?;
            }

            Self::IfStmt(stmt) => {
                stmt.cond.check_syntax(ss)?;
                stmt.true_branch.check_syntax(ss)?;
                if let Some(f) = stmt.false_branch {
                    f.check_syntax(ss)?;
                }
            }
            
            Self::WhileStmt(stmt) => {
                stmt.cond.check_syntax(ss)?;
                stmt.true_branch.check_syntax(ss)?;
            }
            
            Self::BreakStmt => {}

            Self::ExprStmt(e) => {
                e.check_syntax(ss)?;
            }

            _ => panic!("check_syntax not implemented for this statement")
        }
        
        Ok(())
    }
}

impl Expr {
    //returns expr type if Ok
    fn check_syntax(&self, ss: &mut ScopeStack) -> Result<String, SyntaxErr> {
        match self.clone() {
            Self::Assign(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(right_type, Token {
                        ttype: TokenType::Id(left_type),
                        pos: e.operator.pos }))
                }

                return Ok(right_type)
            }

            Self::Equality(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(left_type, Token {
                        ttype: TokenType::Id(right_type),
                        pos: e.operator.pos }))
                }

                return Ok(left_type)
            }

            Self::Comparison(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(left_type, Token {
                        ttype: TokenType::Id(right_type),
                        pos: e.operator.pos }))
                }

                return Ok(left_type)
            }

            Self::Term(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(left_type, Token {
                        ttype: TokenType::Id(right_type),
                        pos: e.operator.pos }))
                }

                return Ok(left_type)
            }

            Self::Shift(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(left_type, Token {
                        ttype: TokenType::Id(right_type),
                        pos: e.operator.pos }))
                }

                return Ok(left_type)
            }

            Self::Unary(e) => {
                return Ok(e.right.check_syntax(ss)?)
            }

            Self::Cast(cast) => {
                Ok(cast.to_type.data())
            }

            Self::Primary(e) => {
                match *e.clone() {
                    PrimaryExpr::Grouping(g) => {
                        return Ok(g.check_syntax(ss)?);
                    }

                    PrimaryExpr::Literal(l) => {
                        //needs to be expanded
                        
                        if l.data().as_bytes()[0] == b'-' {
                            return Ok("i8".to_string())
                        }
                        return Ok("u8".to_string())
                    }

                    PrimaryExpr::Id(id) => {
                        let id_type;
                        
                        match ss.get_var_t(id.data()) {
                            Some(t) => {id_type = t},
                            None => {return Err(SyntaxErr::Undeclared(id))}
                        }

                        return Ok(id_type)
                    }
                }
            }



            _ => panic!("check_syntax for this expr not implemented")
        }
    }

}

struct ScopeStack {
    stack: Vec<ScopeStackOp>,
}

impl ScopeStack {
    fn enter_scope(&mut self) {
        self.stack.push(ScopeStackOp::EnterScope);
    }

    fn leave_scope(&mut self) {
        while !matches!(self.stack[self.stack.len() - 1], ScopeStackOp::EnterScope) {
            self.stack.pop();
        }
        self.stack.pop();
    }

    fn declr(&mut self, tok: Token, t: String) {
        self.stack.push(ScopeStackOp::Variable(VarData {
            tok: tok,
            var_type: t
        }));
    }

    fn check_var_t(&self, checking: Token, target_type: String) -> Result<(), SyntaxErr> {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Variable(var) = element {
                if var.tok.data() == checking.data() { //check if names match
                    if var.var_type != target_type {
                        return Err(SyntaxErr::WrongType(target_type, var.tok.clone()))
                    }
                    
                    return Ok(())
                }
            } 
        }

        Err(SyntaxErr::Undeclared(checking))
    }

    fn get_var_t(&self, target_name: String) -> Option<String> {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Variable(var) = element {
                if var.tok.data() == target_name {
                    return Some(var.var_type.clone())
                }
            } 
        }
        None
    }
}

/*
Undeclared(VARIABLE USED),
WrongType(Should, Is)
*/

#[derive(Debug)]
pub enum SyntaxErr {
    Undeclared(Token),
    WrongType(String, Token)
}

enum ScopeStackOp {
    EnterScope,
    Variable(VarData)
}

struct VarData {
    tok: Token,
    var_type: String,
}