
use crate::common::{chunk::Chunk, param::Param, data_type::DataType};
use crate::common::errors::{Result, ChapError};

pub fn chunk_detector(chunk_str: String) -> Result<Chunk>{
    
    let chunk_str = chunk_str.trim();

    let r= match chunk_str.chars().nth(0).unwrap_or(' ') {
        '$' | '@' | '"' => Chunk::Params(params_parser(&chunk_str)?),
        c => {
            if c.is_digit(10){
                Chunk::Params(params_parser(&chunk_str)?)
            }else{
                Chunk::Function { name: chunk_str.to_string() }
            }
        }
    };
    Ok(r)
}

fn params_parser(chunk_str: &str) -> Result<Vec<Param>>{
    let params_str = chunk_str.split(",").map(|x|{x.trim()});

    let mut result:Vec<Param> = Vec::new();
    for param_str in params_str {
        result.push(param_parser(&param_str)?)
    }

    Ok(result)
}

fn param_parser(param: &str) -> Result<Param>{
        let parsed_param = match param.chars().nth(0).unwrap_or(' ') {
            '$' => Param::Variable((&param[1..]).to_string()),
            '@' => Param::Tag((&param[1..]).to_string()),
            '"' => {
                let len = param.len();
                if !(&param.ends_with("\"")){
                    return Err(ChapError::syntax_with_msg(0, "string should ends with \"".to_string()));
                } 
                Param::Value( DataType::String((&param[1..len-1]).to_string()))
            },
            _=> {
                if param.contains("."){
                    if let Ok(float_value) = param.parse() {
                        Param::Value(DataType::Float(float_value))
                    } else {
                        Err(ChapError::syntax_with_msg(0, "parsing float".to_string()))?
                    }
                } else {
                    if let Ok(float_value) = param.parse() {
                        Param::Value(DataType::Int(float_value))
                    } else {
                        Err(ChapError::syntax_with_msg(0, "parsing int".to_string()))?
                    }
                }
            }
        };
        Ok(parsed_param)
}


#[cfg(test)]
mod tests {
    use crate::parser::chunk_detector::*;

    #[test]
    fn param_parser_empty() {
        let _ = param_parser("");
    }

    #[test]
    fn param_parser_variable() {
        assert_eq!(
            param_parser("$name"),
            Ok(Param::Variable("name".to_string()))
        );
    }

    #[test]
    fn param_parser_tag() {
        assert_eq!(
            param_parser("@name"),
            Ok(Param::Tag("name".to_string()))
        );
    }

    #[test]
    fn param_parser_string() {
        assert_eq!(
            param_parser("\"name\""),
            Ok(Param::Value(DataType::String("name".to_string())))
        );

        assert_eq!(
            param_parser("\"name"),
            Err(ChapError::syntax_with_msg(0, "string should ends with \"".to_string()))
        );
    }

    #[test]
    fn param_parser_integer() {
        assert_eq!(
            param_parser("2"),
            Ok(Param::Value(DataType::Int(2)))
        );

        assert_eq!(
            param_parser("2h"),
            Err(ChapError::syntax_with_msg(0, "parsing int".to_string()))
        );
    }

    #[test]
    fn param_parser_float() {
        assert_eq!(
            param_parser("3.14"),
            Ok(Param::Value(DataType::Float(3.14)))
        );

        assert_eq!(
            param_parser(".3.14"),
            Err(ChapError::syntax_with_msg(0, "parsing float".to_string()))
        );
    }

    #[test]
    fn params_test(){
        assert_eq!(
            params_parser("  $n1,\"s1\"   ,2,3.14  "),
            Ok(vec![ 
                Param::Variable("n1".to_string()),
                Param::Value(DataType::String("s1".to_string())),
                Param::Value(DataType::Int(2)),
                Param::Value(DataType::Float(3.14)),
            ])
        );
    }

    #[test]
    fn chunck_detector_test(){
        assert_eq!(
            chunk_detector("println".to_string()),
            Ok(Chunk::Function { name: "println".to_string() })
        );
    }
}