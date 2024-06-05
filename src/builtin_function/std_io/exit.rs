use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn exit(_runtime: &mut Runtime, _executable: &ExecutableLine) -> Result<()> {
    Err(ChapError::stop())
}
