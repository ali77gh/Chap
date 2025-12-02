use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataType {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    List(Vec<DataType>),
    Map(HashMap<String, DataType>),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(m) => write!(f, "{}", m.clone()),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(f_) => write!(f, "{}", f_),
            Self::Bool(b) => write!(f, "{}", b),
            Self::List(b) => {
                let a = b.iter().map(|x| x.to_string());
                let a: Vec<String> = a.collect();
                let i = a.join(" ");
                write!(f, "[{}]", i)
            }
            Self::Map(b) => {
                let mut a: Vec<_> = b.iter().collect();
                a.sort_by_key(|(k, _)| *k);
                let a: Vec<String> = a.iter().map(|(k, v)| format!("\"{}\": {}", k, v)).collect();
                let i = a.join(" ");
                write!(f, "{{{}}}", i)
            }
        }
    }
}

impl DataType {
    pub fn type_name(&self) -> String {
        match self {
            DataType::String(_) => "string".to_string(),
            DataType::Int(_) => "int".to_string(),
            DataType::Float(_) => "float".to_string(),
            DataType::Bool(_) => "boolean".to_string(),
            DataType::List(_) => "list".to_string(),
            DataType::Map(_) => "map".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_type_equal_test() {
        assert!(DataType::String("abcd".to_string()) == DataType::String("abcd".to_string()));
        assert!(DataType::String("abce".to_string()) != DataType::String("abcd".to_string()));
    }

    #[test]
    fn list_to_string() {
        assert_eq!(
            DataType::List(vec![
                DataType::Int(1),
                DataType::String("hello".to_string())
            ])
            .to_string(),
            "[1 hello]"
        );
    }

    #[test]
    fn map_to_string() {
        let mut map: HashMap<String, DataType> = HashMap::new();
        let mut inner: HashMap<String, DataType> = HashMap::new();

        inner.insert("a".to_string(), DataType::Int(1));

        map.insert("alpha".to_string(), DataType::Bool(true));
        map.insert("bravo".to_string(), DataType::Float(3.1415));
        map.insert("charlie".to_string(), DataType::Int(42));
        map.insert(
            "delta".to_string(),
            DataType::List(vec![
                DataType::Int(1),
                DataType::String("hello".to_string()),
            ]),
        );
        map.insert("echo".to_string(), DataType::Map(inner));
        map.insert(
            "foxtrot".to_string(),
            DataType::String("String".to_string()),
        );

        assert_eq!(
            DataType::Map(map).to_string(),
            "{\"alpha\": true \"bravo\": 3.1415 \"charlie\": 42 \"delta\": [1 hello] \"echo\": {\"a\": 1} \"foxtrot\": String}"
        );
    }
}
