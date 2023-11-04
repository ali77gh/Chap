
pub mod dump;

use crate::{
    runtime::{
        Runtime,
        builtin_function::{
            utils::{
                param_to_datatype,
                get_var
            },
            closure_gen
        }
    },
    common::{
        executable::{
            ExecutableLine,
            BuiltinFunction
        },
        param::Param,
        errors::Result
    }
};


pub fn debugger(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    // inputs
    let mut params_values = Vec::new();
    for param in &executable.params{
        let param_str = if let Param::Tag(tag_name) = param{
            ("@".to_string() + tag_name).to_string()
        }else{
            param_to_datatype(runtime, Some(param), executable.line_number)?.to_string()
        };
        params_values.push(param_str);
    }
    let params_str = if params_values.is_empty(){
        "Nothing".to_string()
    }else{
        params_values.join(", ")
    };


    // run actual function
    let mut executable = executable.clone();
    executable.function_name = executable.function_name.replace("?", "");
    let real_function: BuiltinFunction = closure_gen(&executable)?;
    real_function(runtime, &executable)?;
    
    // output
    let return_str = if let Some(var_name) = &executable.output_var{
        get_var(runtime, var_name, executable.line_number)?.to_string()
    }else{
        "Nothing".to_string()
    };
     

    // print
    runtime.std_out(format!(
        "debugger on line {}: {} -> {} -> {}",
        executable.line_number, params_str, executable.function_name, return_str
    ).as_str());
    Ok(())
}