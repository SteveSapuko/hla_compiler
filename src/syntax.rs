use super::definitions::*;
use super::statement::*;
use super::expression::*;




pub fn syntaxCheck(ast: Vec<Statement>) -> Result<(), ()> {
    let mut ss = ScopeStack {stack: vec![]};

    for statement in ast {

    }

    Ok(())
}

//check_syntax() makes sure variables are declared before used, and that types are correct




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

    fn declr(&mut self, tok: Token, t: TokenType) {
        self.stack.push(ScopeStackOp::Variable(VarData {
            tok: tok,
            v_type: t.data()
        }));
    }

    fn check_var_t(&self, n: String, t: String) -> Result<(), SyntaxErr> {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Variable(v) = element {
                if v.tok.ttype.data() == n {
                    if v.v_type != t {
                        return Err(SyntaxErr::WrongType(t, v.tok.clone()))
                    }
                    
                    return Ok(())
                }
            } 
        }

        Err(SyntaxErr::Undeclared(n))
    }

    fn get_var_t(&self, name: String) -> Option<String> {
        for element in self.stack.iter().rev() {
            if let ScopeStackOp::Variable(v) = element {
                if v.tok.ttype.data() == name {
                    return Some(v.v_type.clone())
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
enum SyntaxErr {
    Undeclared(String),
    WrongType(String, Token)
}

enum ScopeStackOp {
    EnterScope,
    Variable(VarData)
}

struct VarData {
    tok: Token,
    v_type: String,
}