use std::{process::exit, sync::Arc};

use crate::isi::{
    ast::ast::{App, DataType, Expression, IsiNode, IsiToken, Token, Variable},
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

pub fn parse_expression(app: &mut App, expression: &[Token]) -> Expression {
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
                e_value: Arc::from(eval_result),
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
                parsed_expression.e_value = piece.t_value.clone();
                parsed_expression.e_type = piece.t_type.to_data_type();
                parsed_expression.e_length += 1;
            }
            IsiToken::TRUE | IsiToken::FALSE => {
                parsed_expression.e_value = piece.t_value.clone();
                parsed_expression.e_type = piece.t_type.to_data_type();
                parsed_expression.e_length += 1;
            }
            IsiToken::VARIABLE => {
                let var = get_variable(&piece.t_value, app);
                parsed_expression.e_value = var.v_name;
                parsed_expression.e_type = var.v_type;
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
    expression.iter().map(|e| e.t_value.as_ref()).collect()
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

pub fn get_variable(var_name: &str, app: &App) -> Variable {
    // 1) locals / global vars (whatever is currently in the table)
    if let Some(v) = app.variable_table.get(var_name) {
        return v.clone();
    }

    // 2) parameters
    if let Some((params, _ret_type)) = app.get_function_sig_from_map(&app.current_var_str) {
        if let Some(params) = params {
            if let Some(p) = params.iter().find(|p| p.name.as_ref() == var_name) {
                return Variable {
                    v_name: p.name.clone(),
                    v_type: p.p_type,
                    v_node: Box::new(IsiNode::EmptyNode),
                };
            }
        }
    }

    print_compile_error(&format!(
        "Unknown variable `{}` > NEW GET VARIABLE FUNCTION (bruuuh)",
        var_name
    ));
    exit(1);
}

pub fn is_variable_accessable(var_name: &str, app: &App) -> bool {
    if app.variable_table.contains_key(var_name) {
        return true;
    }

    let params_option = app.get_function_sig_from_map(&app.current_var_str);

    if params_option.is_none() {
        return false;
    }

    let params = params_option.unwrap().0;

    if let Some(p) = params {
        if let Some(_) = p.into_iter().find(|p| p.name.as_ref() == var_name) {
            return true;
        }
    }

    false
}

// The new one...
// pub fn get_variable(var_name: &str, app: &App) -> Variable {
//     // 1) search locals first (variables with matching scope)
//     if let Some(v) = app
//         .variable_table
//         .values()
//         .find(|v| v.v_name == var_name && v.scope == app.current_var_str)
//     {
//         return v.clone();
//     }

//     // 2) search function parameters if we are inside a function
//     if !app.current_var_str.is_empty() {
//         if let Some((params_option, _ret_type)) = app.get_function_sig_from_map(&app.current_var_str)
//         {
//             if let Some(params) = params_option {
//                 if let Some(p) = params.iter().find(|p| p.name.as_ref() == var_name) {
//                     return Variable {
//                         v_name: p.name.clone(),
//                         v_type: p.p_type,
//                         v_node: Box::new(IsiNode::EmptyNode),
//                         scope: app.current_var_str.clone(),
//                     };
//                 }
//             }
//         }
//     }

//     // 3) search globals
//     if let Some(v) = app
//         .variable_table
//         .values()
//         .find(|v| v.v_name == var_name && v.scope.is_empty())
//     {
//         return v.clone();
//     }

//     print_compile_error(&format!("Unknown variable `{}`", var_name));
//     exit(1);
// }
