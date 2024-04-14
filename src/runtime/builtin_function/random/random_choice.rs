use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

#[cfg(not(target_family = "wasm"))]
pub fn random_choice(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
    use rand::Rng;

    let mut choices = Vec::new();
    for param in &executable.params {
        let value = param_to_datatype(runtime, Some(param), executable.line_number)?;
        choices.push(value);
    }

    if choices.len() < 2 {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            "random_choice needs many input params".to_string(),
        ));
    }

    let mut rng = rand::thread_rng();
    let result = choices.get(rng.gen_range(0..choices.len())).unwrap();

    returns(runtime, executable, (*result).clone())
}

#[cfg(target_family = "wasm")]
pub fn random_choice(_runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    Err(ChapError::runtime_with_msg(
        executable.line_number,
        "random_choice not supported in wasm".to_string(),
    ))
}
