use crate::isi::{
    ast::ast::{App, DataType, Expression, IsiToken, Token},
    util::util::print_compile_error,
};

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
        if app.index + 1 == tokens_len {
            print_compile_error(String::from(
                "Hit unexpected EOF while gathering expression",
            ));
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
    if is_simple_algebra_expression(expression) && is_valid_math_expression(into_str(expression)) {
        return Expression {
            e_length: expression.len(),
            e_type: DataType::Int,
            e_value: into_str(expression),
            e_body: None,
        };
    }
    let parsed_expression = Expression::default();
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

fn is_valid_math_expression(_expression: String) -> bool {
    todo!("Validate that expression is a valid math expression")
}
