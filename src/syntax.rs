use std::f64::consts::E;
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
        ss.var_declr(Token {ttype: TokenType::Id("void".to_string()), pos: 0}, VarType::Void);

        match self.clone() {
            Self::Block(body) => {
                ss.enter_scope();

                for stmt in body {
                    stmt.check_syntax(ss)?;
                }

                ss.leave_scope();
            }
            
            Self::FnDeclr(declr) => {
                let mut param_names: Vec<Token> = vec![];
                ss.fn_declr(*declr.clone());
                ss.enter_func_def();
                
                for param in declr.params.get_param_vec() {
                    let declared_type = match VarType::from(&param.1.data(), &defined_types) {
                        Ok(t) => t,
                        Err(e) => {return Err(SyntaxErr::UnknownType(param.1.clone(), e))}
                    };

                    param_names.push(param.0.clone());
                    ss.var_declr(param.0, declared_type);
                }

                //this needs to be redone
                //checks that names are uniqe
                for name in param_names.iter() {
                    let mut count = 0;
                    for t in param_names.iter() {
                        if t.data() == name.data() {
                            count += 1;
                        }
                    }

                    if count > 1 {
                        return Err(SyntaxErr::DupParamNames(name.clone()))
                    }
                }

                
                declr.body.check_syntax(ss)?;
                ss.leave_func_def();
            }

            Self::VarDeclr(declr) => {
                let declared_type = match VarType::from(&declr.var_type.data(), &defined_types) {
                    Ok(t) => t,
                    Err(e) => {return Err(SyntaxErr::UnknownType(declr.var_type.clone(), e))}
                };
                
                if let Some(value) = declr.value {
                    let value_type = value.check_syntax(ss)?;  
                    //println!("declared type: {:?}   value: {:#?}    value type: {:?}", declared_type, value, value_type);
                    ss.var_declr(declr.name.clone(), declared_type.clone());

                    if value_type != declared_type {
                        return Err(SyntaxErr::WrongType(declared_type, value_type))
                    }
                }

                ss.var_declr(declr.name, declared_type);
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
            
            Self::BreakStmt(_) => {}

            Self::ReturnStmt(t, d) => {
                if let Some(e) = d {
                    let actual_return_type = e.check_syntax(ss)?;
                    let f = ss.get_nearest_function();

                    match f {
                        Some(declr) => {
                            let ret_type = VarType::from(&declr.ret_type.data(), &defined_types).expect("should have been handled");
                            
                            if ret_type == actual_return_type {
                                return Ok(())
                            } else {
                                return Err(SyntaxErr::WrongType(ret_type, actual_return_type))
                            }

                        }
                        None => return Err(SyntaxErr::ReturnOutsideFunc(t))
                    }
                }
            }

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

            Self::FnCall(call) => {
                let template = match ss.get_fn(call.name.data()) {
                    Some(t) => t,
                    None => return Err(SyntaxErr::Undeclared(call.name))
                };

                //check arg number
                if call.args.len() != template.params.get_param_vec().len() {
                    return Err(SyntaxErr::WrongArgN(call.name.clone()))
                }

                //check args
                for n in 0..call.args.len() {
                    let calling_type = call.args[n].check_syntax(ss)?;

                    let expected_type = template.params.get_param_vec()[n].1.clone();
                    let expected_type = match VarType::from(&expected_type.data(), &vec![]) {
                        Ok(t) => t,
                        Err(e) => return Err(SyntaxErr::UnknownType(expected_type, e))
                    };

                    if calling_type != expected_type {
                        return Err(SyntaxErr::WrongType(expected_type, calling_type))
                    }
                }

                Ok(VarType::from(&template.ret_type.data(), &vec![]).expect("template should have been checked first"))
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

#[derive(Debug)]
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

    fn var_declr(&mut self, tok: Token, t: VarType) {
        self.stack.push(ScopeStackOp::Variable(VarData {
            tok: tok,
            var_type: t
        }));
    }

    fn fn_declr(&mut self, f: FnDeclr) {
        self.stack.push(ScopeStackOp::Func(f.clone()))
    }

    fn enter_func_def(&mut self) {
        self.stack.push(ScopeStackOp::EnterFuncDef);
    }

    fn leave_func_def(&mut self) {
        while !matches!(self.stack[self.stack.len() - 1], ScopeStackOp::EnterFuncDef) {
            self.stack.pop();
        }
        self.stack.pop();
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

    fn get_fn(&self, target_name: String) -> Option<FnDeclr> {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Func(func) = element {
                if func.name.data() == target_name {
                    return Some(func.clone())
                }
            } 
        }
        None
    }

    fn get_nearest_function(&self) -> Option<FnDeclr> {
        let mut next_is_template = false;
        for element in self.stack.iter().rev() {
            if next_is_template {
                if let ScopeStackOp::Func(f) = element {
                    return Some(f.clone())
                }
                panic!("this was supposed to work")
            }
            
            if let ScopeStackOp::EnterFuncDef = element {
                next_is_template = true;
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
    NotDerefAble(Token),
    WrongArgN(Token),
    DupParamNames(Token),
    ReturnOutsideFunc(Token),
}

#[derive(Debug)]
enum ScopeStackOp {
    EnterScope,
    EnterFuncDef,
    Variable(VarData),
    Func(FnDeclr),
}



#[derive(Debug, Clone)]
struct VarData {
    tok: Token,
    var_type: VarType,
}
