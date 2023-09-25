use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};

pub fn exit(_runtime: &mut Runtime,_executablee: &ExecutableLine)-> Result<()>{

    Err(ChapError::stop())
}