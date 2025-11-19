use std::process::exit;

use crate::isi::utils::utils::print_compile_error;

#[derive(Debug, PartialEq)]
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
    EMPTY,
}

#[derive(Debug)]
pub struct Token {
    pub t_value: String,
    pub t_type: IsiToken,
    pub t_line: i64,
    pub t_column: i64,
}

pub struct Expression {
    pub e_type: IsiToken,
    pub e_value: String,
}

impl Default for Expression {
    fn default() -> Self {
        Expression {
            e_type: IsiToken::EMPTY,
            e_value: String::new(),
        }
    }
}

pub struct Variable {
    pub v_value: String,
    pub v_expression: Expression,
}

impl Default for Variable {
    fn default() -> Self {
        Variable {
            v_value: String::new(),
            v_expression: Expression::default(),
        }
    }
}

pub enum IsiNode {
    IsiExpression(Expression),
    IsiVariable(Variable),
}

// pub enum IsiValue {
//     Integer(i64),
//     Float(f64),
//     String(String),
//     Keyword(String),
//     True,
//     False,
//     Nil,

//     // (plus 1 2)
//     Call(Vec<IsiValue>),

//     // [1 2 3]
//     Vector(Vec<IsiValue>),

//     //{:a 1}
//     Map(HashMap<String, IsiValue>),

//     // < (plus 1 2) >
//     Lazy(Box<IsiValue>),

//     Function {
//         params: Vec<String>,
//         body: Box<IsiValue>,
//     },

//     Variable {
//         v_type: IsiToken,
//         v_value: String,
//     },
// }

pub struct App {
    pub file_name: String,
    pub file_dir: String,

    pub content: String,
    pub line_count: i64,
    pub column_count: i64,

    pub index: usize,
    pub tokens: Vec<Token>,
    pub nodes: Vec<IsiNode>,
}

impl App {
    pub fn next(&mut self) {
        self.index += 1
    }

    pub fn get(&self) -> &Token {
        let token = self.tokens.get(self.index);

        match token {
            Some(token) => token,
            None => {
                print_compile_error(format!("Token index {} out of bounds", self.index));
                exit(1);
            }
        }
    }
}
