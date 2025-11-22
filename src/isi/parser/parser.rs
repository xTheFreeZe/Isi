use colored::Colorize;

use crate::isi::{
    ast::ast::{
        App, Expression, IsiNode,
        IsiToken::{ARROW, LBRACKET, LPAREN, VARIABLE},
        Variable,
    },
    utils::utils::print_compile_error,
};

pub fn parse(app: &mut App) -> Vec<IsiNode> {
    let all_nodes: Vec<IsiNode> = Vec::new();

    while app.index < app.tokens.len() {
        let token = app.get();

        match token.t_type {
            VARIABLE => {
                let node = parse_variable(app);
                app.nodes.push(node);
            }
            _ => {}
        }

        app.next();
    }
    return all_nodes;
}

fn parse_variable(app: &mut App) -> IsiNode {
    let mut var = Variable::default();

    let mut token = app.get();
    println!("Got variable with name: {}", &token.t_value);
    var.v_value = token.t_value.to_string();
    app.next();

    token = app.get();
    if token.t_type != ARROW {
        print_compile_error(format!("Unexpected `{}` > Expected `->`", token.t_value));
    }

    app.next();
    token = app.get();
    let valid_tokens = ["(", "[", "{"];

    if !valid_tokens.iter().any(|e| e == &token.t_value) {
        print_compile_error(format!(
            "Unexpected `{}` > Expected one of these: `{:?}`",
            &token.t_value, valid_tokens
        ));
    }

    let ttype = token.t_type;

    let expression: IsiNode = match ttype {
        LPAREN => {
            let next = app.peek_next();

            let function_node = if next.t_type == LBRACKET {
                parse_function(app)
            } else {
                // This is a function call
                println!("{}", "Function calls are not yet implemented".red());
                IsiNode::EmptyNode
            };

            function_node
        }
        _ => return IsiNode::EmptyNode,
    };

    if expression == IsiNode::EmptyNode {
        print_compile_error(format!(
            "Case {} is not handeled yet for parsed variables",
            token.t_value
        ));
    }

    let node = IsiNode::IsiVariable(var);
    return node;
}

fn parse_function(app: &mut App) -> IsiNode {
    println!("{}", app.tokens.len());
    return IsiNode::IsiExpression(Expression::default());
}
