
mod chunk_detector;
mod get_single_var;

mod single_chunk;
mod double_chunk;
mod triplet_chunk;

use crate::common::{line_of_code::LineOfCode, executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};

use single_chunk::single_chunk_parser;
use double_chunk::double_chunk_parser;
use triplet_chunk::triplet_chunk_parser;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

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

        let mut it = line.split("->");
        
        let mut el = match (it.next(), it.next(), it.next()) {
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