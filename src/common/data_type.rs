


pub enum DataType {
    String(String),
    Int(isize),
    Tag(String),
    Float(f64),
    List(Vec<DataType>)
}