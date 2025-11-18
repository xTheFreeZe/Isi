use std::collections::HashMap;

#[derive(Debug)]
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
    MINUS,    // -
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

#[derive(Debug)]
pub struct Token {
    pub t_value: String,
    pub t_type: IsiToken,
    pub t_line: i64,
    pub t_column: i64,
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

    Variable {
        v_type: IsiToken,
        v_value: String,
    },
}

pub struct App {
    pub file_name: String,
    pub file_dir: String,

    pub content: String,
    pub line_count: i64,
    pub column_count: i64,

    pub tokens: Vec<Token>,
    pub nodes: Vec<IsiValue>,
}
