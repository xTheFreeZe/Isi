use crate::isi::{
    ast::ast::{App, Expression, Token},
    util::util::print_compile_error,
};

pub fn get_expression(app: &mut App) -> Vec<Token> {
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

    app.index = old_index;
    expression
}

pub fn parse_single_expression(token: &Token) -> Expression {
    let mut expression = Expression::default();

    expression.e_length = 1;
    expression.e_type = token.t_type.to_data_type();
    expression.e_value = token.t_value.clone();

    expression
}
