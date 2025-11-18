use crate::isi::ast::ast::{App, IsiToken::STRING, IsiValue};

pub fn parse(app: &mut App) -> Vec<IsiValue> {
    let all_nodes: Vec<IsiValue> = Vec::new();
    println!("Hello from parser.rs > Tokens: {}", app.tokens.len());

    let mut nodes = app.tokens.iter().peekable();
    while let Some(&n) = nodes.peek() {
        match n.t_type {
            STRING(_) => {
                println!("Got a string -> {}", n.t_value);
                nodes.next();
            }
            _ => {
                nodes.next();
            }
        }
    }
    return all_nodes;
}
