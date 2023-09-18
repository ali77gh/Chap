
use super::param::Param;

pub enum ExecutableLine {
    Tag,
    FunctionCall{ function_name: String, params: Vec<Param>, output_var: Option<String>}
}