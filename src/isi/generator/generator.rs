use crate::isi::{
    ast::ast::{App, IsiNode},
    util::util::print_compile_error,
};

pub fn generator(app: &mut App) {
    while app.index < app.nodes.len() {
        let node = app.get_node();
        match node {
            IsiNode::IsiVariableDecl(variable_decl) => {
                let full_variable = app.get_variable_from_map(variable_decl.name.as_ref());
                let variable_body = *full_variable.v_node;
                let _variable_body_code = match variable_body {
                    IsiNode::IsiFunctionDecl(function_decl) => {
                        println!("Got  a function decl -> {}", function_decl.name)
                    }
                    IsiNode::IsiExpression(expression) => {
                        println!("Got  an expression -> {}", expression.e_value)
                    }
                    _ => {
                        print_compile_error(&format!(
                            "Unknown node in generator: {:#?}",
                            variable_body
                        ));
                    }
                };
            }
            _ => {
                print_compile_error(&format!("Unknown head node in generator: {:#?}", node));
            }
        }
        app.index += 1;
    }

    println!("Gen Code: \n{}", app.generated_code)
}
