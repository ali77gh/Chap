use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

#[cfg(not(target_family = "wasm"))]
pub fn random_number(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    use crate::common::data_type::DataType;
    use crate::builtin_function::utils::{param_to_datatype, returns};
    use rand::Rng;

    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let mut rng = rand::thread_rng();

    let result = match (p1, p2) {
        (DataType::Int(x1), DataType::Int(x2)) => DataType::Int(rng.gen_range(*x1..*x2)),
        (DataType::Float(x1), DataType::Float(x2)) => DataType::Float(rng.gen_range(*x1..*x2)),
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                "random_number supports int,int or float,float in input".to_string(),
            ));
        }
    };
    returns(runtime, executable, result)
}

#[cfg(target_family = "wasm")]
pub fn random_number(_runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    Err(ChapError::runtime_with_msg(
        executable.line_number,
        "random_number not supported in wasm".to_string(),
    ))
}
