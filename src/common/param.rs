
use super::data_type::DataType;

#[derive(PartialEq, Debug)]
pub enum Param {
    Tag(String),
    Variable(String),
    Value(DataType),
}