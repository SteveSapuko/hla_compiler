use std::vec;

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
        let defined_types: Vec<UserStruct> = vec![];

        match self.clone() {
            Self::Block(body) => {
                ss.enter_scope();

                for stmt in body {
                    stmt.check_syntax(ss)?;
                }

                ss.leave_scope();
            }
            
            Self::VarDeclr(declr) => {
                let declared_type = match VarType::from(&declr.var_type.data(), &defined_types) {
                    Ok(t) => t,
                    Err(e) => {return Err(SyntaxErr::UnknownType(declr.var_type.clone(), e))}
                };
                
                if let Some(value) = declr.value {
                    let value_type = value.check_syntax(ss)?;  
                    println!("declared type: {:?}   value: {}    value type: {:?}", declared_type, value, value_type);
                    ss.declr(declr.name.clone(), declared_type.clone());

                    if value_type != declared_type {
                        return Err(SyntaxErr::WrongType(declared_type, value_type))
                    }
                }

                ss.declr(declr.name, declared_type);
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
    fn check_syntax(&self, ss: &mut ScopeStack) -> Result<VarType, SyntaxErr> {
        match self.clone() {
            Self::Assign(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(right_type, left_type))
                }

                return Ok(right_type)
            }

            Self::Equality(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(right_type, left_type))
                }

                return Ok(left_type)
            }

            Self::Comparison(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(right_type, left_type))
                }

                return Ok(left_type)
            }

            Self::Term(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(right_type, left_type))
                }

                return Ok(left_type)
            }

            Self::Shift(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type {
                    return Err(SyntaxErr::WrongType(right_type, left_type))
                }

                return Ok(left_type)
            }

            Self::Unary(e) => {
                return Ok(e.right.check_syntax(ss)?)
            }

            Self::Cast(cast) => {
                // no casting to a struct
                let defined_types: Vec<UserStruct> = vec![];

                match VarType::from(&cast.to_type.data(), &defined_types) {
                    Ok(t) => Ok(t),
                    Err(e) => return Err(SyntaxErr::UnknownType(cast.to_type, e))
                }

            }

            Self::Ref(r) => {
                if r.operator.data().as_str() == "*" {
                    //code is incorrect, need to add pointer as a type that points to a type
                    let right_type = match ss.get_var_t(r.right.data()) {
                        Some(t) => t,
                        None => return Err(SyntaxErr::Undeclared(r.right))
                    };

                    if let VarType::Pointer(p) = right_type {
                        return Ok(*p.clone())
                    }

                    return Err(SyntaxErr::NotDerefAble(r.right))
                }

                if r.operator.data().as_str() == "&" {
                    println!("got here");
                    let right_type = match ss.get_var_t(r.right.data()) {
                        Some(t) => t,
                        None => return Err(SyntaxErr::Undeclared(r.right))
                    };

                    return Ok(VarType::Pointer(Box::new(right_type)))
                }

                panic!("ref is messed up")
            }

            Self::Primary(e) => {
                match *e.clone() {
                    PrimaryExpr::Grouping(g) => {
                        return Ok(g.check_syntax(ss)?);
                    }

                    PrimaryExpr::Literal(l) => {
                        //needs to be expanded
                        let mut signed = false;
                        if l.data().as_bytes()[0] == b'-' {
                            signed = true;
                        }

                        if !signed {
                            let number = match l.data().parse::<u64>() {
                                Ok(t) => t,
                                Err(_) => return Err(SyntaxErr::LiteralErr(l.clone()))
                            };

                            if number <= u64::MAX {
                                if number <= u32::MAX.into() {
                                    if number <= u16::MAX.into() {
                                        if number <= u8::MAX.into() {
                                            return Ok(VarType::U8)
                                        }
                                        
                                        return Ok(VarType::U16)
                                    }

                                    return Ok(VarType::U32)
                                }

                                return Ok(VarType::U64)
                            }
                        }

                        let number = match l.data().parse::<i64>() {
                            Ok(t) => t,
                            Err(_) => return Err(SyntaxErr::LiteralErr(l.clone()))
                        };

                        if number <= i64::MAX && number >= i64::MIN {
                            if number <= i32::MAX.into() && number >= i32::MIN.into() {
                                if number <= i16::MAX.into() && number >= i16::MIN.into() {
                                    if number <= i8::MAX.into() && number >= i8::MIN.into() {
                                        return Ok(VarType::I8)
                                    }
                                    
                                    return Ok(VarType::I16)
                                }

                                return Ok(VarType::I32)
                            }

                            return Ok(VarType::I64)
                        }

                        panic!("Number literal error")
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

    fn declr(&mut self, tok: Token, t: VarType) {
        self.stack.push(ScopeStackOp::Variable(VarData {
            tok: tok,
            var_type: t
        }));
    }

    fn get_var_t(&self, target_name: String) -> Option<VarType> {
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
Undeclared(VARIABLE_USED),
WrongType(Should, Is)
*/

#[derive(Debug, Clone)]
pub enum SyntaxErr {
    Undeclared(Token),
    WrongType(VarType, VarType),
    UnknownType(Token, &'static str),
    LiteralErr(Token),
    NotDerefAble(Token)
}

enum ScopeStackOp {
    EnterScope,
    Variable(VarData)
}

#[derive(Debug, Clone)]
struct VarData {
    tok: Token,
    var_type: VarType,
}
