use crate::isi::{
    ast::ast::{
        App, DataType, Function, FunctionDecl, FunctionParam, IsiNode, IsiToken, Variable,
        VariableDecl,
    },
    parser::{
        expression::{get_expression, parse_expression},
        parse_call::parse_call,
        parse_function::parse_function,
    },
    util::util::print_compile_error,
};

pub fn parse(app: &mut App) {
    // Todo: This goes away once we have a std of sorts
    let print_function = Function {
        name: String::from("print"),
        function_body: None,
        params: Some(vec![FunctionParam {
            name: String::from("x"),
            p_type: DataType::String,
        }]),
        return_type: DataType::NONE,
    };
    let decl = FunctionDecl {
        name: String::from("print"),
    };
    app.push_node(IsiNode::IsiFunctionDecl(decl));
    app.push_function_into_map(print_function);
    while app.index < app.tokens.len() {
        let token = app.get();

        match token.t_type {
            IsiToken::VARIABLE => {
                let node = parse_variable(app);
                app.nodes.push(node);
            }
            IsiToken::LPAREN => {
                let node = parse_call(app);
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
    app.current_var_str = token.t_value;
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
            let (function_node, function_type) =
                if next.t_type == IsiToken::LBRACKET || next.t_type == IsiToken::COLON {
                    app.next();
                    parse_function(app)
                } else {
                    (IsiNode::EmptyNode, DataType::NONE)
                };
            var.v_type = function_type;
            function_node
        }
        // x -> 10
        // This arm is used when you assign a variable a number
        IsiToken::INTEGER => {
            let expression = get_expression(app);
            let int_expression = parse_expression(&expression.0);
            app.index = expression.1;
            var.v_type = DataType::Int;
            IsiNode::IsiExpression(int_expression)
        }
        IsiToken::STRING => {
            let expression = get_expression(app);
            let string_expression = parse_expression(&expression.0);
            app.index = expression.1;
            var.v_type = DataType::String;
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
    let var_decl = VariableDecl {
        name: var.v_name.clone(),
    };
    app.push_variable_into_map(var);
    IsiNode::IsiVariableDecl(var_decl)
}
