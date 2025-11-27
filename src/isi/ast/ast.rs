use std::process::exit;

use crate::isi::util::util::print_compile_error;

const DATA_TYPES: &[&str] = &["int", "string"];

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
    KEYWORD(String),
    TRUE(),
    FALSE,
    NIL,

    IDENTIFIER(String),
    WILDCARD,
    EOF,
    EMPTY,
}

#[derive(PartialEq, Debug)]
pub enum DataType {
    Int,
    Float,
    String,

    NONE,
}

impl IsiToken {
    pub fn is_data_type(&self) -> bool {
        matches!(self, IsiToken::KEYWORD(s) if DATA_TYPES.contains(&s.as_str()))
    }

    pub fn to_data_type(&self) -> DataType {
        let data_type = match self {
            Self::INTEGER(_) => DataType::Int,
            _ => DataType::NONE,
        };

        if data_type == DataType::NONE {
            print_compile_error(format!(
                "Tried to cast `{:?}` to a data type > Unknown",
                self
            ));
        }

        data_type
    }

    pub fn string_value(&self) -> String {
        match self {
            Self::COLON => ":".to_string(),
            Self::ARROW => "->".to_string(),
            Self::LPAREN => "(".to_string(),
            Self::RPAREN => ")".to_string(),
            Self::LBRACKET => "[".to_string(),
            Self::RBRACKET => "]".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_value: String,
    pub t_type: IsiToken,
    pub t_line: i64,
    pub t_column: i64,
}

impl Token {
    pub fn to_data_type(&self) -> DataType {
        let data_type = match self.t_value.as_str() {
            "int" => DataType::Int,
            "string" => DataType::String,
            "float" => DataType::Float,

            _ => DataType::NONE,
        };

        if data_type == DataType::NONE {
            print_compile_error(format!(
                "Tried to cast `{}` to a data type > Unknown",
                self.t_value
            ));
        }
        data_type
    }
}

#[derive(PartialEq, Debug)]
pub struct Expression {
    pub e_length: u64,
    pub e_type: DataType,
    pub e_value: String,
    pub e_body: Vec<IsiNode>,
}

impl Default for Expression {
    fn default() -> Self {
        Expression {
            e_length: 0,
            e_type: DataType::NONE,
            e_value: String::new(),
            e_body: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub p_type: DataType,
}

#[derive(PartialEq, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub return_type: DataType,
    pub function_body: Vec<IsiNode>,
}

impl Default for Function {
    fn default() -> Self {
        Function {
            name: String::new(),
            params: Vec::new(),
            return_type: DataType::NONE,
            function_body: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Return {
    pub r_value: Expression,
}

impl Default for Return {
    fn default() -> Self {
        Return {
            r_value: Expression::default(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum IsiNode {
    IsiExpression(Expression),
    IsiVariable(Variable),
    IsiFunction(Function),
    IsiReturn(Return),

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

    /// Throws a compile error of `app.get()` != `expected`
    pub fn expect(&self, expected: IsiToken) {
        let token = self.get();

        if token.t_type != expected {
            print_compile_error(format!(
                "Unexpected `{}` > Expected `{}`",
                token.t_value,
                expected.string_value()
            ));
        }
    }
}
