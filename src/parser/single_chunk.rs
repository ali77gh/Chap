
use crate::common::executable::ExecutableLine;
use crate::common::chunk::Chunk;
use crate::common::param::Param;
use super::chunk_detector::chunk_detector;
use crate::common::errors::Result;

pub fn single_chunk_parser(ch1: String, line_number: u32) -> Result<ExecutableLine>{

    let a: ExecutableLine = match chunk_detector(ch1, line_number)? {
        Chunk::Params(params) => {

            match params.get(0).unwrap(){
                Param::Tag(_) => ExecutableLine::new(line_number,"new_tag".to_string(), params, None),
                _ => ExecutableLine::new(line_number, "println".to_string(), params, None),
            }
        },
        Chunk::Function(name) => 
            ExecutableLine::new(line_number,name,vec![], None),
    };
    Ok(a)
}

#[cfg(test)]
mod tests{
    use crate::common::{
        data_type::DataType
    };

    use super::*;

    #[test]
    fn tag_parser(){
        assert_eq!(
            single_chunk_parser(" @myTag ".to_string(), 1),
            Ok(ExecutableLine::new(1,"new_tag".to_string(),vec![Param::Tag("myTag".to_string())],None))
        );
    }

    #[test]
    fn print_detector_parser(){
        assert_eq!(
            single_chunk_parser(" $myVar ".to_string(), 1),
            Ok(ExecutableLine::new(1,"println".to_string(),vec![Param::Variable("myVar".to_string())],None))
        );

        assert_eq!(
            single_chunk_parser(" \"hello\" ".to_string(), 1),
            Ok(ExecutableLine::new(1,"println".to_string(),vec![Param::Value(DataType::String("hello".to_string()))],None))
        );
    }

    #[test]
    fn function_call_paramless(){
        assert_eq!(
            single_chunk_parser(" exit ".to_string(), 1),
            Ok(ExecutableLine::new(1, "exit".to_string(), vec![], None))
        );
    }


}