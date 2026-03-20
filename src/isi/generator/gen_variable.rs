use crate::isi::{ast::ast::Expression, generator::gen_utils::gen_proper_type_code};

pub fn gen_variable_decl(expression: Expression, var_name: &str) -> String {
    let mut code = String::new();

    code += &expression.e_type.to_c_string_type();
    code += &format!(" {}", var_name);
    code += " = ";

    // TODO: This limits us to strings and numbers and things like functions as variables are not possible yet
    code += &gen_proper_type_code(&expression.e_value.as_ref(), expression.e_type);

    code += ";\n";
    code
}
