use crate::isi::{
    ast::ast::{App, Function, FunctionParam, IsiNode, IsiToken, Variable},
    parser::expression::{get_expression, parse_expression},
    util::util::print_compile_error,
};

pub fn parse(app: &mut App) {
    while app.index < app.tokens.len() {
        let token = app.get();

        match token.t_type {
            IsiToken::VARIABLE => {
                let node = parse_variable(app);
                app.nodes.push(node);
            }
            _ => {
                print_compile_error(&format!("Unexpected top level token `{}`", token.t_value));
            }
        }
    }
}

fn parse_variable(app: &mut App) -> IsiNode {
    let mut var = Variable::default();

    let mut token = app.get();
    var.v_name = token.t_value.to_string();
    app.next();

    app.expect(IsiToken::ARROW);

    app.next();
    token = app.get();
    let valid_tokens = ["(", "[", "{"];

    // Checks if the token after -> is one of the bracktes above, a number, a string or a function call
    // TODO: Needs to also match Variables
    if !valid_tokens.iter().any(|e| e == &token.t_value)
        && !matches!(token.t_type, IsiToken::INTEGER)
        && !matches!(token.t_type, IsiToken::STRING)
        && token.t_type != IsiToken::CALL
    {
        print_compile_error(&format!(
            "Unexpected `{}` > Expected either: `(`, `[` or `{{`",
            &token.t_value,
        ));
    }

    let ttype = &token.t_type;

    let expression: IsiNode = match ttype {
        IsiToken::LPAREN => {
            let next = app.peek_next();

            // If no function params are needed, you can omit the [...] and continue with :{return_type}
            // main -> ([] :int) turns into main -> ( :int )
            let function_node =
                if next.t_type == IsiToken::LBRACKET || next.t_type == IsiToken::COLON {
                    app.next();
                    parse_function(app)
                } else {
                    IsiNode::EmptyNode
                };

            function_node
        }
        // x -> 10
        // This arm is used when you assign a variable a number
        IsiToken::INTEGER => {
            let expression = get_expression(app);
            let int_expression = parse_expression(&expression.0);
            app.index = expression.1;
            IsiNode::IsiExpression(int_expression)
        }
        IsiToken::STRING => {
            let expression = get_expression(app);
            let string_expression = parse_expression(&expression.0);
            app.index = expression.1;
            IsiNode::IsiExpression(string_expression)
        }
        _ => IsiNode::EmptyNode,
    };

    if expression == IsiNode::EmptyNode {
        print_compile_error(&format!(
            "Case {} is not handeled yet for parsed variables",
            token.t_value
        ));
    }

    var.v_node = Box::new(expression);
    IsiNode::IsiVariable(var)
}

fn parse_function(app: &mut App) -> IsiNode {
    let mut function = Function::default();

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
    if app.get().t_type == IsiToken::RBRACE {
        app.next();
    } else {
        let f_body = parse_function_body(app);
        function.function_body = Some(f_body);
    }
    app.expect(IsiToken::RPAREN);
    app.next();

    IsiNode::IsiFunction(function)
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

fn parse_function_body(app: &mut App) -> Vec<IsiNode> {
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
    body
}
