use crate::{runtime::runtime::Runtime, common::{executable::ExecutableLine, errors::ChapError}};
use crate::common::errors::Result;

pub fn assign(runtime: &mut Runtime, executable: ExecutableLine) -> Result<()>{

    assign_validator(&executable)?;

    let value = match executable.params.get(0).unwrap() {
        crate::common::param::Param::Tag(tag_name) => 
            return Err(ChapError::static_analyzer_with_msg(executable.line_number, format!("can't put a tag @{} to a variable", tag_name))),
        crate::common::param::Param::Variable(var_name) => {
            match runtime.variables.get(var_name) {
                Some(data) => data.clone(),
                None => return Err(ChapError::runtime_with_msg(executable.line_number, format!("variable {} is not defind", var_name))),
            }
        },
        crate::common::param::Param::Value(v) => v.clone(),
    };

    (*runtime).variables.insert(executable.output_var.clone().unwrap(), value);

    Ok(())
}



fn assign_validator(executable: &ExecutableLine) -> Result<()>{

    if executable.params.len() != 1{
        return Err(ChapError::static_analyzer_with_msg(executable.line_number, "".to_string()));
    }
    Ok(())
}


#[cfg(test)]
mod tests{
    use crate::{
        runtime::runtime::Runtime,
        common::{param::Param, data_type::DataType}
    };

    use super::*;

    #[test]
    fn assign_value_and_variable(){
        let mut runtime = Runtime::new(|_|{}, ||{ "".to_string() });

        assign(
            &mut runtime,
            ExecutableLine::new(
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
            ExecutableLine::new(
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