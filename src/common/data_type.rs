

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    List(Vec<DataType>)
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            Self::String(m) => m.clone(),
            Self::Int(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::List(b) => {
                let a = b.iter().map(|x|x.to_string());
                let a: Vec<String> = a.collect();
                let i = a.join(" ");
                format!("[{}]",i)
            },
        }
    }
}

impl DataType {
    
    pub fn type_name(&self) -> String{
        match self {
            DataType::String(_) => "string".to_string(),
            DataType::Int(_) => "int".to_string(),
            DataType::Float(_) => "float".to_string(),
            DataType::Bool(_) => "boolean".to_string(),
            DataType::List(_) => "list".to_string(),
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

    #[test]
    fn list_to_string(){
        assert_eq!(
            DataType::List(vec![DataType::Int(1), DataType::String("hello".to_string())]).to_string(),
            "[1 hello]"
        );
    }
}