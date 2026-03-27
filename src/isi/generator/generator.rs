use crate::isi::{
    ast::ast::{App, IsiNode},
    generator::{
        gen_extras::gen_match_statement,
        gen_function::{gen_builtin_function, gen_function, gen_function_call},
        gen_variable::{gen_function_call_variable, gen_simple_variable},
    },
    parser::expression::get_variable,
    util::util::print_compile_error,
};

pub fn generator(app: &mut App) {
    let mut main_code = String::new();
    // TODO: Track imports and see if this is even needed...!
    app.generated_code += "#include <stdio.h>\n";
    app.generated_code += "#include <string.h>\n";
    while app.index < app.nodes.len() {
        let node = app.get_node();
        match node {
            IsiNode::IsiVariableDecl(variable_decl) => {
                let full_variable = get_variable(variable_decl.name.as_ref(), app);
                let variable_body = *full_variable.v_node;
                match variable_body {
                    IsiNode::IsiFunctionDecl(function_decl) => {
                        let full_function = app.get_function_from_map(&function_decl.name);
                        let generated_function: String;
                        if full_function.is_builtin {
                            generated_function = gen_builtin_function(&full_function);
                        } else {
                            generated_function = gen_function(&full_function, app);
                        }

                        app.generated_code += &generated_function;
                    }
                    IsiNode::IsiExpression(expression) => {
                        app.generated_code +=
                            &gen_simple_variable(expression, &full_variable.v_name.as_ref());
                    }
                    IsiNode::IsiFunctionCall(call) => {
                        // Eventually we need to move variables with no function to the top:
                        // int result;
                        // Then, in main:
                        // result -> (plus 2 2)
                        // app.generated_code +=
                        //     &format!("{} ", call.function.return_type.to_c_string_type());
                        // app.generated_code += &format!("{};", &full_variable.v_name.as_ref());
                        main_code +=
                            &gen_function_call_variable(&call, &full_variable.v_name.as_ref())
                    }
                    IsiNode::IsiMatchStatement(match_stmt) => {
                        let match_code = gen_match_statement(&match_stmt, app, true);
                        main_code += &match_code.generated_code;
                        main_code += &format!(
                            "{} {} = {}; \n",
                            match_code.match_data_type,
                            variable_decl.name.as_ref(),
                            match_code.generated_match_var_name
                        )
                    }
                    _ => {
                        print_compile_error(&format!(
                            "Unknown node in generator: {:#?}",
                            variable_body
                        ));
                    }
                };
            }
            IsiNode::IsiFunctionCall(function_call) => {
                let generated_call = gen_function_call(&function_call);
                main_code += &generated_call;
            }
            IsiNode::IsiMatchStatement(match_stmt) => {
                let generadted_match = gen_match_statement(&match_stmt, app, false);
                main_code += &generadted_match.generated_code;
            }
            IsiNode::EmptyNode => {
                app.index += 1;
                continue;
            }
            _ => {
                print_compile_error(&format!("Unknown head node in generator: {:#?}", node));
            }
        }
        app.index += 1;
    }

    let complete_main = format!("int main() {{\n {} }}", main_code);
    app.generated_code += &complete_main;
}
