use std::sync::Arc;
use crate::parser::ast;
use crate::common::*;

pub type LispVal = Arc<LispValX>;

#[derive(Clone, Debug)]
pub enum LispValX {
    List(Arc<Vec<Spanned<LispVal>>>), 
    Int(Arc<String>), 
    String(Arc<String>), 
    Bool(bool), 
    Function {
        name: Arc<String>, 
        params: Vec<Arc<String>>, 
        requires: Option<Arc<Vec<Spanned<LispVal>>>>, 
        ensures: Option<Arc<Spanned<LispVal>>>, 
        body: Arc<Vec<Spanned<LispVal>>>
    },

    FunctionCall {
        name: Arc<String>, 
        args: Arc<Vec<Spanned<LispVal>>>
    }
}
