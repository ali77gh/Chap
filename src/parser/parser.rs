use crate::common::{line_of_code::LineOfCode, executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};

use super::single_chunk::single_chunk_parser;
use super::double_chunk::double_chunk_parser;
use super::triplet_chunk::triplet_chunk_parser;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn on_new_line(&self, new_line: LineOfCode) -> Result<ExecutableLine>{
        let mut it = new_line.code.split("->");
        
        match (it.next(), it.next(), it.next()) {
            (Some(chunk1), None, None) => 
                single_chunk_parser(chunk1.to_string(), new_line.line_number),
            (Some(chunk1), Some(chunk2), None) => 
                double_chunk_parser(chunk1.to_string(),chunk2.to_string(), new_line.line_number),
            (Some(chunk1), Some(chunk2), Some(chunk3)) => 
                triplet_chunk_parser(chunk1.to_string(), chunk2.to_string(), chunk3.to_string(), new_line.line_number),
            _ => 
                Err(ChapError::syntax_with_msg(new_line.line_number, "nothing to parse".to_string())), // will never happen
        }
    }
}
