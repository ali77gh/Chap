use crate::{
    runtime::{
        Runtime
    },
    common::{
        executable::{
            ExecutableLine
        },
        errors::Result
    }
};

pub fn dump(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    runtime.std_out(format!("------- Memory dump line: {}",executable.line_number).as_str());
    for variable in runtime.variables.iter(){
        let name = variable.0;
        let value = variable.1;
        runtime.std_out(format!("{} -> ${}", value.to_string(), name).as_str())
    }
    runtime.std_out("------- Memory dump ends");
    
    Ok(())
}