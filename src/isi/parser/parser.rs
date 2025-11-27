use colored::Colorize;

use crate::isi::{
    ast::ast::{App, Function, FunctionParam, IsiNode, IsiToken::*, Variable},
    parser::parse_return::parse_return,
    util::util::print_compile_error,
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
    var.v_value = token.t_value.to_string();
    app.next();

    app.expect(ARROW);

    app.next();
    token = app.get();
    let valid_tokens = ["(", "[", "{"];

    if !valid_tokens.iter().any(|e| e == &token.t_value) {
        print_compile_error(format!(
            "Unexpected `{}` > Expected either: `(`, `[` or `{{`",
            &token.t_value,
        ));
    }

    let ttype = token.t_type;

    let expression: IsiNode = match ttype {
        LPAREN => {
            let next = app.peek_next();

            let function_node = if next.t_type == LBRACKET {
                app.next();
                parse_function_head(app)
            } else {
                // This is a function call
                println!("{}", "Function calls are not yet implemented".red());
                IsiNode::EmptyNode
            };

            function_node
        }
        _ => IsiNode::EmptyNode,
    };

    if expression == IsiNode::EmptyNode {
        print_compile_error(format!(
            "Case {} is not handeled yet for parsed variables",
            token.t_value
        ));
    }

    let node = IsiNode::IsiVariable(var);
    node
}

fn parse_function_head(app: &mut App) -> IsiNode {
    let mut function = Function::default();
    // The current token is a LBRACKET `[`, so we are parsing function arguments now
    app.next();
    let function_params = parse_function_params(app);
    function.params = function_params;

    app.expect(ARROW);

    app.next();
    let return_type = app.get();
    if !return_type.t_type.is_data_type() {
        print_compile_error(format!(
            "Unexpected `{}` with type `{:?}` > Expected data type",
            return_type.t_value, return_type.t_type
        ));
    }

    let f_return_type = return_type.to_data_type();
    function.return_type = f_return_type;

    app.next();
    app.expect(LBRACE);
    app.next();

    let f_body = parse_function_body(app);
    function.function_body = f_body;

    IsiNode::IsiFunction(function)
}

fn parse_function_params(app: &mut App) -> Vec<FunctionParam> {
    let mut params = Vec::new();

    while app.get().t_type != RBRACKET {
        let arg_name = app.get();
        if arg_name.t_type != VARIABLE {
            print_compile_error(format!(
                "Unexpected `{}` with type `{:?}` > Expected function parameter",
                arg_name.t_value, arg_name.t_type
            ));
        }

        app.next();
        app.expect(COLON);

        app.next();
        let arg_type = app.get();
        if !arg_type.t_type.is_data_type() {
            print_compile_error(format!(
                "Unexpected `{}` with type `{:?}` > Expected data type",
                arg_type.t_value, arg_type.t_type
            ));
        }

        let data_type = arg_type.to_data_type();
        let param = FunctionParam {
            name: arg_name.t_value,
            p_type: data_type,
        };
        params.push(param);
        app.next();
    }
    // Jump over the `]`
    app.next();
    params
}

fn parse_function_body(app: &mut App) -> Vec<IsiNode> {
    let mut body = Vec::new();
    let token = app.get();
    match token.t_type {
        KEYWORD(r) => match r.as_str() {
            "return" => {
                let return_stmt = parse_return(app);
                println!("Got return {:?}", return_stmt);
                body.push(return_stmt);
            }
            _ => {
                print_compile_error(format!("Unknown keyword `{r}`"));
            }
        },
        _ => {
            print_compile_error(format!(
                "Unexpected token: `{}` `{:?}`",
                token.t_value, token.t_type
            ));
        }
    }
    body
}
