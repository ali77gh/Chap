
use crate::common::{chunk::Chunk, param::Param, data_type::DataType};
use crate::common::errors::{Result, ChapError};

pub fn chunk_detector(chunk_str: String, line_number: u32) -> Result<Chunk>{
    
    let chunk_str = chunk_str.trim();

    if chunk_str.starts_with("true") || chunk_str.starts_with("false"){
        return Ok(Chunk::Params(params_parser(&chunk_str, line_number)?));
    }

    let r= match chunk_str.chars().nth(0).unwrap_or(' ') {
        '$' | '@' | '"' => Chunk::Params(params_parser(&chunk_str, line_number)?),
        c => {
            if c.is_digit(10){
                Chunk::Params(params_parser(&chunk_str, line_number)?)
            }else{
                Chunk::Function(chunk_str.to_string())
            }
        }
    };
    Ok(r)
}

fn params_parser(chunk_str: &str,line_number: u32) -> Result<Vec<Param>>{
    let temp = comma_spliter(chunk_str);
    let params_str = temp.iter().map(|x|{x.trim()});

    let mut result:Vec<Param> = Vec::new();
    for param_str in params_str {
        result.push(param_parser(&param_str,line_number)?)
    }

    Ok(result)
}

fn param_parser(param: &str, line_number: u32) -> Result<Param>{

        if param == "true" { return Ok(Param::Value(DataType::Bool(true))); }
        if param == "false" { return Ok(Param::Value(DataType::Bool(false))); }

        let parsed_param = match param.chars().nth(0).unwrap_or(' ') {
            '$' => Param::Variable((&param[1..]).to_string()),
            '@' => Param::Tag((&param[1..]).to_string()),
            '"' => {
                let len = param.len();
                if !(&param.ends_with("\"")){
                    return Err(ChapError::syntax_with_msg(line_number, "string should ends with \"".to_string()));
                } 
                Param::Value( DataType::String((&param[1..len-1]).to_string()))
            },
            _=> {
                if param.contains("."){
                    if let Ok(float_value) = param.parse() {
                        Param::Value(DataType::Float(float_value))
                    } else {
                        Err(ChapError::syntax_with_msg(line_number, "parsing float".to_string()))?
                    }
                } else {
                    if let Ok(float_value) = param.parse() {
                        Param::Value(DataType::Int(float_value))
                    } else {
                        Err(ChapError::syntax_with_msg(line_number, "parsing int".to_string()))?
                    }
                }
            }
        };
        Ok(parsed_param)
}

fn comma_spliter(inp: &str) -> Vec<&str>{
    let mut result: Vec<&str> = vec![];

    let mut quotations_on_left=0;
    let mut last_seen = 0;
    for (i,ch) in inp.chars().enumerate(){
        if ch == '\"'{
            quotations_on_left+=1;
        }
        if ch == ',' && quotations_on_left%2==0{
            result.push(&inp[last_seen..i]);
            last_seen=i+1
        }
    }
    result.push(&inp[last_seen..]);

    result
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
            param_parser("3.14",1),
            Ok(Param::Value(DataType::Float(3.14)))
        );

        assert_eq!(
            param_parser(".3.14",1),
            Err(ChapError::syntax_with_msg(1, "parsing float".to_string()))
        );
    }

    #[test]
    fn params_test(){
        assert_eq!(
            params_parser("  $n1,\"s1\"   ,2,3.14  ",0),
            Ok(vec![ 
                Param::Variable("n1".to_string()),
                Param::Value(DataType::String("s1".to_string())),
                Param::Value(DataType::Int(2)),
                Param::Value(DataType::Float(3.14)),
            ])
        );
    }

    #[test]
    fn chunk_detector_test(){
        assert_eq!(
            chunk_detector("println".to_string(),0),
            Ok(Chunk::Function("println".to_string()))
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
    fn comma_spliter_test(){
        assert_eq!(
            comma_spliter("ali,hasan,majid"),
            vec!["ali","hasan","majid"]
        );
        assert_eq!(
            comma_spliter("\"ali,hasan\",majid"),
            vec!["\"ali,hasan\"","majid"]
        );
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
}