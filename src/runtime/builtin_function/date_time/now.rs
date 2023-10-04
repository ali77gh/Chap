use crate::common::data_type::DataType;
use crate::runtime::builtin_function::utils::returns;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::Result;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_sec(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64();
    let result = DataType::Float(now);

    returns(runtime, executable, result)
}
