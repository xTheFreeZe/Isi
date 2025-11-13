pub enum Token {
    TypeString(String), // "Hello"
    TypeNumber(i64),    // 10

    Literal,     // Word
    SingleQuote, // ''
    DoubleQuote, // ""
    Dash,        // -
    ArrowR,      // >
    ArrowL,      // <
}

pub struct App {
    pub program_path: String,
}
