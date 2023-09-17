
use super::data_type::DataType;

pub enum Param {
    Tag(String),
    Variable(String),
    Value(DataType),
}