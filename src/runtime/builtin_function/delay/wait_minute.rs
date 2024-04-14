use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::{common::executable::ExecutableLine, runtime::Runtime};
use std::{thread, time};

pub fn wait_minute(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;

    if let DataType::Int(x) = p1 {
        let min = x * 60;
        let ten_millis = time::Duration::from_secs((min).try_into().unwrap());
        thread::sleep(ten_millis);
    } else {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            format!(
                "function {} needs int param as input",
                executable.function_name
            ),
        ));
    }

    Ok(())
}
