

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    String(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    // List(Vec<DataType>)
}