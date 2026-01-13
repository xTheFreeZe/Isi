use crate::isi::{
    ast::ast::{App, DataType, Expression, IsiToken, Token},
    util::util::print_compile_error,
};
use colored::Colorize;
use prejsx_math::eval_math;

/// Gatheres the expression from the index **to the end of the line**
///
/// Returns said expression as get_expression.0
/// and the new index position where the expression ended as get_expression.1
///
/// This is usefull as it is now optional to advance the index after gathering the expression
///
/// Sometimes you just want to know what kind of expression lies ahead without moving the index there
pub fn get_expression(app: &mut App) -> (Vec<Token>, usize) {
    let mut expression: Vec<Token> = Vec::new();
    let old_index = app.index;
    let current_line = app.get().t_line;
    let tokens_len = app.tokens.len();

    while app.get().t_line == current_line {
        // When we are at the last token of the file, consume it and break the loop
        if app.index + 1 == tokens_len {
            expression.push(app.get());
            app.next();
            break;
        }
        expression.push(app.get());
        app.next();
    }

    let new_index = app.index;
    app.index = old_index;
    (expression, new_index)
}

pub fn parse_expression(expression: &[Token]) -> Expression {
    // If it is in fact a simple math expression, we can return early,
    // as we already know the type and value
    if is_simple_algebra_expression(expression) {
        let string_expression = into_str(expression);
        if let Some(eval_result) = eval_simple_math_expression(&string_expression) {
            let debug_msg = format!(
                "`{}` got evaluated to `{}`",
                &string_expression, &eval_result
            );
            println!("{}", debug_msg.bright_black());
            return Expression {
                e_length: expression.len(),
                e_type: DataType::Int,
                e_value: eval_result,
                e_body: None,
            };
        } else {
            print_compile_error(&format!(
                "`{}` is not a valid math expression",
                string_expression
            ));
        }
    }

    let mut parsed_expression = Expression::default();
    for (index, piece) in expression.iter().enumerate() {
        let next_token = expression.get(index + 1);
        let next_does_exist = next_token.is_some();
        match &piece.t_type {
            IsiToken::STRING => {
                // TODO: Find a better way without cloning
                parsed_expression.e_value = piece.t_value.clone();
                parsed_expression.e_type = piece.t_type.to_data_type();
                parsed_expression.e_length += 1;
            }
            IsiToken::TRUE | IsiToken::FALSE => {
                parsed_expression.e_value = piece.t_value.clone();
                parsed_expression.e_type = piece.t_type.to_data_type();
                parsed_expression.e_length += 1;
            }
            _ => {
                print_compile_error(&format!(
                    "Unknown token type in expression parser: `{:?}` \nStopped on value: `{}`",
                    piece.t_type, piece.t_value
                ));
            }
        }
        // We've reached the last piece of the expression
        if !next_does_exist {
            break;
        }
    }

    parsed_expression
}

pub fn parse_single_expression(token: &Token) -> Expression {
    let mut expression = Expression::default();

    expression.e_length = 1;
    expression.e_type = token.t_type.to_data_type();
    expression.e_value = token.t_value.clone();

    expression
}

///Returns `true` if the expression is a simple math expression
///
/// 1 + 2 -> True
///
/// (add 1 2) + 2 -> False
fn is_simple_algebra_expression(expression: &[Token]) -> bool {
    expression.iter().all(|e| {
        matches!(
            e.t_type,
            IsiToken::INTEGER
                | IsiToken::PLUS
                | IsiToken::MINUS
                | IsiToken::STAR
                | IsiToken::SLASH
                | IsiToken::LPAREN
                | IsiToken::RPAREN
        )
    })
}

/// Turns an expression into a String
///
/// \[a, +, b] -> "a+b"
fn into_str(expression: &[Token]) -> String {
    expression.iter().map(|e| e.t_value.as_str()).collect()
}

/// This function uses the `prejsx_math` crate to eval math expressions for the compiler
///
/// It returns said expression or None, in case of an invalid expression
fn eval_simple_math_expression(expression: &str) -> Option<String> {
    if let Ok(e) = eval_math(expression) {
        Some(e.to_string())
    } else {
        None
    }
}
