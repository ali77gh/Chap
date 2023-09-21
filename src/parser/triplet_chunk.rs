
use crate::common::executable::ExecutableLine;
use crate::common::chunk::Chunk;
use super::chunk_detector::chunk_detector as cd;
use super::get_single_var::get_single_var;
use crate::common::errors::{Result, ChapError};

fn triplet_chunk_parser(ch1: String, ch2: String, ch3: String, line_number: u32) -> Result<ExecutableLine>{

    let a: ExecutableLine = match (cd(ch1, line_number)?, cd(ch2, line_number)?, cd(ch3, line_number)?) {
        (Chunk::Params(left), Chunk::Function(fname), Chunk::Params(right)) => {
            let right_name = get_single_var(right, line_number)?;
            ExecutableLine::new(line_number, fname, left, Some(right_name))
        },
        (a,b,c) => {
            let a = get_type_string(&a);
            let b = get_type_string(&b);
            let c = get_type_string(&c);
            return Err(ChapError::syntax_with_msg(
                line_number,
                format!(
                    "this: <{}> -> <{}> -> <{}> is an invalid expression. the only allowed form for 3 chunk expression is: <params> -> <function> -> <variable>",
                    a,b,c
                )
            ));
        }
    };
    Ok(a)
}

fn get_type_string(chunk: &Chunk) -> String{
    match chunk {
        Chunk::Params(_) => "params".to_string(),
        Chunk::Function(_) => "function".to_string(),
    }
}

#[cfg(test)]
mod tests{
    use crate::common::{param::Param, data_type::DataType};

    use super::*;

    #[test]
    fn get_type_string_test(){
        assert_eq!(
            get_type_string(&Chunk::Params(vec![])),
            "params".to_string()
        );
        assert_eq!(
            get_type_string(&Chunk::Function("println".to_string())),
            "function".to_string()
        );
    }


    #[test]
    fn invalid_expression(){
        assert_eq!(
            triplet_chunk_parser("func1".to_string(),"$func2".to_string(),"func3".to_string(),1),
            Err(ChapError::syntax_with_msg(1, "this: <function> -> <params> -> <function> is an invalid expression. the only allowed form for 3 chunk expression is: <params> -> <function> -> <variable>".to_string()))
        );
        assert_eq!(
            triplet_chunk_parser("2, 3".to_string(),"func".to_string(),"5".to_string(),1),
            Err(ChapError::syntax_with_msg(1, "you can't set function result to a value".to_string()))
        );
        assert_eq!(
            triplet_chunk_parser("2, 3".to_string(),"func".to_string(),"5,3".to_string(),1),
            Err(ChapError::syntax_with_msg(1, "result of function cant be multiple params".to_string()))
        );
    }

    #[test]
    fn valid_expression(){
        assert_eq!(
            triplet_chunk_parser("$first,2".to_string(), "sum".to_string(), "$sum_of".to_string(), 1),
            Ok(ExecutableLine::new(
                1,
                "sum".to_string(),
                vec![Param::Variable("first".to_string()),Param::Value(DataType::Int(2))],
                Some("sum_of".to_string())
            ))
        );
    }
}