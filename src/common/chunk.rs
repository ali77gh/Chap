
use super::param::Param;

pub enum Chunk{
    Params(Vec<Param>),
    Function{ name: String },
}