

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    // List(Vec<DataType>)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn data_type_equal_test(){
        assert_eq!(
            DataType::String("abcd".to_string()) == DataType::String("abcd".to_string()),
            true
        );
        assert_eq!(
            DataType::String("abce".to_string()) == DataType::String("abcd".to_string()),
            false
        );
    }
}