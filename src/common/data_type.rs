

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    // List(Vec<DataType>)
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            Self::String(m) => m.clone(),
            Self::Int(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Bool(b) => b.to_string(),
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn data_type_equal_test(){
        assert!(
            DataType::String("abcd".to_string()) == DataType::String("abcd".to_string())
        );
        assert!(
            DataType::String("abce".to_string()) != DataType::String("abcd".to_string())
        );
    }
}