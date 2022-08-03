
pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    x: T, 
    span: Span
}

#[derive(Clone, Debug)]
pub enum LispVal {
    Atom(String), 
    List(Vec<Spanned<LispVal>>),
    Int(i64),
    String(String),
    Nil, 
    Bool(bool),
    Function{
        name: String,
        params: String
    },
}



