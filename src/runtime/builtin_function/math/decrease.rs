use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::common::param::Param;
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn decrease(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    if let Some(Param::Variable(name)) = executable.params.get(0) {
        match runtime.variables.get(name) {
            Some(x) => match x {
                DataType::Int(x) => {
                    runtime
                        .variables
                        .insert(name.to_string(), DataType::Int(x - 1));
                    Ok(())
                }
                _ => Err(ChapError::runtime_with_msg(
                    executable.line_number,
                    "decrease function needs one integer variable as input param".to_string(),
                )),
            },
            None => Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!("variable {} is not defind", name),
            )),
        }
    } else {
        Err(ChapError::runtime_with_msg(
            executable.line_number,
            "decrease function need one variable".to_string(),
        ))
    }
}
