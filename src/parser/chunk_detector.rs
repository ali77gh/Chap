
use crate::common::{chunk::Chunk, param::Param, data_type::DataType};


pub fn chunk_detector(chunk_str: String) -> Chunk{
    
    let chunk_str = chunk_str.trim();

    match chunk_str.chars().nth(0).unwrap_or(' ') {
        '$' | '@' | '"' => Chunk::Params(param_parser(&chunk_str)),
        c => {
            if c.is_digit(10){
                Chunk::Params(param_parser(&chunk_str))
            }else{
                Chunk::Function { name: chunk_str.to_string() }
            }
        }
    }
}

fn param_parser(chunk_str: &str) -> Vec<Param>{
    let params_str = chunk_str.split(",").map(|x|{x.trim()});

    params_str.map(|param_str|{
        match chunk_str.chars().nth(0).unwrap() {
            '$' => Param::Variable((&chunk_str[1..]).to_string()),
            '@' => Param::Tag((&chunk_str[1..]).to_string()),
            '"' => {
                let len = chunk_str.len();
                Param::Value( DataType::String((&chunk_str[1..len-1]).to_string()))
            },
            _=> {
                if chunk_str.contains("."){
                    Param::Value(DataType::Float(chunk_str.parse().expect("err: syntax error while parsing float"))) //TODO better error system
                } else {
                    Param::Value(DataType::Int(chunk_str.parse().expect("err: syntax error while parsing integer"))) //TODO better error system
                }
            }
        }
    }).collect()
}