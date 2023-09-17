
use crate::common::executable::ExecutableLine;
use crate::common::chunk::Chunk;
use super::chunk_detector::chunk_detector;

fn single_chunk_parser(ch1: String) -> ExecutableLine{

    match chunk_detector(ch1) {
        Chunk::Params(params) => ExecutableLine::FunctionCall {
            function_name: "println".to_string(),
            params: params,
            output_var: None 
        },
        Chunk::Function { name } => ExecutableLine::FunctionCall {
            function_name: name,
            params: vec![], 
            output_var: None
        },
    } 
}