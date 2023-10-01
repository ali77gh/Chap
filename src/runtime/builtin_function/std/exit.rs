use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};

pub fn exit(_runtime: &mut Runtime,_executable: &ExecutableLine)-> Result<()>{

    Err(ChapError::stop())
}