use std::sync::Arc;

type AVec<T> = Arc<Vec<Arc<T>>>;

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub x: T, 
    pub span: Span
}

pub type LispVal = Arc<LispValX>;

#[derive(Clone, Debug)]
pub enum LispValX {
    Atom(Arc<String>), 
    List(Arc<Vec<Spanned<LispVal>>>),
    Int(Arc<String>),
    String(Arc<String>),
    Nil, 
    Bool(bool),
    Function{
        name: Arc<String>,
        params: Arc<String>
    },
}



