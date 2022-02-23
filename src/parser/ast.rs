pub enum LispVal {
    Atom(String), 
    List(Vec<LispVal>),
    ListAlt(Vec<LispVal>),
    Number(String),
    String(String),
    Nil, 
    Bool(bool),
}

pub struct Function {
    params: Vec<String>,
    vararg: Option<String>, 
    body: Vec<LispVal>
}
