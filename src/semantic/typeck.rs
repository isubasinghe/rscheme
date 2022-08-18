use crate::common::Spanned;
use std::sync::Arc;
use std::rc::Rc;

use crate::parser::ast;

#[derive(Clone, Debug)]
struct State {
    existentials: usize,
}


impl State {
    fn initial() -> State {
        State{ existentials: 0 }
    }

    fn fresh_existentials(&mut self) -> Rc<String> {
        let  result = format!("t{}", self.existentials);
        self.existentials += 1;
        Rc::new(result)
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Literal(LiteralType),
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    Unit, 
    String, 
    Int, 
    Bool
}

#[derive(Debug, Clone)]
enum ContextElement {
    Variable(Rc<String>), 
    Existential(Rc<String>), 
    Solved(Rc<String>), 
    Marker(Rc<String>), 
    TypedVariable(Rc<String>, Type)
}

#[derive(Clone, Debug)]
struct Context {
    elements: Vec<ContextElement>
}

fn synt_to(context: &mut Context, state: &mut State) {}

fn topo_sort(fns: Arc<Vec<Spanned<ast::LispVal>>>) {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    for (i, spanned_lval) in fns.iter().enumerate() {
        match *spanned_lval.x {
            ast::LispValX::Function {..} => {
                h.insert(spanned_lval.x.clone(), i); 
            }, 
            _ => panic!("invalid LispVal passed of {:#?}", &spanned_lval)
        }
    }
}
pub fn synth(module: ast::LispModule) {

}

