use crate::isi::{
    ast::ast::{App, DataType, MatchStatement},
    generator::gen_utils::generate_node,
    util::util::print_compile_error,
};

pub fn gen_match_statement(match_stmt: &MatchStatement, app: &mut App) -> String {
    let mut code = String::new();

    // TODO: uuugly
    let head = match_stmt.input.clone();
    let generated_head = &generate_node(*head, app);
    let head_node_name = app.generate_new_unique_name("_tmp_head_node");
    code += &format!(
        "{} {} = {}",
        match_stmt.input_type.to_c_string_type(),
        head_node_name,
        generated_head
    );

    for (i, pattern) in match_stmt.patterns.iter().enumerate() {
        let prefix = if i == 0 { "if" } else { "else if" };
        let val = pattern.pattern.t_value.as_ref();

        let condition = match (match_stmt.input_type, val) {
            (DataType::Bool, "true") => format!("{}", head_node_name),
            (DataType::Bool, "false") => format!("!{}", head_node_name),
            (_, "_") => "".to_string(), // Handle wildcard later
            _ => format!("{} == {}", head_node_name, val),
        };

        if val == "_" {
            code += "else {\n";
        } else {
            code += &format!("{} ({}) {{\n", prefix, condition);
        }

        if pattern.result.len() > 1 {
            print_compile_error("Match results with a length greater than 1 are not yet supported");
        }

        let result_code = generate_node(pattern.result[0].clone(), app);
        code += &format!("  {result_code} }} ")
    }

    code
}
