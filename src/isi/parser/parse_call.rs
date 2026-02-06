use std::{process::exit, sync::Arc};

use crate::isi::{
    ast::ast::{App, DataType, FunctionCall, FunctionCallArgument, IsiNode, IsiToken, Token},
    parser::expression::get_variable,
    util::util::print_compile_error,
};

pub fn parse_call(app: &mut App) -> IsiNode {
    // Skip the `(`
    app.next();
    let mut call = FunctionCall::default();

    app.expect(IsiToken::VARIABLE);
    let mut function_name = app.get().t_value;

    //TODO: This is really primitive and should be done better...
    // It's also very limiting because this only allows for variables and or primitive data types
    // Plus, we essentialy do the type checking twice...
    let mut peek_next_type = app.peek_next().t_type;
    if function_name.as_ref() == "print" {
        if peek_next_type == IsiToken::RPAREN {
            print_compile_error("Function `print` expects 1 argument(s), got 0");
        }
        if peek_next_type == IsiToken::VARIABLE {
            peek_next_type = get_variable(&app.peek_next().t_value.as_ref(), app)
                .v_type
                .to_token_type();
        }
        function_name = match peek_next_type {
            IsiToken::STRING => Arc::from("print_string"),
            IsiToken::INTEGER => Arc::from("print_int"),
            _ => {
                print_compile_error(&format!(
                    "Argument of type `{:?}` is not valid for function `print`",
                    peek_next_type
                ));
                exit(1);
            }
        }
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

    let mut call_arguments: Vec<FunctionCallArgument> = Vec::new();
    if let Some(params) = &function.params {
        for (i, a) in arguments.iter().enumerate() {
            let expected = params[i].p_type;
            let got: DataType;
            if a.t_type == IsiToken::VARIABLE {
                let var = get_variable(&a.t_value, app);
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
            let call_argument = FunctionCallArgument {
                name: arguments[i].t_value.clone(),
                a_type: got,
            };
            call_arguments.push(call_argument);
        }
    }

    if !call_arguments.is_empty() {
        call.arguments = Some(call_arguments);
    }

    // Skip the `)` to close the function call
    app.next();
    IsiNode::IsiFunctionCall(call)
}
