use super::statement::*;
use super::expression::*;

pub fn syntaxCheck(ast: Vec<Statement>) -> Result<(), ()> {
    let mut ss = ScopeStack {stack: vec![]};

    for statement in ast {

    }

    Ok(())
}


impl Statement {
    fn checkSyntax(&self, ss: ScopeStack) -> Result<(), (CheckResult, )>
}

struct ScopeStack {
    stack: Vec<ScopeStackOp>,
}

impl ScopeStack {
    fn enter_scope(&mut self) {
        self.stack.push(ScopeStackOp::EnterScope);
    }

    fn leave_scope(&mut self) {
        while !matches!(self.stack[self.stack.len()], ScopeStackOp::EnterScope) {
            self.stack.pop();
        }
        self.stack.pop();
    }

    fn declr(&mut self, name: String, t: String) {
        self.stack.push(ScopeStackOp::Variable(VarData {
            name: name,
            v_type: t
        }));
    }

    fn check(&mut self, n: String, t: String) -> CheckResult {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Variable(v) = element {
                if v.name == n {
                    if v.v_type != t {
                        return CheckResult::WrongType
                    } else {
                        return CheckResult::Ok
                    }
                }
            } 
        }

        CheckResult::Undeclared
    }
}

enum CheckResult {
    Ok,
    Undeclared,
    WrongType
}

enum ScopeStackOp {
    EnterScope,
    Variable(VarData)
}

struct VarData {
    name: String,
    v_type: String,
}