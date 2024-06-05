use crate::common::errors::Result;
use crate::{
    common::{errors::ChapError, executable::ExecutableLine, param::Param},
    runtime::Runtime,
};

// this function can't jump to a tag that is not added to runtime.executables
pub fn jump(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    if let Some(Param::Tag(tag_name, line_number)) = executable.params.first() {
        if let Some(ln) = line_number {
            runtime.current_line = *ln;
            return Ok(());
        }
        if let Some(line_number) = runtime.tags.get(tag_name) {
            // bind line number value to tag param
            let line = runtime
                .executables
                .get_mut(runtime.current_line - 1)
                .unwrap();
            if let Param::Tag(_, ln) = line.params.first_mut().unwrap() {
                *ln = Some(*line_number);
            }
            runtime.current_line = *line_number;
        } else {
            // jumping forward (conditional execution) // not possible in repel mode
            loop {
                match runtime.executables.get(runtime.current_line) {
                    Some(el) => {
                        if el.function_name == "new_tag" {
                            if let Some(Param::Tag(el_tag, _)) = el.params.first() {
                                if tag_name == el_tag {
                                    runtime.tags.insert(tag_name.clone(), runtime.current_line);
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        return Err(ChapError::runtime_with_msg(
                            executable.line_number,
                            format!("cant find tag: {}", tag_name),
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
