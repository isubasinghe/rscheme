use std::sync::Arc;
use crate::common::*;


pub type LispVal = Arc<LispValX>;

#[derive(Clone, Debug)]
pub enum LispValX {
    Atom(Arc<String>), 
    List(Arc<Vec<Spanned<LispVal>>>),
    Int(Arc<String>),
    String(Arc<String>),
    Bool(bool),
    Function{
        name: Arc<String>,
        params: Vec<Arc<String>>,
        body: Arc<Vec<Spanned<LispVal>>>
    },
}

pub type LispModule = Arc<LispModuleX>;

#[derive(Clone, Debug)]
pub struct LispModuleX {
    pub functions: Arc<Vec<Spanned<LispVal>>> 
}
