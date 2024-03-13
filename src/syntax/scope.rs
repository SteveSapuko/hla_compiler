use super::*;


#[derive(Debug)]
pub struct ScopeStack {
    pub stack: Vec<ScopeStackOp>,
    pub defined_types: Vec<UserType>,
    pub used_ids: Vec<String>
}

impl ScopeStack {
    pub fn enter_scope(&mut self) {
        self.stack.push(ScopeStackOp::EnterScope(self.used_ids.clone()));
        self.used_ids.clear();
    }

    pub fn user_type_declr(&mut self, declr: UserType) {
        self.stack.push(ScopeStackOp::UserType(declr.clone()));
        self.defined_types.push(declr.clone());
        self.used_ids.push(declr.name())
    }

    pub fn leave_scope(&mut self) {
        while !matches!(self.stack[self.stack.len() - 1], ScopeStackOp::EnterScope(_)) {
            if matches!(self.stack[self.stack.len() - 1], ScopeStackOp::UserType(_)) {
                self.defined_types.pop();
            }

            self.stack.pop();
        }
        if let ScopeStackOp::EnterScope(used) = self.stack.pop().unwrap() {
            self.used_ids = used;
        }

        /*if self.stack.len() > 2 {
            if matches!(self.stack[self.stack.len() - 2], ScopeStackOp::EnterBreakable) {
                self.stack.pop();
            }
        }*/
    }

    pub fn var_declr(&mut self, name: String, t: VarType) {
        self.stack.push(ScopeStackOp::Variable(VarData {
            name: name.clone(),
            var_type: t
        }));
        self.used_ids.push(name.clone());
    }

    pub fn get_user_enum(&self, name: String) -> Option<UserEnumDef> {
        for user_type in self.defined_types.iter().rev() {
            if let UserType::UserEnum(e) = user_type {
                if e.name == name {
                    return Some(e.clone())
                }
            }
        }

        None
    }

    pub fn enter_breakable(&mut self) {
        self.stack.push(ScopeStackOp::EnterBreakable);
    }

    pub fn inside_breakable(&mut self) -> bool {
        for element in self.stack.iter().rev() {
            if matches!(element, ScopeStackOp::EnterBreakable) {
                return true
            }
        }
        return false
    }

    pub fn fn_declr(&mut self, f: FnDeclr) {
        self.stack.push(ScopeStackOp::Func(f.clone()));
        self.used_ids.push(f.name.data())
    }

    pub fn enter_func_def(&mut self) {
        self.stack.push(ScopeStackOp::EnterFuncDef);
    }

    pub fn leave_func_def(&mut self) {
        while !matches!(self.stack[self.stack.len() - 1], ScopeStackOp::EnterFuncDef) {
            self.stack.pop();
        }
        self.stack.pop();
    }

    pub fn get_var_t(&self, target_name: String) -> Option<VarType> {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Variable(var) = element {
                if var.name == target_name {
                    return Some(var.var_type.clone())
                }
            } 
        }
        None
    }

    pub fn get_fn(&self, target_name: String) -> Option<FnDeclr> {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Func(func) = element {
                if func.name.data() == target_name {
                    return Some(func.clone())
                }
            } 
        }
        None
    }

    pub fn get_nearest_function(&self) -> Option<FnDeclr> {
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

#[derive(Debug)]
pub enum ScopeStackOp {
    EnterScope(Vec<String>),
    EnterFuncDef,
    EnterBreakable,
    UserType(UserType),
    Variable(VarData),
    Func(FnDeclr),
}