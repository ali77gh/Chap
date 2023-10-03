use crate::runtime::builtin_function::utils::returns;
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::Result;
use rand::Rng;

pub fn random_bool(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

	let mut rng = rand::thread_rng();
    returns(runtime, executable, DataType::Bool(rng.gen_bool(0.5)))
}
