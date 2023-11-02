
use super::param::Param;

#[derive(Debug, PartialEq, Clone)]
pub enum Chunk{
    Params(Vec<Param>),
    Function(String),
}