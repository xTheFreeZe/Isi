use crate::isi::{
    ast::ast::{App, DataType, MatchStatement},
    generator::gen_utils::generate_node,
    util::util::print_compile_error,
};

pub struct GeneratedMatchStatement {
    pub generated_code: String,
    pub generated_match_var_name: String,
    pub match_data_type: String,
}

pub fn gen_match_statement(
    match_stmt: &MatchStatement,
    app: &mut App,
    assign_to_var: bool,
) -> GeneratedMatchStatement {
    let mut code = String::new();

    // TODO: uuugly
    let head = match_stmt.input.clone();
    let generated_head = &generate_node(*head, app, false);
    let head_node_name = app.generate_new_unique_name("_tmp_head_node");
    code += &format!(
        "{} {} = {}",
        match_stmt.input_type.to_c_string_type(),
        head_node_name,
        generated_head
    );

    let match_type = match_stmt
        .patterns
        .first()
        .unwrap()
        .result_type
        .to_c_string_type();
    let match_assign_name = app.generate_new_unique_name("_tmp_match_assign");
    if assign_to_var {
        code += &format!("{} {}; \n", match_type, match_assign_name);
    }

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

        let result_code = generate_node(pattern.result[0].clone(), app, false);
        if assign_to_var {
            code += &format!("  {match_assign_name} = {result_code} }}\n")
        } else {
            code += &format!("  {result_code} }}\n ");
        }
    }

    GeneratedMatchStatement {
        generated_code: code,
        generated_match_var_name: match_assign_name,
        match_data_type: match_type,
    }
}
