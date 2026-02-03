use crate::isi::ast::ast::{Function, FunctionParam};

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
    code += "\n}";

    code
}
