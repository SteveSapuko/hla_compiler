mod scope;
mod types;

use crate::definitions::*;
use crate::statement::*;
use crate::expression::*;

use scope::*;
use types::*;

pub fn check_ast_syntax(ast: Vec<Statement>) -> Result<(), SyntaxErr> {
    let mut ss = ScopeStack {
        stack: vec![],
        defined_types: vec![],
        used_ids: vec![]};


    define_types_in_scope(&ast, &mut ss)?;
    for statement in ast {
        statement.check_syntax(&mut ss)?;
    }

    Ok(())
}

fn define_types_in_scope(ast: &Vec<Statement>, ss: &mut ScopeStack) -> Result<(), SyntaxErr> {
    let mut types_to_recheck: Vec<UserStructDef> = vec![];

    //to allow for pointers to the struct
    let mut temp_defined_types: Vec<UserType> = ss.defined_types.clone();
    
    for stmt in ast {
        match stmt {
            Statement::StructDeclr(declr) => {
                let struct_name = declr.name.clone();

                if ss.used_ids.contains(&struct_name.data()) {
                    return Err(SyntaxErr::AlreadyDefined(struct_name))
                }

                let mut params: Vec<(String, FieldType)> = vec![];
                let mut needs_rechecking = false;

                for param in declr.params.get_param_vec() {
                    let param_type = match VarType::from(&param.1.data(), &ss.defined_types) {
                        Ok(t) => FieldType::Defined(t),
                        Err(_) => {needs_rechecking = true; FieldType::Undefined(param.1)}
                    };

                    params.push((param.0.data(), param_type));
                }

                let current_definition = UserStructDef {
                    name: struct_name.data(),
                    fields: params };
                
                if needs_rechecking {
                    types_to_recheck.push(current_definition.clone());
                    temp_defined_types.push(UserType::UserStruct(current_definition));
                } else {
                    ss.user_type_declr(UserType::UserStruct(current_definition));
                }
            }

            Statement::EnumDeclr(declr) => {
                //needs to be expanded
                if ss.used_ids.contains(&declr.name.data()) {
                    return Err(SyntaxErr::AlreadyDefined(declr.name.clone()))
                }

                let mut e_variants: Vec<String> = vec![];

                if let Statement::Variant(variants) = declr.variants.clone() {
                    for v in variants {
                        e_variants.push(v.data());
                    }
                }

                let user_enum = UserType::UserEnum(UserEnumDef {
                    name: declr.name.data(),
                    variants: e_variants });

                ss.user_type_declr(user_enum.clone());
                temp_defined_types.push(user_enum);
            }

            _ => {}
        }
    }

    for checking_struct in types_to_recheck.iter_mut() {
        for field in checking_struct.fields.iter_mut() {
            if let FieldType::Undefined(field_type) = field.1.clone() {
                if field_type.data() == checking_struct.name {
                    return Err(SyntaxErr::RecursiveStruct(field_type))
                }

                let resolved_field_type = match VarType::from(&field_type.data(), &temp_defined_types) {
                    Ok(t) => t,
                    Err(e) => {return Err(SyntaxErr::UnknownType(field_type, e))}
                };

                field.1 = FieldType::Defined(resolved_field_type);
            }
        }

        ss.user_type_declr(UserType::UserStruct(checking_struct.clone()));
    }

    Ok(())
}

//check_syntax() makes sure variables are declared before used, and that types are correct
impl Statement {
    fn check_syntax(&self, ss: &mut ScopeStack) -> Result<(), SyntaxErr> {
        match self.clone() {
            Self::Block(body) => {
                ss.enter_scope();
                define_types_in_scope(&body, ss)?;

                for stmt in body {
                    stmt.check_syntax(ss)?;
                }

                ss.leave_scope();
            }
            
            Self::FnDeclr(declr) => {
                if RESERVED_IDS.contains(&declr.name.data().as_str()) {
                    return Err(SyntaxErr::ReservedID(declr.name))
                }

                if ss.used_ids.contains(&declr.name.data()) {
                    return Err(SyntaxErr::AlreadyDefined(declr.name))
                }
                
                let mut param_names: Vec<Token> = vec![];
                ss.fn_declr(*declr.clone());
                ss.enter_func_def();
                
                for param in declr.params.get_param_vec() {
                    let declared_type = match VarType::from(&param.1.data(), &ss.defined_types) {
                        Ok(t) => t,
                        Err(e) => {return Err(SyntaxErr::UnknownType(param.1.clone(), e))}
                    };

                    param_names.push(param.0.clone());
                    ss.var_declr(param.0.data(), declared_type);
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
                if RESERVED_IDS.contains(&declr.name.data().as_str()) {
                    return Err(SyntaxErr::ReservedID(declr.name))
                }

                if ss.used_ids.contains(&declr.name.data()) {
                    return Err(SyntaxErr::AlreadyDefined(declr.name))
                }
                
                let declared_type = match VarType::from(&declr.var_type.data(), &ss.defined_types) {
                    Ok(t) => t,
                    Err(e) => {return Err(SyntaxErr::UnknownType(declr.var_type.clone(), e))}
                };
                
                if let Some(value) = declr.value {
                    let value_type = value.check_syntax(ss)?;  
                    //println!("declared type: {:?}   value: {:#?}    value type: {:?}", declared_type, value, value_type);
                    ss.var_declr(declr.name.data(), declared_type.clone());

                    if value_type != declared_type{
                        return Err(SyntaxErr::WrongType(declared_type, value_type))
                    }
                }

                ss.var_declr(declr.name.data(), declared_type);
            }

            Self::StructDeclr(_) => {}
            
            Self::EnumDeclr(_) => {}

            Self::LoopStmt(body) => {
                ss.enter_breakable();
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
                ss.enter_breakable();
                stmt.true_branch.check_syntax(ss)?;
            }
            
            Self::BreakStmt(t) => {
                if ss.inside_breakable() {
                    return Ok(())
                }

                return Err(SyntaxErr::BreakOutsideLoop(t.clone()))
            }

            Self::ReturnStmt(t, d) => {
                    let actual_return_type = d.check_syntax(ss)?;
                    
                    let f = ss.get_nearest_function();

                    match f {
                        Some(declr) => {
                            let ret_type = VarType::from(&declr.ret_type.data(), &ss.defined_types).expect("should have been handled");
                            
                            if ret_type == actual_return_type {
                                return Ok(())
                            } else {
                                return Err(SyntaxErr::WrongType(ret_type, actual_return_type))
                            }
                        }
                        None => return Err(SyntaxErr::ReturnOutsideFunc(t))
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
    ///returns expr type if Ok
    ///ss: ScopeStack
    /// dt: Defined Types
    fn check_syntax(&self, ss: &mut ScopeStack) -> Result<VarType, SyntaxErr> {
        match self.clone() {
            Self::Assign(e) => {
                let left_type = e.left.check_syntax(ss)?;
                let right_type = e.right.check_syntax(ss)?;

                if left_type != right_type{
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

                match VarType::from(&cast.to_type.data(), &ss.defined_types) {
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
                        if id.data().as_str() == "void" {
                            return Ok(VarType::Void)
                        }
                        
                        let id_type;
                        
                        match ss.get_var_t(id.data()) {
                            Some(t) => {id_type = t},
                            None => {return Err(SyntaxErr::Undeclared(id))}
                        }

                        return Ok(id_type)
                    }
                
                    PrimaryExpr::StructField(s_name, s_field) => {
                        let var_type = match ss.get_var_t(s_name.data()) {
                            Some(t) => t,
                            None => {return Err(SyntaxErr::Undeclared(s_name))}
                        };

                        if let VarType::UserStruct(user_s) = var_type {
                            let field_type = match user_s.get_field_type(s_field.data()){
                                Some(t) => t,
                                None => {
                                    return Err(SyntaxErr::UnknownField(s_field))
                                }
                            };

                            return Ok(field_type.unwrap())
                        }
                        
                        return Err(SyntaxErr::NotAStruct(s_name))
                    }

                    PrimaryExpr::EnumVariant(e_name, e_variant) => {
                        let user_enum = match ss.get_user_enum(e_name.data()) {
                            Some(t) => t,
                            None => return Err(SyntaxErr::UnknownType(e_name, "Undefined Enum"))
                        };

                        if !user_enum.check_variant(e_variant.data()) {
                            return Err(SyntaxErr::UnknownVariant(e_variant))
                        }

                        return Ok(VarType::UserEnum(user_enum))
                    }
                }
            }

            _ => panic!("check_syntax for this expr not implemented")
        }
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
    BreakOutsideLoop(Token),
    ReservedID(Token),
    AlreadyDefined(Token),
    UnknownVariant(Token),
    UnknownField(Token),
    NotAStruct(Token),
    RecursiveStruct(Token)
}

