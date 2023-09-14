
#[derive(Debug)]
pub enum ExecutableLine{
    // comments and empty lines
    Noting, 

    // parser cant parse this line
    SyntaxError(String),

    // Tag(String),

    // BuiltinFunction(String, Vec<Variable>, Variable),

    // Function(String, Vec<Variable>, Variable),
}
