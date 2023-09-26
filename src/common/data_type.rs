

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    // List(Vec<DataType>)
}