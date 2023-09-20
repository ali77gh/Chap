
use super::param::Param;

#[derive(Debug,PartialEq)]
pub enum Chunk{
    Params(Vec<Param>),
    Function(String),
}