use crate::isi::{
    ast::ast::{
        App, IsiNode,
        IsiToken::{ARROW, STRING},
        Variable,
    },
    utils::utils::print_compile_error,
};

pub fn parse(app: &mut App) -> Vec<IsiNode> {
    let all_nodes: Vec<IsiNode> = Vec::new();

    while app.index < app.tokens.len() {
        let token = app.get();

        match token.t_type {
            STRING(_) => {
                let node = parse_variable(app);
                app.nodes.push(node);
            }
            _ => {}
        }

        app.next();
    }
    return all_nodes;
}

fn parse_variable(app: &mut App) -> IsiNode {
    let mut var = Variable::default();

    let mut token = app.get();
    println!("Got variable with name: {}", &token.t_value);
    var.v_value = token.t_value.to_string();
    app.next();

    token = app.get();
    if token.t_type != ARROW {
        print_compile_error(format!("Unexpected `{}` > Expected `->`", token.t_value));
    }

    let node = IsiNode::IsiVariable(var);
    return node;
}
