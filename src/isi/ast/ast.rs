use std::process::exit;

use crate::isi::utils::utils::print_compile_error;

#[derive(Debug, PartialEq, Clone)]
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
    PLUS,     // +
    ARROW,    // ->
    SQUOTE,   // ''
    DQUOTE,   // ""
    COLON,    // :

    VARIABLE,
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

#[derive(Debug, Clone)]
pub struct Token {
    pub t_value: String,
    pub t_type: IsiToken,
    pub t_line: i64,
    pub t_column: i64,
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum IsiNode {
    IsiExpression(Expression),
    IsiVariable(Variable),

    EmptyNode,
}

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

    pub fn get(&self) -> Token {
        let token = self.tokens.get(self.index);

        match token {
            Some(token) => token.clone(),
            None => {
                print_compile_error(format!("Unexpected end of file at index: {}", self.index));
                exit(1);
            }
        }
    }

    pub fn peek_next(&self) -> Token {
        let token = self.tokens.get(self.index + 1);

        match token {
            Some(token) => token.clone(),
            None => {
                print_compile_error(format!("Unexpected end of file at index: {}", self.index));
                exit(1);
            }
        }
    }
}
