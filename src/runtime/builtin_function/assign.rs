use crate::{runtime::Runtime, common::{executable::ExecutableLine, errors::ChapError}};
use crate::common::errors::Result;
use crate::runtime::builtin_function::utils::param_to_datatype;

pub fn assign(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    assign_validator(executable)?;

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;

    if let Some(var_name) = &executable.output_var{
        runtime.variables.insert(var_name.clone(), p1.clone()); //Datatype impelements PartialEq
        Ok(())
    }else{
        Err(
            ChapError::runtime_with_msg(executable.line_number, "assign function needs output variable".to_string())
        )
    }
}

fn assign_validator(executable: &ExecutableLine) -> Result<()>{

    if executable.params.len() != 1{
        return Err(ChapError::static_analyzer_with_msg(executable.line_number, "assign function needs one input param".to_string()));
    }
    Ok(())
}


#[cfg(test)]
mod tests{
    use crate::{
        runtime::Runtime,
        common::{param::Param, data_type::DataType}
    };

    use super::*;

    #[test]
    fn assign_value_and_variable(){
        let mut runtime = Runtime::new(|_|{}, ||{ "".to_string() });

        assign(
            &mut runtime,
            &ExecutableLine::new(
                1,
                "".to_string(),
                vec![Param::Value(DataType::Int(2))],
                Some("var".to_string())
            )
        ).unwrap();
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
                Some("var2".to_string())
            )
        ).unwrap();

        assert_eq!(
            runtime.variables.get("var2").unwrap().clone(),
            DataType::Int(2)
        );
    }
}