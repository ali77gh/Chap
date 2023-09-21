
use crate::common::executable::ExecutableLine;
use crate::common::chunk::Chunk;
use super::chunk_detector::chunk_detector as cd;
use crate::common::errors::{Result, ChapError};
use crate::parser::get_single_var::get_single_var;


pub fn double_chunk_parser(ch1: String, ch2: String, line_number: u32) -> Result<ExecutableLine>{

    let a: ExecutableLine = match (cd(ch1, line_number)?, cd(ch2, line_number)?) {
        (Chunk::Params(left), Chunk::Params(right)) => {
            let right_name = get_single_var(right, line_number)?;
            ExecutableLine::new(line_number, "assign".to_string(), left, Some(right_name))
        },
        (Chunk::Params(p), Chunk::Function(f)) => {
            ExecutableLine::new(line_number, f, p, None)
        },
        (Chunk::Function(f), Chunk::Params(p)) => {
            let right_name = get_single_var(p, line_number)?;
            ExecutableLine::new(line_number, f, vec![], Some(right_name))
        },
        (Chunk::Function(_), Chunk::Function(_)) => 
            return Err(ChapError::syntax_with_msg(
                line_number,
                "assign a function to another function is not possible".to_string())
            ),
    };
    Ok(a)
}


#[cfg(test)]
mod tests{

    use crate::common::{data_type::DataType, param::Param};

    use super::*;


    #[test]
    fn param_param_test(){
        assert_eq!(
            double_chunk_parser("$the_name".to_string(), "$name".to_string(), 1),
            Ok(ExecutableLine::new(1, "assign".to_string(), vec![Param::Variable("the_name".to_string())], Some("name".to_string())))
        );
        assert_eq!(
            double_chunk_parser("2".to_string(), "$name".to_string(), 1),
            Ok(ExecutableLine::new(1, "assign".to_string(), vec![Param::Value(DataType::Int(2))], Some("name".to_string())))
        );
        assert_eq!(
            double_chunk_parser("$name".to_string(), "2".to_string(), 1),
            Err(ChapError::syntax_with_msg(1, "you can't set function result to a value".to_string()))
        );
        assert_eq!(
            double_chunk_parser("2".to_string(), "2".to_string(), 1),
            Err(ChapError::syntax_with_msg(1, "you can't set function result to a value".to_string()))
        );
    }

    #[test]
    fn param_function_test(){
        assert_eq!(
            double_chunk_parser("$the_name".to_string(), "println".to_string(), 1),
            Ok(ExecutableLine::new(1, "println".to_string(), vec![Param::Variable("the_name".to_string())], None))
        );
    }

    #[test]
    fn function_param_test(){
        assert_eq!(
            double_chunk_parser("input".to_string(), "$name".to_string(), 1),
            Ok(ExecutableLine::new(1, "input".to_string(), vec![], Some("name".to_string())))
        );
    }

    #[test]
    fn function_function_test(){
        assert_eq!(
            double_chunk_parser("exit".to_string(), "println".to_string(), 1),
            Err(ChapError::syntax_with_msg(1, "assign a function to another function is not possible".to_string()))
        );

    }
}