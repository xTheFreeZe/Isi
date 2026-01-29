use crate::isi::util::util::print_compile_error;
use std::{collections::HashMap, fmt::Display, process::exit, sync::Arc};

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
    EQUALS,   // =
    STAR,     // *
    SLASH,    // /
    ARROW,    // ->
    SQUOTE,   // ''
    DQUOTE,   // ""
    COLON,    // :
    DOLLAR,   // $

    VARIABLE,
    INTEGER,
    KEYWORD,
    STRING,
    TRUE,
    FALSE,
    NIL,

    CALL, // Function calls. For Variables which are followed by a `(`
    WILDCARD,
    EOF,
    EMPTY,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Nil,

    NONE,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DataType::Int => write!(f, "integer"),
            DataType::Float => write!(f, "floating_point_number"),
            DataType::String => write!(f, "string_literal"),
            DataType::Bool => write!(f, "boolean"),
            DataType::Nil => write!(f, "nil"),
            DataType::NONE => write!(f, "none"),
        }
    }
}

impl IsiToken {
    pub fn to_data_type(&self) -> DataType {
        let data_type = match self {
            Self::INTEGER => DataType::Int,
            Self::STRING => DataType::String,
            Self::TRUE | Self::FALSE => DataType::Bool,
            _ => DataType::NONE,
        };

        if data_type == DataType::NONE {
            print_compile_error(&format!(
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
    pub t_value: Arc<str>,
    pub t_type: IsiToken,
    pub t_line: u64,
    pub t_column: u64,
}

impl Token {
    /// Checks of the `token value` is a data type
    ///
    /// "int" or "string" for example
    pub fn is_data_type(&self) -> bool {
        match self.t_value.as_ref() {
            "int" => true,
            "string" => true,
            "nil" => true,
            _ => false,
        }
    }

    /// Returns a [`DataType`] by matching on the value of the token
    pub fn to_data_type(&self) -> DataType {
        let data_type = match self.t_value.as_ref() {
            "int" => DataType::Int,
            "string" => DataType::String,
            "float" => DataType::Float,
            "nil" => DataType::Nil,

            _ => DataType::NONE,
        };

        if data_type == DataType::NONE {
            print_compile_error(&format!(
                "Tried to cast `{}` to a data type > Unknown",
                self.t_value
            ));
        }
        data_type
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Expression {
    pub e_length: usize,
    pub e_type: DataType,
    pub e_value: Arc<str>,
    pub e_body: Option<Vec<IsiNode>>,
}

impl Default for Expression {
    fn default() -> Self {
        Expression {
            e_length: 0,
            e_type: DataType::NONE,
            e_value: Arc::from(""),
            e_body: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariableDecl {
    pub name: Arc<str>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Variable {
    pub v_name: Arc<str>,
    pub v_type: DataType,
    pub v_node: Box<IsiNode>,
}

impl Default for Variable {
    fn default() -> Self {
        Variable {
            v_name: Arc::from(""),
            v_type: DataType::NONE,
            v_node: Box::new(IsiNode::EmptyNode),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionParam {
    pub name: Arc<str>,
    pub p_type: DataType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionDecl {
    pub name: Arc<str>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: Arc<str>,
    pub params: Option<Vec<FunctionParam>>,
    pub return_type: DataType,
    pub is_builtin: bool,
    pub function_body: Option<Vec<IsiNode>>,
    pub builtin_code: String,
}

impl Default for Function {
    fn default() -> Self {
        Function {
            name: Arc::from(""),
            params: None,
            return_type: DataType::NONE,
            is_builtin: false,
            function_body: None,
            builtin_code: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallArgument {
    pub name: Arc<str>,
    pub a_type: DataType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub function: Function,
    pub arguments: Option<Vec<FunctionCallArgument>>,
}

impl Default for FunctionCall {
    fn default() -> Self {
        FunctionCall {
            function: Function::default(),
            arguments: None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum IsiNode {
    IsiExpression(Expression),
    IsiVariableDecl(VariableDecl),
    IsiVariable(Variable),
    IsiFunctionDecl(FunctionDecl),
    IsiFunction(Function),
    IsiFunctionCall(FunctionCall),

    EmptyNode,
}

pub struct App {
    pub file_name: Arc<str>,
    pub file_dir: Arc<str>,

    pub content: Arc<str>,
    pub line_count: u64,
    pub column_count: u64,

    pub index: usize,
    pub current_var_str: String,
    pub tokens: Vec<Token>,
    pub nodes: Vec<IsiNode>,
    pub function_table: HashMap<Arc<str>, Function>,
    pub variable_table: HashMap<Arc<str>, Variable>,

    pub generated_code: String,
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
                print_compile_error(&format!("Unexpected end of file at index: {}", self.index));
                exit(1);
            }
        }
    }

    pub fn peek_next(&self) -> Token {
        let token = self.tokens.get(self.index + 1);

        match token {
            Some(token) => token.clone(),
            None => {
                print_compile_error(&format!("Unexpected end of file at index: {}", self.index));
                exit(1);
            }
        }
    }

    /// Throws a compile error of `app.get()` != `expected`
    pub fn expect(&self, expected: IsiToken) {
        let token = self.get();

        if token.t_type != expected {
            print_compile_error(&format!(
                "Unexpected `{}` > Expected `{}`",
                token.t_value,
                expected.string_value()
            ));
        }
    }

    pub fn get_node(&self) -> IsiNode {
        let node = self.nodes.get(self.index);

        match node {
            Some(n) => return n.clone(),
            None => {
                print_compile_error(&format!(
                    "Was unable to retrieve node at index: {}",
                    self.index
                ));
                exit(1);
            }
        }
    }

    pub fn get_function_from_map(&mut self, name: &str) -> Function {
        if let Some(f) = self.function_table.get(name) {
            return f.clone();
        } else {
            print_compile_error(&format!("Unknown function `{}`", name));
            exit(1);
        }
    }

    pub fn push_function_into_map(&mut self, function: Function) {
        self.function_table.insert(function.name.clone(), function);
    }

    pub fn get_variable_from_map(&mut self, name: &str) -> Variable {
        if let Some(f) = self.variable_table.get(name) {
            return f.clone();
        } else {
            print_compile_error(&format!("Unknown variable `{}`", name));
            exit(1);
        }
    }

    pub fn push_variable_into_map(&mut self, variable: Variable) {
        self.variable_table
            .insert(variable.v_name.clone(), variable);
    }
}
