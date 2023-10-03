use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use rand::Rng;

pub fn random_choice(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    let mut choices = Vec::new();
    for param in &executable.params{
        let value = param_to_datatype(runtime, Some(param), executable.line_number)?;
        choices.push(value);
    }

    if choices.len() < 2{
        return Err(
           	ChapError::runtime_with_msg(executable.line_number, "random_choice needs many input params".to_string())
        );
    }

	let mut rng = rand::thread_rng();
    let result = choices.get(
        rng.gen_range(0..choices.len())
    ).unwrap();

    returns(runtime, executable, (*result).clone())
}
