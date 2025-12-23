use crate::isi::ast::ast::Token;
use crate::isi::util::util::print_compile_error;
use crate::{App, isi::ast::ast::IsiToken};

fn default_token(app: &App) -> Token {
    Token {
        t_column: app.column_count,
        t_line: app.line_count,
        t_value: String::new(),
        t_type: IsiToken::EMPTY,
    }
}

pub fn scan(app: &mut App) -> Vec<Token> {
    let keywords: Vec<&str> = vec!["return", "int", "string", "float"];
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = app.content.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut full_number = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        app.column_count += 1;
                        full_number.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                app.column_count -= 1;
                tokens.push(Token {
                    t_value: full_number,
                    t_type: IsiToken::INTEGER,
                    ..default_token(app)
                });
            }
            'a'..='z' => {
                let mut full_str = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_alphanumeric() || d == '_' {
                        app.column_count += 1;
                        full_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                app.column_count -= 1;
                // Keyword
                if keywords.contains(&full_str.as_str()) {
                    tokens.push(Token {
                        t_value: String::from(&full_str),
                        t_type: IsiToken::KEYWORD,
                        ..default_token(app)
                    });
                // x( -> Call
                } else if chars.peek().unwrap() == &'(' {
                    tokens.push(Token {
                        t_value: String::from(&full_str),
                        t_type: IsiToken::CALL,
                        ..default_token(app)
                    });
                } else {
                    tokens.push(Token {
                        t_value: String::from(&full_str),
                        t_type: IsiToken::VARIABLE,
                        ..default_token(app)
                    });
                }
            }
            '-' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::MINUS,
                    ..default_token(app)
                });
                chars.next();

                if chars.peek().unwrap() == &'>' {
                    tokens.pop();
                    tokens.push(Token {
                        t_value: String::from("->"),
                        t_type: IsiToken::ARROW,
                        ..default_token(app)
                    });
                    chars.next();
                }
            }
            '+' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::PLUS,
                    ..default_token(app)
                });
                chars.next();
            }
            '>' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::RARROW,
                    ..default_token(app)
                });
                chars.next();
            }
            '<' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::LARROW,
                    ..default_token(app)
                });
                chars.next();
            }
            '(' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::LPAREN,
                    ..default_token(app)
                });
                chars.next();
            }
            ')' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::RPAREN,
                    ..default_token(app)
                });
                chars.next();
            }
            '[' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::LBRACKET,
                    ..default_token(app)
                });
                chars.next();
            }
            ']' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::RBRACKET,
                    ..default_token(app)
                });
                chars.next();
            }
            '{' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::LBRACE,
                    ..default_token(app)
                });
                chars.next();
            }
            '}' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::RBRACE,
                    ..default_token(app)
                });
                chars.next();
            }
            ':' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::COLON,
                    ..default_token(app)
                });
                chars.next();
            }
            '"' => {
                let mut full_str = String::new();
                chars.next();
                while let Some(&d) = chars.peek() {
                    // TODO: How to check if there is no closing " ? Currently just runs until file end
                    if d != '"' {
                        app.column_count += 1;
                        full_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Consume the closing "
                chars.next();
                tokens.push(Token {
                    t_value: full_str,
                    t_type: IsiToken::STRING,
                    ..default_token(app)
                });
            }
            _ => {
                if c.is_whitespace() || c == '\r' {
                    // For windwos new lines -> \r\n
                    if let Some(&n) = chars.peek()
                        && n == '\n'
                    {
                        app.line_count += 1;
                        app.column_count = 1;
                    }
                    chars.next();
                } else if c == '\n' {
                    app.line_count += 1;
                    chars.next();
                } else {
                    print_compile_error(format!("Unknown token: `{}`", c));
                }
            }
        }
        app.column_count += 1;
    }

    return tokens;
}
