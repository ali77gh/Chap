use crate::common::errors::Result;
use crate::{common::executable::ExecutableLine, runtime::Runtime};

#[cfg(not(target_family = "wasm"))]
pub fn random_bool(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    use crate::builtin_function::utils::returns;
    use crate::common::data_type::DataType;
    use rand::Rng;

    let mut rng = rand::thread_rng();
    returns(runtime, executable, DataType::Bool(rng.gen_bool(0.5)))
}

#[cfg(target_family = "wasm")]
pub fn random_bool(_runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    use crate::common::errors::ChapError;
    Err(ChapError::runtime_with_msg(
        executable.line_number,
        "random_bool not supported in wasm".to_string(),
    ))
}
