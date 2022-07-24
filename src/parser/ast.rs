pub enum LispVal {
    Atom(String), 
    List(Vec<LispVal>),
    Number(String),
    String(String),
    Nil, 
    Bool(bool),
    Function{
        params: Vec<String>,
        name: Option<String>, // anonymous or not
        vararg: Option<String>,
        body: Vec<LispVal>
    },
}


