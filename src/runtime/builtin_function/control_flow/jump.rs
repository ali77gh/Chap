use crate::{runtime::runtime::Runtime, common::{executable::ExecutableLine, errors::ChapError, param::Param}};
use crate::common::errors::Result;

// this function can't jump to a tag that is not added to runtime.executables
pub fn jump(runtime: &mut Runtime, executable: ExecutableLine)-> Result<()>{

    if let Some(Param::Tag(tag)) = executable.params.get(0){

        if let Some(line_number) = runtime.tags.get(tag){
            // jumping back (loop)
            runtime.current_line = *line_number;
        }else{
            // jumping forward (conditional execution) // not possible in repel mode
            loop{
                match runtime.executables.get(runtime.current_line){
                    Some(el) => {
                        if el.function_name == "new_tag"{
                            if let Some(Param::Tag(eltag)) = el.params.get(0){
                                if tag == eltag{
                                    runtime.tags.insert(tag.clone(), runtime.current_line);
                                    break;
                                }
                            }
                        }
                    },
                    None => return Err(
                        ChapError::runtime_with_msg(executable.line_number, format!("cant find tag: {}", tag))
                    ),
                };
                runtime.current_line += 1;
            }
        }

    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, "error while creating a tag".to_string()));
    }
    Ok(())
}


// TODO tests