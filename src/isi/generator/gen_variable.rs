use crate::isi::{
    ast::ast::{Expression, FunctionCall},
    generator::{gen_function::gen_function_call, gen_utils::gen_proper_type_code},
};

pub fn gen_simple_variable(expression: Expression, var_name: &str) -> String {
    let mut code = String::new();

    code += &format!("{} ", expression.e_type.to_c_string_type());
    code += var_name;
    code += " = ";

    code += &gen_proper_type_code(&expression.e_value.as_ref(), expression.e_type);

    code += ";\n";
    code
}

pub fn gen_function_call_variable(call: &FunctionCall, var_name: &str) -> String {
    let mut code = String::new();

    code += &format!("{} ", call.function.return_type.to_c_string_type());
    code += var_name;
    code += " = ";
    code += &gen_function_call(call);

    code
}
