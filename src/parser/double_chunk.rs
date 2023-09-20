
use crate::common::{executable::ExecutableLine, param::Param};
use crate::common::chunk::Chunk;
use super::chunk_detector::chunk_detector as cd;
use crate::common::errors::{Result, ChapError};


fn double_chunk_parser(ch1: String, ch2: String, line_number: u32) -> Result<ExecutableLine>{

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

fn get_single_var(params:Vec<Param>, line_number: u32) -> Result<String>{

    if params.len() != 1{
        return Err(ChapError::syntax_with_msg(line_number, "result of function cant be multiple params".to_string()));
    }

    match params.get(0).unwrap(){
        Param::Tag(_) => Err(ChapError::syntax_with_msg(line_number, "you can't set function result to a tag".to_string())),
        Param::Value(_) => Err(ChapError::syntax_with_msg(line_number, "you can't set function result to a value".to_string())),
        Param::Variable(name) => Ok(name.clone()),
    }
}


#[cfg(test)]
mod tests{

    use crate::common::data_type::DataType;

    use super::*;

    #[test]
    fn get_single_var_test(){
        assert_eq!(
            get_single_var(vec![Param::Tag("".to_string()),Param::Tag("".to_string())], 1),
            Err(ChapError::syntax_with_msg(1, "result of function cant be multiple params".to_string()))
        );
        assert_eq!(
            get_single_var(vec![], 1),
            Err(ChapError::syntax_with_msg(1, "result of function cant be multiple params".to_string()))
        );
        assert_eq!(
            get_single_var(vec![Param::Value(DataType::String("hich".to_string()))], 1),
            Err(ChapError::syntax_with_msg(1, "you can't set function result to a value".to_string()))
        );
        assert_eq!(
            get_single_var(vec![Param::Tag("mytag".to_string())], 1),
            Err(ChapError::syntax_with_msg(1, "you can't set function result to a tag".to_string()))
        );
    }

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