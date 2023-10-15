
mod chunk_detector;
mod get_single_var;

mod single_chunk;
mod double_chunk;
mod triplet_chunk;

use crate::common::{line_of_code::LineOfCode, executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use crate::common::splitter::string_safe_split;

use single_chunk::single_chunk_parser;
use double_chunk::double_chunk_parser;
use triplet_chunk::triplet_chunk_parser;

#[derive(Default)]
pub struct Parser;
    
impl Parser {

    pub fn on_new_line(&self, new_line: LineOfCode) -> Result<ExecutableLine>{

        let mut debug_mode = false;
        let line = if new_line.code.ends_with('?'){
            debug_mode = true;
            let mut temp = new_line.code.clone();
            temp.pop();
            temp
        }else{
            new_line.code
        };

        let it = string_safe_split(&line, "->".to_string());
        
        let mut el = match (it.get(0), it.get(1), it.get(2)) {
            (Some(chunk1), None, None) => 
                single_chunk_parser(chunk1.to_string(), new_line.line_number)?,
            (Some(chunk1), Some(chunk2), None) => 
                double_chunk_parser(chunk1.to_string(),chunk2.to_string(), new_line.line_number)?,
            (Some(chunk1), Some(chunk2), Some(chunk3)) => 
                triplet_chunk_parser(chunk1.to_string(), chunk2.to_string(), chunk3.to_string(), new_line.line_number)?,
            _ => 
                return Err(
                    ChapError::syntax_with_msg(new_line.line_number, "nothing to parse".to_string())
                ), // will never happen
        };
        el.debug_mode = debug_mode;
        Ok(el)
    }
}


#[cfg(test)]
mod tests{

    use crate::common::{param::Param, data_type::DataType};

    use super::*;

    #[test]
    fn string_includes_arrow(){
        let p = Parser::default();
        assert_eq!(
            p.on_new_line(LineOfCode::new(1, "\"hello -> world\"".to_string())),
            Ok(ExecutableLine::new(
                1,
                "print".to_string(),
                vec![Param::Value(DataType::String("hello -> world".to_string()))],
                None
            ))
        );
    }

    #[test]
    fn string_includes_comment(){
        let p = Parser::default();
        assert_eq!(
            p.on_new_line(LineOfCode::new(1, "\"hello // world\"".to_string())),
            Ok(ExecutableLine::new(
                1,
                "print".to_string(),
                vec![Param::Value(DataType::String("hello // world".to_string()))],
                None
            ))
        );
    }
}