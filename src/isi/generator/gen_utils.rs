use crate::isi::{
    ast::ast::{App, DataType, IsiNode},
    generator::{
        gen_extras::gen_match_statement, gen_function::gen_function_call,
        gen_variable::gen_simple_variable,
    },
    parser::expression::get_variable,
    util::util::print_compile_error,
};

pub fn gen_proper_type_code(value: &str, data_type: DataType) -> String {
    match data_type {
        DataType::String => {
            format!("\"{}\"", value)
        }
        DataType::Int => String::from(value),
        DataType::Bool => {
            if value == "true" {
                String::from("1")
            } else {
                String::from("0")
            }
        }
        _ => {
            todo!("gen_proper_type_code() not yet implemented for {data_type:?}")
        }
    }
}

pub fn generate_node(node: IsiNode, app: &mut App, return_node: bool) -> String {
    let mut code = String::new();

    match node {
        crate::isi::ast::ast::IsiNode::IsiVariableDecl(ref var_decl) => {
            let full = get_variable(&var_decl.name.as_ref(), app);
            let variable_body = *full.v_node;
            match variable_body {
                crate::isi::ast::ast::IsiNode::IsiExpression(expression) => {
                    code += &gen_simple_variable(expression, &full.v_name.as_ref());
                }
                crate::isi::ast::ast::IsiNode::IsiFunctionCall(call) => {
                    let call_code = &gen_function_call(&call);

                    code += &format!(
                        "{} {} = {}",
                        call.function.return_type.to_c_string_type(),
                        var_decl.name,
                        call_code
                    );
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
            code += &gen_function_call(&function_call);
        }
        crate::isi::ast::ast::IsiNode::IsiMatchStatement(match_stmt) => {
            let stmt_code = &gen_match_statement(&match_stmt, app, false);
            code += &stmt_code.generated_code;
        }
        crate::isi::ast::ast::IsiNode::IsiExpression(expr) => {
            if return_node {
                code += "return";
            }
            code += &format!(
                " {};\n",
                gen_proper_type_code(expr.e_value.as_ref(), expr.e_type)
            );
        }
        _ => {
            print_compile_error(&format!("Unknown node in function body: {:#?}", node));
        }
    }

    code
}
