use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};

pub fn exit(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    Err(ChapError::stop())
}