use std::sync::Arc;

use crate::isi::{
    ast::ast::{App, DataType, IsiNode, IsiToken, Token, Variable, VariableDecl},
    parser::{
        expression::{get_expression, parse_expression},
        extras::parse_match,
        parse_call::parse_call,
        parse_function::parse_function,
    },
    util::util::print_compile_error,
};

pub fn parse(app: &mut App) {
    while app.index < app.tokens.len() {
        let token = app.get();

        match token.t_type {
            IsiToken::VARIABLE => {
                let node = parse_variable(app, false);
                app.nodes.push(node);
            }
            IsiToken::LPAREN => {
                let node = parse_call(app);
                app.nodes.push(node.0);
            }
            IsiToken::QUESTION => {
                let node = parse_match(app, false);
                app.nodes.push(node.0);
            }
            _ => {
                print_compile_error(&format!("Unexpected top level token `{}`", token.t_value));
            }
        }
    }
}

/// Parses the current variable and just returns the VariableDecl -> A String Node
///
/// The actual variable gets pushed into the HashMap
///
/// `inside_function`: Tells the compiler to overwrite the current function in the app if true
pub fn parse_variable(app: &mut App, inside_function: bool) -> IsiNode {
    let mut var = Variable::default();
    let mut is_builtin_func = false;

    let mut token = app.get();
    var.v_name = Arc::clone(&token.t_value);

    if var.v_name.as_ref() == "main" {
        print_compile_error("Variable name `main` can not be used");
    }

    if !inside_function {
        app.current_var_str = token.t_value.to_string();
    }

    app.next();

    app.expect(IsiToken::ARROW);

    app.next();

    if app.get().t_value.as_ref() == "c" {
        is_builtin_func = true;
        app.next();
    }

    token = app.get();

    let valid_tokens = ["(", "[", "{", "?"];

    // Checks if the token after -> is one of the bracktes above, a number, a string or a function call
    // TODO: Needs to also match Variables
    if !valid_tokens.iter().any(|e| e == &token.t_value.as_ref())
        && !matches!(token.t_type, IsiToken::INTEGER)
        && !matches!(token.t_type, IsiToken::STRING)
        && !matches!(token.t_type, IsiToken::TRUE)
        && !matches!(token.t_type, IsiToken::FALSE)
        && token.t_type != IsiToken::CALL
    {
        print_compile_error(&format!(
            "Unexpected `{}` > Expected either: `(`, `[` or `{{` or a valid value",
            &token.t_value,
        ));
    }

    let expression: IsiNode = match token.t_type {
        IsiToken::LPAREN => {
            let next = app.peek_next();

            // If no function params are needed, you can omit the [...] and continue with :{return_type}
            // main -> ([] :int) turns into main -> ( :int )
            let (function_node, function_type) =
                if next.t_type == IsiToken::LBRACKET || next.t_type == IsiToken::COLON {
                    if inside_function {
                        print_compile_error("Can not create a function inside a function");
                    }
                    app.next();
                    let function = parse_function(app, is_builtin_func);
                    app.current_var_str = String::from("");
                    function
                } else if next.t_type == IsiToken::VARIABLE {
                    // This is a function call:
                    // x -> (plus x x)
                    let call_return = parse_call(app);

                    if call_return.1 == DataType::Nil {
                        print_compile_error(&format!(
                            "Tried assigning a value of type `nil` to variable {}",
                            var.v_name
                        ));
                    }

                    call_return
                } else {
                    (IsiNode::EmptyNode, DataType::NONE)
                };
            var.v_type = function_type;
            function_node
        }
        // x -> 10
        // This arm is used when you assign a variable a number
        IsiToken::INTEGER => {
            let expression = get_expression(app, None);
            let int_expression = parse_expression(app, &expression.0);
            app.index = expression.1;
            var.v_type = DataType::Int;
            IsiNode::IsiExpression(int_expression)
        }
        IsiToken::STRING => {
            let expression = get_expression(app, None);
            let string_expression = parse_expression(app, &expression.0);
            app.index = expression.1;
            var.v_type = DataType::String;
            IsiNode::IsiExpression(string_expression)
        }
        IsiToken::TRUE | IsiToken::FALSE => {
            let expression = get_expression(app, None);
            let bool_expression = parse_expression(app, &expression.0);
            app.index = expression.1;
            var.v_type = DataType::Bool;
            IsiNode::IsiExpression(bool_expression)
        }
        IsiToken::QUESTION => {
            let match_parse = parse_match(app, true);
            var.v_type = match_parse.1;
            match_parse.0
        }
        _ => IsiNode::EmptyNode,
    };

    if expression == IsiNode::EmptyNode {
        print_compile_error(&format!(
            "Case `{}{}` is not handeled yet for parsed variables",
            token.t_value,
            app.peek_next().t_value
        ));
    }

    match &expression {
        IsiNode::IsiFunctionDecl(decl) => {
            if decl.name.is_empty() {
                // This is a stupid bug where:
                // If we are currently IN a function and that function has a variable that calls another function:
                // func -> ((
                //  x -> plus(2 2)
                // ))
                // The compiler will add another function decl with an empty name for some reason?!
                // TODO: Fix this
                // return IsiNode::EmptyNode;
                // dbg!(decl);
            }
        }
        _ => {}
    }

    var.v_node = Box::new(expression);
    let var_decl = VariableDecl {
        name: var.v_name.clone(),
    };
    app.push_variable_into_map(var);
    IsiNode::IsiVariableDecl(var_decl)
}

pub fn parse_until(app: &mut App, ttype: IsiToken) -> Vec<IsiNode> {
    let mut nodes = Vec::new();
    while app.get().t_type != ttype.clone() {
        let token = app.get();
        match token.t_type {
            IsiToken::INTEGER => {
                let expression = get_expression(app, Some(ttype.clone()));
                let int_expression = parse_expression(app, &expression.0);
                nodes.push(IsiNode::IsiExpression(int_expression));
                app.index = expression.1;
            }
            IsiToken::STRING => {
                let expression = get_expression(app, Some(ttype.clone()));
                let string_expression = parse_expression(app, &expression.0);
                nodes.push(IsiNode::IsiExpression(string_expression));
                app.index = expression.1;
            }
            IsiToken::TRUE | IsiToken::FALSE => {
                let expression = get_expression(app, Some(ttype.clone()));
                let bool_expression = parse_expression(app, &expression.0);
                nodes.push(IsiNode::IsiExpression(bool_expression));
                app.index = expression.1;
            }
            IsiToken::VARIABLE => {
                let expression = get_expression(app, Some(ttype.clone()));
                let variable_expression = parse_expression(app, &expression.0);
                nodes.push(IsiNode::IsiExpression(variable_expression));

                app.index = expression.1;
            }
            IsiToken::LPAREN => {
                let node = parse_call(app);
                nodes.push(node.0);
            }
            IsiToken::QUESTION => {
                let node = parse_match(app, false);
                nodes.push(node.0);
            }
            _ => {
                print_compile_error(&format!(
                    "Unexpected token: `{}` with type `{:?}` in function body [parse_until]",
                    token.t_value, token.t_type
                ));
            }
        }
    }

    nodes
}

pub fn parse_single_node(token: Token, app: &mut App) -> IsiNode {
    let mut node = IsiNode::EmptyNode;
    match token.t_type {
        IsiToken::INTEGER => {
            let expression = get_expression(app, None);
            let int_expression = parse_expression(app, &expression.0);
            node = IsiNode::IsiExpression(int_expression);
            app.index = expression.1;
        }
        IsiToken::STRING => {
            let expression = get_expression(app, None);
            let string_expression = parse_expression(app, &expression.0);
            node = IsiNode::IsiExpression(string_expression);
            app.index = expression.1;
        }
        IsiToken::TRUE | IsiToken::FALSE => {
            let expression = get_expression(app, None);
            let bool_expression = parse_expression(app, &expression.0);
            node = IsiNode::IsiExpression(bool_expression);
            app.index = expression.1;
        }
        IsiToken::VARIABLE => {
            let expression = get_expression(app, None);
            let variable_expression = parse_expression(app, &expression.0);
            node = IsiNode::IsiExpression(variable_expression);

            app.index = expression.1;
        }
        IsiToken::LPAREN => {
            let call_node = parse_call(app);
            node = call_node.0;
        }
        IsiToken::QUESTION => {
            let match_node = parse_match(app, false);
            node = match_node.0;
        }
        _ => {
            print_compile_error(&format!(
                "Unexpected token: `{}` with type `{:?}` [parse_single_node]",
                token.t_value, token.t_type
            ));
        }
    };
    node
}
