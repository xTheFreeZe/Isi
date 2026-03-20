use crate::isi::{
    ast::ast::{App, IsiNode},
    generator::{
        gen_function::{gen_builtin_function, gen_function, gen_function_call},
        gen_variable::gen_variable_decl,
    },
    parser::expression::get_variable,
    util::util::print_compile_error,
};

pub fn generator(app: &mut App) {
    let mut main_code = String::new();
    // TODO: Track imports and see if this is even needed...!
    app.generated_code += "#include <stdio.h>\n";
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
                            generated_function = gen_function(&full_function);
                        }

                        app.generated_code += &generated_function;
                    }
                    IsiNode::IsiExpression(expression) => {
                        app.generated_code +=
                            &gen_variable_decl(expression, &full_variable.v_name.as_ref());
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
            _ => {
                print_compile_error(&format!("Unknown head node in generator: {:#?}", node));
            }
        }
        app.index += 1;
    }

    let complete_main = format!("int main() {{\n {} \n}}", main_code);
    app.generated_code += &complete_main;
}
