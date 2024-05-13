use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

#[cfg(not(target_family = "wasm"))]
pub fn random_string(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    use crate::common::data_type::DataType;
    use crate::builtin_function::utils::{param_to_datatype, returns};

    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let mut rng = rand::thread_rng();

    let result = match (p1, p2) {
        (DataType::String(x1), DataType::Int(x2)) => {
            DataType::String((0..*x2).map(|_| get_random_char(&mut rng, x1)).collect())
        }
        _ => {
            return Err(
            	ChapError::runtime_with_msg(executable.line_number, "random_string function needs string, int as param (first one is alphabet and second one is length of generated string) ".to_string())
        	);
        }
    };

    returns(runtime, executable, result)
}

#[cfg(not(target_family = "wasm"))]
use rand::rngs::ThreadRng;

#[cfg(not(target_family = "wasm"))]
fn get_random_char(rng: &mut ThreadRng, alphabet: &str) -> char {
    use rand::Rng;
    alphabet
        .chars()
        .nth(rng.gen_range(0..alphabet.len()))
        .unwrap()
}

#[cfg(target_family = "wasm")]
pub fn random_string(_runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    Err(ChapError::runtime_with_msg(
        executable.line_number,
        "random_string not supported in wasm".to_string(),
    ))
}
