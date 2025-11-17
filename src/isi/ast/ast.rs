use std::collections::HashMap;
pub enum IsiToken {
    LPAREN,   // (
    RPAREN,   // )
    LBRACKET, // [
    RBRACKET, // ]
    LBRACE,   // {
    RBRACE,   // }
    PIPE,     // |
    BANG,     // !
    LARROW,   // <
    RARROW,   // >
    QUESTION, // ?
    ARROW,    // ->
    SQUOTE,   // ''
    DQUOTE,   // ""

    INTEGER(i64),
    FLOAT(f64),
    STRING(String),
    KEYWORD(String),
    TRUE(),
    FALSE,
    NIL,

    IDENTIFIER(String),
    WILDCARD,
    EOF,
}

pub enum IsiValue {
    Integer(i64),
    Float(f64),
    String(String),
    Keyword(String),
    True,
    False,
    Nil,

    // (plus 1 2)
    Call(Vec<IsiValue>),

    // [1 2 3]
    Vector(Vec<IsiValue>),

    //{:a 1}
    Map(HashMap<String, IsiValue>),

    // < (plus 1 2) >
    Lazy(Box<IsiValue>),

    Function {
        params: Vec<String>,
        body: Box<IsiValue>,
    },
}

pub struct App {
    pub file_name: String,
    pub file_dir: String,

    pub content: String,
}
