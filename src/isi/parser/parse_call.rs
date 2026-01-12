use crate::isi::{
    ast::ast::{App, DataType, FunctionCall, IsiNode, IsiToken, Token},
    util::util::print_compile_error,
};

pub fn parse_call(app: &mut App) -> IsiNode {
    // Skip the `(`
    app.next();
    let mut call = FunctionCall::default();

    app.expect(IsiToken::VARIABLE);
    let function_name = app.get().t_value;

    if !app.function_table.contains_key(&function_name) {
        print_compile_error(&format!("Unknown function `{}`", function_name))
    }

    let function = app.get_function_from_map(&function_name);
    call.function = function.clone();

    app.next();

    let mut arguments: Vec<Token> = Vec::new();
    while app.get().t_type != IsiToken::RPAREN {
        let token = app.get();
        arguments.push(token);
        app.next();
    }

    let args_len = arguments.len();
    let mut function_params_len = 0;

    if let Some(params) = &function.params {
        function_params_len = params.len();
    }

    if args_len != function_params_len {
        print_compile_error(&format!(
            "Function `{}` expects {} argument(s), got {}",
            &function_name, function_params_len, args_len
        ));
    }

    if let Some(params) = &function.params {
        for (i, a) in arguments.iter().enumerate() {
            let expected = params[i].p_type;
            let got: DataType;
            if a.t_type == IsiToken::VARIABLE {
                let var = app.get_variable_from_map(&a.t_value);
                got = var.v_type;
            } else {
                got = a.t_type.to_data_type();
            }
            if got != expected {
                print_compile_error(&format!(
                    "Expected `{:?}`, found `{:?}` > Parameter `{}` in function `{}`",
                    expected, got, params[i].name, function.name
                ));
            }
        }
    }

    if function_name == "print" {
        // Das ist so ass, aber geht nur bis jetzt so
        let mut value = arguments[0].t_value.clone();
        if arguments[0].t_type == IsiToken::VARIABLE {
            let arg_var = app.get_variable_from_map(&value);
            match *arg_var.v_node {
                IsiNode::IsiExpression(expression) => {
                    value = expression.e_value;
                }
                _ => {}
            }
        }
        println!("{}", value)
    }

    // Skip the `)` to close the function call
    app.next();
    IsiNode::IsiFunctionCall(call)
}
