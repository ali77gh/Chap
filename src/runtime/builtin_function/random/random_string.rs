use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use rand::Rng;
use rand::rngs::ThreadRng;

pub fn random_string(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

	let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
	let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

	let mut rng = rand::thread_rng();

	let result = match (p1, p2) {
    	(DataType::String(x1), DataType::Int(x2)) => {
            DataType::String(
                (0..*x2).map(|_|{ get_random_char(&mut rng,x1)}).collect()
            )
        },
    	_=>{
        	return Err(
            	ChapError::runtime_with_msg(executable.line_number, "random_string function needs string, int as param (first one is alphabet and second one is length of generated string) ".to_string())
        	);
    	}
	};

    returns(runtime, executable, result)
}

fn get_random_char(rng: &mut ThreadRng, alphabet: &String) -> char{
    alphabet.chars().nth(
        rng.gen_range(0..alphabet.len())
    ).unwrap()
}
