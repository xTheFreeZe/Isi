use crate::isi::{
    ast::ast::{App, Expression, IsiNode, Return},
    parser::expression::{get_expression, parse_single_expression},
    util::util::print_compile_error,
};

pub fn parse_return(app: &mut App) -> IsiNode {
    let mut return_stmt = Return::default();
    app.next();

    let expression_array = get_expression(app);
    let mut expression: Expression = Expression::default();

    if expression_array.len() == 1 {
        if let Some(e) = expression_array.first() {
            expression = parse_single_expression(e)
        } else {
            print_compile_error(String::from("Error when parsing single length expression"));
        }
    }
    // else do a more advanced expression parsing

    return_stmt.r_value = expression;
    IsiNode::IsiReturn(return_stmt)
}
