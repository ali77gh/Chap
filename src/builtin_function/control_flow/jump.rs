use crate::common::errors::Result;
use crate::{
    common::{errors::ChapError, executable::ExecutableLine, param::Param},
    runtime::Runtime,
};

// this function can't jump to a tag that is not added to runtime.executables
pub fn jump(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    if let Some(Param::Tag(tag)) = executable.params.first() {
        if let Some(line_number) = runtime.tags.get(tag) {
            // jumping back (loop)
            runtime.current_line = *line_number;
        } else {
            // jumping forward (conditional execution) // not possible in repel mode
            loop {
                match runtime.executables.get(runtime.current_line) {
                    Some(el) => {
                        if el.function_name == "new_tag" {
                            if let Some(Param::Tag(el_tag)) = el.params.first() {
                                if tag == el_tag {
                                    runtime.tags.insert(tag.clone(), runtime.current_line);
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        return Err(ChapError::runtime_with_msg(
                            executable.line_number,
                            format!("cant find tag: {}", tag),
                        ))
                    }
                };
                runtime.current_line += 1;
            }
        }
    } else {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            "function jump needs a tag for first params".to_string(),
        ));
    }
    Ok(())
}
