use crate::common::errors::Result;
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn pass(_runtime: &mut Runtime, _executable: &ExecutableLine) -> Result<()> {
    Ok(())
}
