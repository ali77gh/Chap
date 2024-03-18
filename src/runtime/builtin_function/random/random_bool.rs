use crate::common::data_type::DataType;
use crate::common::errors::Result;
use crate::runtime::builtin_function::utils::returns;
use crate::{common::executable::ExecutableLine, runtime::Runtime};
use rand::Rng;

pub fn random_bool(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let mut rng = rand::thread_rng();
    returns(runtime, executable, DataType::Bool(rng.gen_bool(0.5)))
}
