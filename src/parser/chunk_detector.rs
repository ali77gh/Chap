
use crate::common::{chunk::Chunk, param::Param, data_type::DataType};
use crate::common::errors::{Result, ChapError};
use crate::common::splitter::string_safe_split;

pub fn chunk_detector(chunk_str: String, line_number: u32) -> Result<Chunk>{
    
    let chunk_str = chunk_str.trim();

    if chunk_str.starts_with("true") || chunk_str.starts_with("false"){
        return Ok(Chunk::Params(params_parser(chunk_str, line_number)?));
    }

    let r= match chunk_str.chars().next().unwrap_or(' ') {
        '$' | '@' | '"' | '-' | '+' | '[' => Chunk::Params(params_parser(chunk_str, line_number)?),
        c => {
            if c.is_ascii_digit(){
                Chunk::Params(params_parser(chunk_str, line_number)?)
            }else{
                Chunk::Function(chunk_str.to_string())
            }
        }
    };
    Ok(r)
}

fn params_parser(chunk_str: &str,line_number: u32) -> Result<Vec<Param>>{
    let temp = string_safe_split(chunk_str, ",".to_string());
    let params_str: Vec<&str> = temp.iter().map(|x|{x.trim()}).collect();

    let mut result:Vec<Param> = Vec::new();
    for param_str in params_str {
        result.push(param_parser(param_str,line_number)?)
    }

    Ok(result)
}

fn param_parser(param: &str, line_number: u32) -> Result<Param>{

        if param == "true" { return Ok(Param::Value(DataType::Bool(true))); }
        if param == "false" { return Ok(Param::Value(DataType::Bool(false))); }

        let parsed_param = match param.chars().next().unwrap_or(' ') {
            '$' => Param::Variable((param[1..]).to_string()),
            '@' => Param::Tag((param[1..]).to_string()),
            '"' => {
                let len = param.len();
                if !(&param.ends_with('\"')){
                    return Err(ChapError::syntax_with_msg(line_number, "string should ends with \"".to_string()));
                } 
                Param::Value( DataType::String((param[1..len-1]).to_string()))
            },
            '[' => {
                let mut list: Vec<DataType> = vec![];
                if !(&param.ends_with(']')){
                    return Err(ChapError::syntax_with_msg(line_number, "list should ends with ]".to_string()));
                }
                let param = &param[1..param.len()-1];
                let items = param.split(' ').map(|token| param_parser(token, line_number));
                for item in items{
                    let item = item?;
                    match item {
                        Param::Tag(_) => return Err(ChapError::syntax_with_msg(line_number,"tags cant be in list".to_string())),
                        Param::Variable(_) => return Err(ChapError::syntax_with_msg(line_number,"variables cant be in list".to_string())),
                        Param::Value(v) => list.push(v),
                    }
                }
                Param::Value(DataType::List(list))
            }
            _=> {
                if param.contains('.'){
                    if let Ok(float_value) = param.parse() {
                        Param::Value(DataType::Float(float_value))
                    } else {
                        Err(ChapError::syntax_with_msg(line_number, "parsing float".to_string()))?
                    }
                } else if let Ok(float_value) = param.parse() {
                    Param::Value(DataType::Int(float_value))
                } else {
                    Err(ChapError::syntax_with_msg(line_number, "parsing int".to_string()))?
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
        let _ = param_parser("",0);
    }

    #[test]
    fn param_parser_variable() {
        assert_eq!(
            param_parser("$name",0),
            Ok(Param::Variable("name".to_string()))
        );
    }

    #[test]
    fn param_parser_tag() {
        assert_eq!(
            param_parser("@name",0),
            Ok(Param::Tag("name".to_string()))
        );
    }

    #[test]
    fn param_parser_string() {
        assert_eq!(
            param_parser("\"name\"",1),
            Ok(Param::Value(DataType::String("name".to_string())))
        );

        assert_eq!(
            param_parser("\"name",1),
            Err(ChapError::syntax_with_msg(1, "string should ends with \"".to_string()))
        );
    }

    #[test]
    fn param_parser_integer() {
        assert_eq!(
            param_parser("2",1),
            Ok(Param::Value(DataType::Int(2)))
        );

        assert_eq!(
            param_parser("2h",1),
            Err(ChapError::syntax_with_msg(1, "parsing int".to_string()))
        );
    }

    #[test]
    fn param_parser_float() {
        assert_eq!(
            param_parser("1.5",1),
            Ok(Param::Value(DataType::Float(1.5)))
        );

        assert_eq!(
            param_parser(".1.5",1),
            Err(ChapError::syntax_with_msg(1, "parsing float".to_string()))
        );
    }

    #[test]
    fn chunk_parser_negetive_number() {
        assert_eq!(
            chunk_detector("-1.5".to_string(),1),
            Ok(Chunk::Params(vec![Param::Value(DataType::Float(-1.5))]))
        );
        assert_eq!(
            chunk_detector("-3".to_string(),1),
            Ok(Chunk::Params(vec![Param::Value(DataType::Int(-3))]))
        );
    }

    #[test]
    fn params_test(){
        assert_eq!(
            params_parser("  $n1,\"s1\"   ,2,1.5  ",0),
            Ok(vec![ 
                Param::Variable("n1".to_string()),
                Param::Value(DataType::String("s1".to_string())),
                Param::Value(DataType::Int(2)),
                Param::Value(DataType::Float(1.5)),
            ])
        );
    }

    #[test]
    fn chunk_detector_test(){
        assert_eq!(
            chunk_detector("print".to_string(),0),
            Ok(Chunk::Function("print".to_string()))
        );
    }

    #[test]
    fn comma_in_quotation_error(){
        assert_eq!(
            chunk_detector("\"ali, majid\"".to_string(), 0),
            Ok(Chunk::Params(vec![Param::Value(DataType::String("ali, majid".to_string()))])) 
        )//this is single string param 
    } 


    #[test]
    fn bool_parser(){
        assert_eq!(
            chunk_detector("true".to_string(), 1),
            Ok(Chunk::Params(vec![Param::Value(DataType::Bool(true))]) )
        );
        assert_eq!(
            chunk_detector("false".to_string(), 1),
            Ok(Chunk::Params(vec![Param::Value(DataType::Bool(false))]) )
        );
    }

    #[test]
    fn list_parser(){
        assert_eq!(
            params_parser("[1 2 3], 3", 1),
            Ok(vec![
                Param::Value(DataType::List(vec![DataType::Int(1), DataType::Int(2),DataType::Int(3)])),
                Param::Value(DataType::Int(3)),
            ])
        );
    }
}