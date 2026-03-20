use crate::isi::{
    ast::ast::{App, Function, FunctionCall, FunctionCallArgument, FunctionParam},
    generator::{gen_utils::gen_proper_type_code, gen_variable::gen_variable_decl},
    parser::expression::get_variable,
    util::util::print_compile_error,
};

fn gen_function_params(params: &Option<Vec<FunctionParam>>) -> String {
    let mut code = String::new();
    if let Some(p) = params {
        for (index, param) in p.iter().enumerate() {
            let param_type = param.p_type.to_c_string_type();
            code += &format!("{} {}", param_type, param.name.as_ref());

            if index + 1 != p.len() {
                code += ", "
            }
        }
    }
    code
}

fn gen_call_args(args: &Option<Vec<FunctionCallArgument>>) -> String {
    let mut code = String::new();

    if let Some(a) = args {
        for (index, arg) in a.iter().enumerate() {
            let argument = if arg.is_variable {
                arg.name.as_ref()
            } else {
                &gen_proper_type_code(arg.name.as_ref(), arg.a_type)
            };
            code += &argument;
            if index + 1 != a.len() {
                code += ", "
            }
        }
    }
    code
}

fn gen_function_sig(function: &Function) -> String {
    let mut code = String::new();

    let fn_return_type = function.return_type.to_c_string_type();

    code += &fn_return_type;
    code += " ";
    code += function.name.as_ref();
    code += " (";
    code += &gen_function_params(&function.params);
    code += ") ";

    code
}

pub fn gen_builtin_function(function: &Function) -> String {
    let mut code = String::new();

    code += &gen_function_sig(function);
    code += "{\n";
    code += &function.builtin_code;
    code += "\n}\n";

    code
}

pub fn gen_function(function: &Function, app: &App) -> String {
    let mut code = String::new();

    code += &gen_function_sig(function);
    code += "{\n";
    if function.function_body.is_some() {
        code += &gen_function_body(function, app);
    }
    code += "\n}\n";

    code
}

pub fn gen_function_call(call: &FunctionCall) -> String {
    let mut code = String::new();

    code += call.function.name.as_ref();
    code += "(";
    code += &gen_call_args(&call.arguments);
    code += ");\n";

    code
}

/// Make sure the function has a body before calling this function
fn gen_function_body(function: &Function, app: &App) -> String {
    let mut code = String::new();
    let body = function.function_body.as_ref().unwrap();
    for node in body {
        match node {
            crate::isi::ast::ast::IsiNode::IsiVariableDecl(var_decl) => {
                let full = get_variable(&var_decl.name.as_ref(), app);
                let variable_body = *full.v_node;
                match variable_body {
                    crate::isi::ast::ast::IsiNode::IsiExpression(expression) => {
                        code += &gen_variable_decl(expression, &full.v_name.as_ref());
                    }
                    _ => {
                        print_compile_error(&format!(
                            "Unknown node in body of variable [currently in variable {}]: {:#?}",
                            full.v_name.as_ref(),
                            node,
                        ));
                    }
                }
            }
            crate::isi::ast::ast::IsiNode::IsiFunctionCall(function_call) => {
                code += &gen_function_call(function_call);
            }
            _ => {
                print_compile_error(&format!("Unknown node in function body: {:#?}", node));
            }
        }
    }

    code
}
