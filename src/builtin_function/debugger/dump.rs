use crate::{
    common::{errors::Result, executable::ExecutableLine},
    runtime::Runtime,
};

pub fn dump(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let mut result = String::new();
    for variable in runtime.variables.iter() {
        let name = variable.0;
        let value = variable.1;
        result.push_str(format!("{} -> ${}\n", value, name).as_str())
    }
    result.pop();
    runtime.std_out(
        format!(
            "------- Memory dump line: {} -------",
            executable.line_number
        )
        .as_str(),
    );
    runtime.std_out(&result);
    runtime.std_out("------- Memory dump ends -------");

    Ok(())
}
