use crate::isi::{
    ast::ast::{App, DataType, IsiNode, IsiToken, MatchPattern, MatchStatement},
    parser::{
        parse_function::retrieve_last_data_type,
        parser::{parse_single_node, parse_until},
    },
    util::util::print_compile_error,
};

pub fn parse_match(app: &mut App, assign_to_var: bool) -> (IsiNode, DataType) {
    let mut match_stmt = MatchStatement::default();

    // Skip the '?'
    app.next();

    let input: Vec<IsiNode>;
    let input_type: DataType;
    let head_type: DataType;

    if app.get().t_type != IsiToken::LPAREN {
        input = vec![parse_single_node(app.get(), app)];
        input_type = retrieve_last_data_type(&input, app);
        head_type = evaluate_head_type(&input_type);
    } else {
        app.expect(IsiToken::LPAREN);
        app.next();
        input = parse_until(app, IsiToken::RPAREN);
        input_type = retrieve_last_data_type(&input, app);
        head_type = evaluate_head_type(&input_type);
        app.expect(IsiToken::RPAREN);
        app.next();
    }

    if input_type == DataType::Nil {
        print_compile_error("Can not match on a value with type `nil`");
    }

    if input.len() > 1 {
        print_compile_error("Not a valid match expression");
    }

    let predicted_arms = calculate_match_arms(&head_type);

    match_stmt.input = Box::new(input[0].clone());
    match_stmt.input_type = input_type;

    let mut latest_pattern_type = DataType::NONE;
    while app.get().t_type != IsiToken::QUESTION {
        let pattern = app.get();
        let pattern_type;
        if pattern.t_value.as_ref() != "_" {
            if pattern.t_type.to_data_type() != head_type {
                print_compile_error(&format!(
                    "Isi Type Error: Arm pattern `{}` with type `{}` does not match the expression type `{}`",
                    &pattern.t_value,
                    &pattern.t_type.to_data_type(),
                    head_type
                ));
            }
            pattern_type = pattern.t_type.to_data_type();
        } else {
            pattern_type = DataType::Nil;
        }

        app.next();
        app.expect(IsiToken::ARROW);
        app.next();
        app.expect(IsiToken::LPAREN);
        app.next();

        let result = parse_until(app, IsiToken::RPAREN);
        let result_type = retrieve_last_data_type(&result, app);

        if latest_pattern_type == DataType::NONE {
            latest_pattern_type = result_type
        } else {
            if latest_pattern_type != result_type {
                print_compile_error(&format!(
                    "Match arms have different types: `{latest_pattern_type}` and `{result_type}`"
                ));
            }
        }

        let arm = MatchPattern {
            pattern: pattern.clone(),
            pattern_type: pattern_type,
            result,
            result_type,
        };
        match_stmt.patterns.push(arm);

        app.expect(IsiToken::RPAREN);
        app.next();
    }

    // Jump over the `?`
    app.next();

    if let Some(number_of_arms) = predicted_arms
        && number_of_arms != -1
    {
        if match_stmt.patterns.len() as i32 != number_of_arms {
            print_compile_error(&format!(
                "Insufficient number of match arms > Expected {number_of_arms}, got {}",
                match_stmt.patterns.len()
            ));
        }
    }

    if latest_pattern_type == DataType::Nil && assign_to_var {
        print_compile_error(
            "Can not assign match statement to variable, as no arm returns anything",
        );
    }
    (IsiNode::IsiMatchStatement(match_stmt), latest_pattern_type)
}

/// Calculates the number of match patterns according to the type.
///
/// -1 -> The compiler does not know > Infinite options
///
/// {Some(x) | x > 0} -> x number of arms
///
/// None -> No arms
fn calculate_match_arms(input: &DataType) -> Option<i32> {
    match input {
        DataType::Int => Some(-1),
        DataType::Float => Some(-1),
        DataType::String => Some(-1),
        DataType::Bool => Some(2),
        _ => None,
    }
}

/// Evaluates the head type of the match statement:
///
/// isBlue (boolean) -> true / false
///
/// Strings can only match on strings
///
/// Ints can only match on ints...
fn evaluate_head_type(input: &DataType) -> DataType {
    match input {
        DataType::Int => DataType::Int,
        DataType::Float => DataType::Float,
        DataType::String => DataType::String,
        DataType::Bool => DataType::Bool,
        _ => DataType::NONE,
    }
}
