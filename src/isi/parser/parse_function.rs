use crate::isi::{
    ast::ast::{App, DataType, Function, FunctionDecl, FunctionParam, IsiNode, IsiToken},
    parser::expression::{get_expression, parse_expression},
    util::util::print_compile_error,
};

pub fn parse_function(app: &mut App) -> (IsiNode, DataType) {
    let mut function = Function::default();
    function.name = app.current_var_str.clone();

    // This check is necessary because the [...] might have been omitted
    if app.get().t_type == IsiToken::LBRACKET {
        // Consume the [ and go the first param name
        app.next();
        let function_params = parse_function_params(app);
        function.params = Some(function_params);
    }

    app.expect(IsiToken::COLON);
    app.next();
    let return_type = app.get();
    if !return_type.is_data_type() {
        print_compile_error(&format!(
            "Unexpected `{}` with type `{:?}` > Expected data type",
            return_type.t_value, return_type.t_type
        ));
    }

    let f_return_type = return_type.to_data_type();
    function.return_type = f_return_type;

    app.next();
    app.expect(IsiToken::LBRACE);
    app.next();

    // Empty function
    let mut latest_expression_type = DataType::NONE;
    if app.get().t_type == IsiToken::RBRACE {
        app.next();
    } else {
        let (f_body, latest_expression) = parse_function_body(app);
        function.function_body = Some(f_body);
        latest_expression_type = latest_expression;
    }
    app.expect(IsiToken::RPAREN);
    app.next();

    if latest_expression_type == DataType::NONE {
        print_compile_error(format!(
            "Empty block in function `{}`: An empty block is not allowed > All blocks must evaluate to a value",
            app.current_var_str
        ).as_str());
    }

    if latest_expression_type != f_return_type {
        print_compile_error(
            format!(
                "Mismatched types: Function `{}` expexted `{}`, found `{}`",
                app.current_var_str, f_return_type, latest_expression_type
            )
            .as_str(),
        );
    }

    app.push_function_into_map(function);
    let function_decl = FunctionDecl {
        name: app.current_var_str.clone(),
    };
    (
        IsiNode::IsiFunctionDecl(function_decl),
        latest_expression_type,
    )
}

fn parse_function_params(app: &mut App) -> Vec<FunctionParam> {
    let mut params = Vec::new();

    if app.get().t_type == IsiToken::RBRACKET {
        app.next();
        return params;
    }

    while app.get().t_type != IsiToken::RBRACKET {
        let arg_name = app.get();
        if arg_name.t_type != IsiToken::VARIABLE {
            print_compile_error(&format!(
                "Unexpected `{}` with type `{:?}` > Expected function parameter",
                arg_name.t_value, arg_name.t_type
            ));
        }

        app.next();
        app.expect(IsiToken::COLON);

        app.next();
        let arg_type = app.get();
        if !arg_type.is_data_type() {
            print_compile_error(&format!(
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

fn parse_function_body(app: &mut App) -> (Vec<IsiNode>, DataType) {
    let mut body: Vec<IsiNode> = Vec::new();
    while app.get().t_type != IsiToken::RBRACE {
        let token = app.get();
        match token.t_type {
            IsiToken::KEYWORD => match token.t_value.as_str() {
                _ => {
                    print_compile_error(&format!("Unknown keyword `{}`", token.t_value));
                }
            },
            IsiToken::INTEGER => {
                let expression = get_expression(app);
                let int_expression = parse_expression(&expression.0);
                body.push(IsiNode::IsiExpression(int_expression));

                // Set the index of the parser to the end of the parsed expression
                app.index = expression.1;
            }
            IsiToken::STRING => {
                let expression = get_expression(app);
                let string_expression = parse_expression(&expression.0);
                body.push(IsiNode::IsiExpression(string_expression));

                app.index = expression.1;
            }
            _ => {
                print_compile_error(&format!(
                    "Unexpected token: `{}` with type `{:?}` in function body",
                    token.t_value, token.t_type
                ));
            }
        }
    }
    app.expect(IsiToken::RBRACE);
    // Go over the `}`

    app.next();

    let latest_data_type = retrieve_last_data_type(&mut body);
    (body, latest_data_type)
}

/// Starts at the end of the function body and goes back to the front, until it finds
/// a Datatype, in which case, that is the latest
pub fn retrieve_last_data_type(body: &mut [IsiNode]) -> DataType {
    body.reverse();
    let mut latest = DataType::NONE;
    for element in body {
        match element {
            IsiNode::IsiExpression(expression) => {
                latest = expression.e_type;
                break;
            }
            _ => {
                continue;
            }
        }
    }
    latest
}
