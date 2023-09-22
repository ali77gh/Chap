use crate::common::data_type::DataType;
use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::Result;


pub fn input(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let inp = runtime.std_in();
    runtime.variables.insert(executable.output_var.clone().unwrap(), DataType::String(inp));
    // TODO validation
    // TODO tests
    Ok(())
}