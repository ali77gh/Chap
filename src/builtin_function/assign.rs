use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::errors::Result;
use crate::{
    common::{errors::ChapError, executable::ExecutableLine},
    runtime::Runtime,
};

pub fn assign(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    assign_validator(executable)?;

    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;

    returns(runtime, executable, p1.clone())
}

fn assign_validator(executable: &ExecutableLine) -> Result<()> {
    if executable.params.len() != 1 {
        return Err(ChapError::static_analyzer_with_msg(
            executable.line_number,
            "assign function needs one input param".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{data_type::DataType, param::Param},
        runtime::Runtime,
    };

    use super::*;

    #[test]
    fn assign_value_and_variable() {
        let mut runtime = Runtime::new(Box::new(|_| {}), Box::new(|| "".to_string()));

        assign(
            &mut runtime,
            &ExecutableLine::new(
                1,
                "".to_string(),
                vec![Param::Value(DataType::Int(2))],
                Some("var".to_string()),
            ),
        )
        .unwrap();
        assert_eq!(
            runtime.variables.get("var").unwrap().clone(),
            DataType::Int(2)
        );

        assign(
            &mut runtime,
            &ExecutableLine::new(
                1,
                "".to_string(),
                vec![Param::Variable("var".to_string())],
                Some("var2".to_string()),
            ),
        )
        .unwrap();

        assert_eq!(
            runtime.variables.get("var2").unwrap().clone(),
            DataType::Int(2)
        );
    }
}
