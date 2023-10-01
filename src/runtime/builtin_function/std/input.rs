use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn input(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let inp = runtime.std_in();

    let o_var = match &executable.output_var {
        Some(v) => v.clone(),
        None => return Err(ChapError::runtime_with_msg(executable.line_number, "input function needs output variable".to_string())),
    };

    runtime.variables.insert(o_var, DataType::String(inp));
    Ok(())
}


#[cfg(test)]
mod tests{
    use crate::{runtime::Runtime, common::{executable::ExecutableLine, data_type::DataType}};

    use super::input;


    #[test]
    fn input_test(){

        let mut runtime = Runtime::new(|_|{}, ||{ "test".to_string() });

        input(
            &mut runtime,
            &ExecutableLine::new(
                1,
                "".to_string(),
                vec![],
                Some("test_var".to_string())
            )
        ).unwrap();

        assert_eq!(
            runtime.variables.get("test_var").unwrap(),
            &DataType::String("test".to_string())
        )
    }
}