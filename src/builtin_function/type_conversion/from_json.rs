use std::collections::HashMap;

use serde_json::Value;

use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn from_json(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;

    match p1 {
        DataType::String(s) => {
            let parsed: Value = serde_json::from_str(s).map_err(|_| {
                ChapError::runtime_with_msg(
                    executable.line_number,
                    format!("failed to parse string to map"),
                )
            })?;

            let result = DataType::try_from(parsed)
                .map_err(|e| ChapError::runtime_with_msg(executable.line_number, e))?;

            returns(runtime, executable, result)
        }
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!("can not convert {} to map", p1.type_name()),
            ));
        }
    }
}

impl TryFrom<Value> for DataType {
    type Error = String;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(DataType::String(s)),
            Value::Number(s) => {
                if s.is_f64() {
                    // safe to use here, as we checked for being a float
                    Ok(DataType::Float(s.as_f64().unwrap()))
                } else {
                    // safe to use here, as we checked for being an integer
                    // right now Chap does not support i64, so we cast to i32
                    Ok(DataType::Int(s.as_i64().unwrap() as i32))
                }
            }
            Value::Bool(b) => Ok(DataType::Bool(b)),
            Value::Array(a) => {
                let mut list = vec![];

                for item in a {
                    list.push(DataType::try_from(item)?);
                }

                Ok(DataType::List(list))
            }
            Value::Object(o) => {
                let mut map = HashMap::new();

                for (key, value) in o {
                    map.insert(key, DataType::try_from(value)?);
                }

                Ok(DataType::Map(map))
            }
            Value::Null => {
                // right now Chap doesnt support Null as a data type, so we just cast it to a string
                Ok(DataType::String("null".to_string()))
            }
            _ => Err("unsupported type in source json string".to_string()),
        }
    }
}
