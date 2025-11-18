use crate::isi::ast::ast::{App, IsiValue};

pub fn parse(app: &mut App) -> Vec<IsiValue> {
    let nodes: Vec<IsiValue> = Vec::new();
    println!("Hello from parser.rs > Tokens: {}", app.tokens.len());
    return nodes;
}
