use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use rand::Rng;

pub fn random_number(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

	let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
	let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

	let mut rng = rand::thread_rng();

	let result = match (p1, p2) {
    	(DataType::Int(x1), DataType::Int(x2)) => DataType::Int(rng.gen_range(*x1..*x2)),
    	(DataType::Float(x1), DataType::Float(x2)) => DataType::Float(rng.gen_range(*x1..*x2)),
    	_=>{
        	return Err(
            	ChapError::runtime_with_msg(executable.line_number, "random_number supports int,int or float,float in input".to_string())
        	);
    	}
	};
    returns(runtime, executable, result)
}
