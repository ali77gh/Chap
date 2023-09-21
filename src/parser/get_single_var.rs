
use crate::common::{param::Param, errors::ChapError};
use crate::common::errors::Result;


pub fn get_single_var(params:Vec<Param>, line_number: u32) -> Result<String>{

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
}