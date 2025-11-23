use colored::Colorize;

use crate::isi::{
    ast::ast::{
        App, DataType, Function, FunctionParam, IsiNode,
        IsiToken::{ARROW, COLON, LBRACKET, LPAREN, RBRACKET, VARIABLE},
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
        print_compile_error(format!(
            "Hit unexpected `{}` in variable > Expected `->`",
            token.t_value
        ));
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
                app.next();
                parse_function(app)
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

fn parse_function(app: &mut App) -> IsiNode {
    let mut function = Function::default();
    // The current token is a LBRACKET `[`, so we are parsing function arguments now
    app.next();
    let function_params = parse_function_params(app);
    function.params = function_params;

    if app.get().t_type != ARROW {
        print_compile_error(format!(
            "Unexpected `{}` > Expected `->`",
            app.get().t_value
        ));
    }

    app.next();
    let return_type = app.get();
    if !return_type.t_type.is_data_type() {
        print_compile_error(format!(
            "Unexpected `{}` with type `{:?}` > Expected data type",
            return_type.t_value, return_type.t_type
        ));
    }

    let function_return_type = match return_type.t_value.as_str() {
        "int" => DataType::Int,
        "string" => DataType::String,
        "float" => DataType::Float,

        //Should never be hit because we already made sure its a datatype keyword
        _ => DataType::NONE,
    };
    function.return_type = function_return_type;

    app.next();

    let msg = "Return type is done, go parse the hecking body";
    println!("{}", msg.bright_cyan());

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
        if app.get().t_type != COLON {
            print_compile_error(format!(
                "Unexpected `{}` with type `{:?}` > Expected `:`",
                arg_name.t_value, arg_name.t_type
            ));
        }

        app.next();
        let arg_type = app.get();
        if !arg_type.t_type.is_data_type() {
            print_compile_error(format!(
                "Unexpected `{}` with type `{:?}` > Expected data type",
                arg_type.t_value, arg_type.t_type
            ));
        }

        let data_type = match arg_type.t_value.as_str() {
            "int" => DataType::Int,
            "string" => DataType::String,
            "float" => DataType::Float,

            //Should never be hit because we already made sure its a datatype keyword
            _ => DataType::NONE,
        };

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
